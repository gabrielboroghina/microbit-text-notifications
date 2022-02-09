//! Network Driver
//! Commands
//!     0 -> SUCCESS
//!     1 -> Network buffer
//!     2 -> characters Networked
//!
//! Allow
//!     0 -> buffer to display
//!

// GET/POST address data_out (base64)\n
use core::cell::Cell;

use kernel::errorcode::into_statuscode;
use kernel::grant::Grant;
use kernel::hil::uart::{ReceiveClient, TransmitClient, UartData};
use kernel::process::{Error, ProcessId};
use kernel::processbuffer::{
    ReadOnlyProcessBuffer, ReadWriteProcessBuffer, ReadableProcessBuffer, WriteableProcessBuffer,
};
use kernel::syscall::{CommandReturn, SyscallDriver};
use kernel::utilities::cells::TakeCell;
use kernel::{debug, ErrorCode};

pub const DRIVER_NUM: usize = 0xa0001;

#[derive(Copy, Clone)]
enum NetworkState {
    Idle,
    Requesting(ProcessId),
}

#[derive(Default)]
pub struct AppStorage {
    address: ReadOnlyProcessBuffer,
    data_out: ReadOnlyProcessBuffer,
    data_in: ReadWriteProcessBuffer,
}

pub struct Network<'a> {
    grant_access: Grant<AppStorage, 1>,
    uart: &'a dyn UartData<'a>,
    state: Cell<NetworkState>,
    buffer: TakeCell<'static, [u8]>,
    response_curr_idx: Cell<usize>,
}

impl<'a> Network<'a> {
    pub fn new(
        grant_access: Grant<AppStorage, 1>,
        uart: &'a dyn UartData<'a>,
        buffer: &'static mut [u8],
    ) -> Network<'a> {
        Network {
            grant_access,
            uart: uart,
            state: Cell::new(NetworkState::Idle),
            buffer: TakeCell::new(buffer),
            response_curr_idx: Cell::new(0),
        }
    }
}

impl<'a> SyscallDriver for Network<'a> {
    fn command(
        &self,
        command_num: usize,
        _r2: usize,
        _r3: usize,
        process_id: ProcessId,
    ) -> CommandReturn {
        match command_num {
            0 => CommandReturn::success(),
            // send request
            1 => {
                if let NetworkState::Idle = self.state.get() {
                    self.response_curr_idx.set(0);
                    let res = self.grant_access.enter(process_id, |app_storage, _upcalls_table| {
                        // Result<Result<(), ErrorCode>, Error>
                        let res = app_storage.address.enter(|address| {
                            let buffer = self.buffer.take();
                            if let Some(buffer) = buffer {
                                // buf[index].get() -> u8
                                if 5 + address.len() <= buffer.len() {

                                    address.copy_to_slice(&mut buffer[5..5 + address.len()]);

                                    if app_storage.data_out.len() > 0 {
                                        // POST
                                        app_storage.data_out.enter(move |data_out| {

                                            let len1 = 5 + address.len();
                                            let len2 = len1 + 57 + data_out.len();

                                            if len2 + 2 <= buffer.len() {

                                                &buffer[0..5].copy_from_slice("POST ".as_bytes());
                                                &buffer[len1..len1 + 57].copy_from_slice("\r\nContent-Type: application/json\r\nContent-Length: 000\r\n\r\n".as_bytes());
                                                buffer[len1 + 50] = (data_out.len() / 100) as u8 + '0' as u8;
                                                buffer[len1 + 51] = (data_out.len() / 10 % 10) as u8 + '0' as u8;
                                                buffer[len1 + 52] = (data_out.len() % 10) as u8 + '0' as u8;
                                                data_out.copy_to_slice(&mut buffer[len1 + 57..len2]);
                                                &buffer[len2..len2 + 2].copy_from_slice("\r\n".as_bytes());

                                                if let Err((error, buffer)) = self.uart.transmit_buffer(buffer, len2 + 2) {
                                                    self.buffer.replace(buffer);
                                                    Err(error)
                                                }
                                                else {
                                                    self.state.set(NetworkState::Requesting(process_id));
                                                    Ok(())
                                                }
                                            } else {
                                                Err(ErrorCode::INVAL)
                                            }
                                        })
                                        .map_err(|err| err.into())
                                        .and_then(|x| x)
                                    } else {
                                        // GET
                                        &buffer[0..5].copy_from_slice("GET  ".as_bytes());
                                        &buffer[5 + address.len()..9 + address.len()].copy_from_slice("\r\n\r\n".as_bytes());

                                        if let Err((error, buffer)) = self.uart.transmit_buffer(buffer, 5 + address.len() + 5) {
                                            self.buffer.replace(buffer);
                                            Err(error)
                                        }
                                        else {
                                            self.state.set(NetworkState::Requesting(process_id));
                                            Ok(())
                                        }
                                    }
                                } else {
                                    Err(ErrorCode::SIZE)
                                }
                            } else {
                                Err(ErrorCode::NOMEM)
                            }
                        });
                        match res {
                            Ok(Ok(())) => Ok(()),
                            Ok(Err(err)) => Err(err),
                            Err(err) => Err(err.into()),
                        }
                    });
                    match res {
                        Ok(Ok(())) => CommandReturn::success(),
                        Ok(Err(err)) => CommandReturn::failure(err),
                        Err(err) => CommandReturn::failure(err.into()),
                    }
                } else {
                    CommandReturn::failure(ErrorCode::BUSY)
                }
            }
            _ => CommandReturn::failure(ErrorCode::NOSUPPORT),
        }
    }

