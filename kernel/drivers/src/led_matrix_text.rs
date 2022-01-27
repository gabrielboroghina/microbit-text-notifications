//! LED matrix text display driver
//! Commands
//!     0 -> SUCCESS
//!     1 -> display a string
//!

use capsules::led_matrix::LedMatrixDriver;

use core::cell::Cell;

use kernel::grant::Grant;
use kernel::hil::gpio::{Pin};
use kernel::hil::time::{Alarm, AlarmClient, ConvertTicks};
use kernel::process::{Error, ProcessId};
use kernel::syscall::{CommandReturn, SyscallDriver};
use kernel::{debug, ErrorCode};
use kernel::processbuffer::{ReadOnlyProcessBuffer, ReadableProcessBuffer};

pub const DRIVER_NUM: usize = 0xa0000;

// LED matrices for characters
const LTR_A: [[u8; 5]; 5] = [
    [0, 1, 1, 1, 0],
    [1, 0, 0, 0, 1],
    [1, 1, 1, 1, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
];
const LTR_B: [[u8; 5]; 5] = [
    [1, 1, 1, 1, 0],
    [1, 0, 0, 0, 1],
    [1, 1, 1, 1, 0],
    [1, 0, 0, 0, 1],
    [1, 1, 1, 1, 0],
];
const LTR_C: [[u8; 5]; 5] = [
    [0, 1, 1, 1, 1],
    [1, 0, 0, 0, 0],
    [1, 0, 0, 0, 0],
    [1, 0, 0, 0, 0],
    [0, 1, 1, 1, 1],
];
const LTR_D: [[u8; 5]; 5] = [
    [1, 1, 1, 1, 0],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 1, 1, 1, 0],
];
const LTR_E: [[u8; 5]; 5] = [
    [1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0],
    [1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0],
    [1, 1, 1, 1, 1],
];
const LTR_F: [[u8; 5]; 5] = [
    [1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0],
    [1, 1, 1, 0, 0],
    [1, 0, 0, 0, 0],
    [1, 0, 0, 0, 0],
];
const LTR_G: [[u8; 5]; 5] = [
    [0, 1, 1, 1, 1],
    [1, 0, 0, 0, 0],
    [1, 0, 1, 1, 1],
    [1, 0, 0, 0, 1],
    [0, 1, 1, 1, 1],
];
const LTR_H: [[u8; 5]; 5] = [
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 1, 1, 1, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
];
const LTR_I: [[u8; 5]; 5] = [
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
];
const LTR_J: [[u8; 5]; 5] = [
    [0, 0, 1, 1, 1],
    [0, 0, 0, 0, 1],
    [0, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [0, 1, 1, 1, 0],
];
const LTR_K: [[u8; 5]; 5] = [
    [1, 0, 0, 1, 0],
    [1, 0, 1, 0, 0],
    [1, 1, 1, 1, 0],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
];
const LTR_L: [[u8; 5]; 5] = [
    [1, 0, 0, 0, 0],
    [1, 0, 0, 0, 0],
    [1, 0, 0, 0, 0],
    [1, 0, 0, 0, 0],
    [1, 1, 1, 1, 1],
];
const LTR_M: [[u8; 5]; 5] = [
    [1, 0, 0, 0, 1],
    [1, 1, 0, 1, 1],
    [1, 0, 1, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
];
const LTR_N: [[u8; 5]; 5] = [
    [1, 1, 0, 0, 1],
    [1, 0, 1, 0, 1],
    [1, 0, 1, 0, 1],
    [1, 0, 1, 0, 1],
    [1, 0, 0, 1, 1],
];
const LTR_O: [[u8; 5]; 5] = [
    [0, 1, 1, 1, 0],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [0, 1, 1, 1, 0],
];
const LTR_P: [[u8; 5]; 5] = [
    [1, 1, 1, 1, 0],
    [1, 0, 0, 0, 1],
    [1, 1, 1, 1, 0],
    [1, 0, 0, 0, 0],
    [1, 0, 0, 0, 0],
];
const LTR_Q: [[u8; 5]; 5] = [
    [0, 1, 1, 0, 0],
    [1, 0, 0, 1, 0],
    [1, 0, 0, 1, 0],
    [1, 0, 0, 1, 0],
    [0, 1, 1, 1, 1],
];
const LTR_R: [[u8; 5]; 5] = [
    [1, 1, 1, 1, 0],
    [1, 0, 0, 0, 1],
    [1, 1, 1, 1, 0],
    [1, 0, 0, 1, 0],
    [1, 0, 0, 0, 1],
];
const LTR_S: [[u8; 5]; 5] = [
    [0, 1, 1, 1, 1],
    [1, 0, 0, 0, 0],
    [0, 1, 1, 1, 0],
    [0, 0, 0, 0, 1],
    [1, 1, 1, 1, 0],
];
const LTR_T: [[u8; 5]; 5] = [
    [1, 1, 1, 1, 1],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
];
const LTR_U: [[u8; 5]; 5] = [
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [0, 1, 1, 1, 0],
];
const LTR_V: [[u8; 5]; 5] = [
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [0, 1, 0, 1, 0],
    [0, 1, 0, 1, 0],
    [0, 0, 1, 0, 0],
];
const LTR_W: [[u8; 5]; 5] = [
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 1, 0, 1],
    [1, 1, 0, 1, 1],
    [1, 0, 0, 0, 1],
];
const LTR_X: [[u8; 5]; 5] = [
    [1, 0, 0, 0, 1],
    [0, 1, 0, 1, 0],
    [0, 0, 1, 0, 0],
    [0, 1, 0, 1, 0],
    [1, 0, 0, 0, 1],
];
const LTR_Y: [[u8; 5]; 5] = [
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [0, 1, 1, 1, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
];
const LTR_Z: [[u8; 5]; 5] = [
    [1, 1, 1, 1, 1],
    [0, 0, 0, 0, 1],
    [0, 1, 1, 1, 0],
    [1, 0, 0, 0, 0],
    [1, 1, 1, 1, 1],
];

const LTR_: [[u8; 5]; 5] = [
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
];

pub struct LedMatrixText<'a, A: Alarm<'a>, L: Pin> {
    access_grant: Grant<AppStorage, 1>,
    alarm: &'a A,
    led_matrix: &'a LedMatrixDriver<'a, L, A>,
    status: Cell<Status>,
}

#[derive(Copy, Clone)]
enum Status {
    Idle,
    Processing(usize, usize, ProcessId, u32) // idx, len, process_id
}

#[derive(Default)]
pub struct AppStorage {
    text: ReadOnlyProcessBuffer,
}

impl<'a, A: Alarm<'a>, L: Pin> LedMatrixText<'a, A, L> {
    pub fn new(alarm: &'a A, access_grant: Grant<AppStorage, 1>, led_matrix: &'a LedMatrixDriver<'a, L, A>) -> LedMatrixText<'a, A, L> {
        // Initialize all LEDs and turn them off
        led_matrix.init();

        LedMatrixText {
            alarm,
            access_grant,
            led_matrix,
            status: Cell::new(Status::Idle),
        }
    }

    fn display_char(&self, ch: u8) {
        for c in 0..self.led_matrix.cols_len() {
            for r in 0..self.led_matrix.rows_len() {
                let ch_leds = match ch as char {
                    'A' => LTR_A,
                    'B' => LTR_B,
                    'C' => LTR_C,
                    'D' => LTR_D,
                    'E' => LTR_E,
                    'F' => LTR_F,
                    'G' => LTR_G,
                    'H' => LTR_H,
                    'I' => LTR_I,
                    'J' => LTR_J,
                    'K' => LTR_K,
                    'L' => LTR_L,
                    'M' => LTR_M,
                    'N' => LTR_N,
                    'O' => LTR_O,
                    'P' => LTR_P,
                    'Q' => LTR_Q,
                    'R' => LTR_R,
                    'S' => LTR_S,
                    'T' => LTR_T,
                    'U' => LTR_U,
                    'V' => LTR_V,
                    'W' => LTR_W,
                    'X' => LTR_X,
                    'Y' => LTR_Y,
                    'Z' => LTR_Z,
                    _ => LTR_
                };
                let _ = if ch_leds[r][c] == 1 { self.led_matrix.on(c, r) } else { self.led_matrix.off(c, r) };
            }
        }
    }

    fn display_chars(&self) {
        match self.status.get() {
            Status::Idle => {
                unreachable!();
            }
            Status::Processing(idx, len, process_id, delay) => {
                if idx < len {
                    // Show the first remaining letter

                    let _res = self.access_grant.enter(process_id, |app_storage, _upcalls_table| {
                        // Result<Result<(), ErrorCode>, Error>
                        app_storage.text.enter(|text| {
                            self.display_char(text[idx].get());
                        })
                    });

                    self.status.set(Status::Processing(idx + 1, len, process_id, delay));
                    self.alarm.set_alarm(self.alarm.now(), self.alarm.ticks_from_ms(delay));
                } else {
                    // Return in userspace
                    let _ = self.access_grant.enter(process_id, |_app_storage, upcalls_table| {
                        let _ = upcalls_table.schedule_upcall(0, (0, 0, 0));
                    });

                    // Stop all LEDs
                    for c in 0..self.led_matrix.cols_len() {
                        for r in 0..self.led_matrix.rows_len() {
                            let _ = self.led_matrix.off(c, r);
                        }
                    }
                    self.status.set(Status::Idle);
                }
            }
        }
    }
}

impl<'a, A: Alarm<'a>, L: Pin> SyscallDriver for LedMatrixText<'a, A, L> {
    fn command(
        &self,
        command_num: usize,
        delay: usize,
        _r3: usize,
        process_id: ProcessId,
    ) -> CommandReturn {
        match command_num {
            0 => CommandReturn::success(),
            1 => {
                if let Status::Idle = self.status.get() {
                    let res = self.access_grant.enter(process_id, |app_storage, _upcalls_table| {
                        // Result<Result<(), ErrorCode>, Error>
                        app_storage.text.enter(|text| {
                            // In process of displaying the text
                            self.status.set(Status::Processing(1, text.len(), process_id, delay as u32));
                            self.display_char(text[0].get());
                            self.alarm.set_alarm(self.alarm.now(), self.alarm.ticks_from_ms(delay as u32));
                        })
                    });
                    match res {
                        Ok(Ok(())) => CommandReturn::success(),
                        Ok(Err(err)) => CommandReturn::failure(err.into()),
                        Err(err) => CommandReturn::failure(err.into()),
                    }
                } else {
                    CommandReturn::failure(ErrorCode::BUSY)
                }
            },
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
            // text
            0 => {
                let res = self.access_grant.enter(process_id, |app_storage, _upcalls_table| {
                    core::mem::swap(&mut app_storage.text, &mut buffer);
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
        self.access_grant.enter(process_id, |_app_data, _upcalls_table| {})
    }
}

impl<'a, A: Alarm<'a>, L: Pin> AlarmClient for LedMatrixText<'a, A, L> {
    fn alarm(&self) {
        self.display_chars();
    }
}