    fn allow_readonly(
        &self,
        process_id: ProcessId,
        allow_num: usize,
        mut buffer: ReadOnlyProcessBuffer,
    ) -> Result<ReadOnlyProcessBuffer, (ReadOnlyProcessBuffer, ErrorCode)> {
        match allow_num {
            // address
            0 => {
                let res = self.grant_access.enter(process_id, |app_storage, _upcalls_table| {
                    core::mem::swap(&mut app_storage.address, &mut buffer);
                });
                match res {
                    Ok(()) => Ok(buffer),
                    Err(err) => Err((buffer, err.into())),
                }
            }
            // data_out
            1 => {
                let res = self.grant_access.enter(process_id, |app_storage, _upcalls_table| {
                    core::mem::swap(&mut app_storage.data_out, &mut buffer);
                });
                match res {
                    Ok(()) => Ok(buffer),
                    Err(err) => Err((buffer, err.into())),
                }
            }
            _ => Err((buffer, ErrorCode::NOSUPPORT)),
        }
    }

    fn allow_readwrite(
        &self,
        process_id: ProcessId,
        allow_num: usize,
        mut buffer: ReadWriteProcessBuffer,
    ) -> Result<ReadWriteProcessBuffer, (ReadWriteProcessBuffer, ErrorCode)> {
        match allow_num {
            // data_in
            0 => {
                let res = self.grant_access.enter(process_id, |app_storage, _upcalls_table| {
                    core::mem::swap(&mut app_storage.data_in, &mut buffer);
                });
                match res {
                    Ok(()) => Ok(buffer),
                    Err(err) => Err((buffer, err.into())),
                }
            }
            _ => Err((buffer, ErrorCode::NOSUPPORT)),
        }
    }

    fn allocate_grant(&self, process_id: ProcessId) -> Result<(), Error> {
        self.grant_access.enter(process_id, |_app_storage, _upcalls_table| {})
    }
}

impl<'a> TransmitClient for Network<'a> {
    fn transmitted_buffer(
        &self,
        tx_buffer: &'static mut [u8],
        _tx_len: usize,
        rval: Result<(), ErrorCode>,
    ) {
        match rval {
            Ok(()) => {
                // Start reading the response from UART
                if let Err((error, buffer)) = self.uart.receive_buffer(tx_buffer, 1) {
                    self.buffer.replace(buffer);
                    if let NetworkState::Requesting(process_id) = self.state.get() {
                        let _ = self.grant_access.enter(process_id, |_, upcalls_table| {
                            let _ = upcalls_table
                                .schedule_upcall(0, (into_statuscode(Err(error)), 0, 0));
                        });
                    }
                    self.state.set(NetworkState::Idle);
                }
            }
            Err(error) => {
                self.buffer.replace(tx_buffer);
                if let NetworkState::Requesting(process_id) = self.state.get() {
                    let _ = self.grant_access.enter(process_id, |_, upcalls_table| {
                        let _ =
                            upcalls_table.schedule_upcall(0, (into_statuscode(Err(error)), 0, 0));
                    });
                }
                self.state.set(NetworkState::Idle);
            }
        }
    }
}

impl<'a> ReceiveClient for Network<'a> {
    fn received_buffer(
        &self,
        rx_buffer: &'static mut [u8],
        rx_len: usize,
        rval: Result<(), ErrorCode>,
        _error: kernel::hil::uart::Error,
    ) {
        match rval {
            Ok(()) => {
                if let NetworkState::Requesting(process_id) = self.state.get() {
                    let _ = self.grant_access.enter(process_id, |app_storage, _upcalls_table| {
                        let _res = app_storage.data_in.mut_enter(|data_in| {
                            if rx_buffer.len() <= data_in.len() {
                                data_in[self.response_curr_idx.get()].set(rx_buffer[0]);
                                self.response_curr_idx.set(self.response_curr_idx.get() + 1);
                            }
                            if rx_buffer[0] == 0 && self.response_curr_idx.get() < data_in.len() {
                                data_in[self.response_curr_idx.get()].set(0);
                            }
                        });
                    });

                    if rx_buffer[0] != 0 {
                        // Wait for the next byte
                        if let Err((error, buffer)) = self.uart.receive_buffer(rx_buffer, 1) {
                            self.buffer.replace(buffer);
                            if let NetworkState::Requesting(process_id) = self.state.get() {
                                let _ = self.grant_access.enter(process_id, |_, upcalls_table| {
                                    let _ = upcalls_table.schedule_upcall(0, (into_statuscode(Err(error)), 0, 0));
                                });
                            }
                            self.state.set(NetworkState::Idle);
                        }
                    } else {
                        let _ = self.grant_access.enter(process_id, |app_storage, upcalls_table| {
                            let mut has_body = false;
                            let mut body_offset: usize = 0;

                            let _res = app_storage.data_in.mut_enter(|data_in| {
                                for i in 0..data_in.len() - 3 {
                                    if data_in[i].get() == 0 {
                                        break;
                                    }
                                    if data_in[i].get() == '\r' as u8 && data_in[i + 1].get() == '\n' as u8 && data_in[i + 2].get() == '\r' as u8 && data_in[i + 3].get() == '\n' as u8 {
                                        // "\r\n\r\n" delimits the headers section from the body of the response
                                        has_body = true;
                                        body_offset = i + 4;
                                        break;
                                    }
                                }
                                // Keep only the body in the response buffer; Shift body content to the beginning of the buffer
                                if has_body {
                                    for i in body_offset..data_in.len() {
                                        data_in[i - body_offset].set(data_in[i].get());
                                    }
                                }
                            });
                            if !has_body {
                                let _ = upcalls_table.schedule_upcall(0, (418, 0, 0));
                            } else {
                                let _ = upcalls_table.schedule_upcall(0, (0, rx_len, 0));
                            }
                        });

                        // Finished reading the response; reset the state
                        self.buffer.replace(rx_buffer);
                        self.state.set(NetworkState::Idle);
                    }
                }
            }
            Err(error) => {
                if let NetworkState::Requesting(process_id) = self.state.get() {
                    let _ = self.grant_access.enter(process_id, |_, upcalls_table| {
                        let _ = upcalls_table.schedule_upcall(0, (into_statuscode(Err(error)), 0, 0));
                    });
                }
            }
        }
    }
}
