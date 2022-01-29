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

// ----------------------------------------------
// Capital letters
// ----------------------------------------------

const L_A: [[u8; 5]; 5] = [
    [0, 1, 1, 1, 0],
    [1, 0, 0, 0, 1],
    [1, 1, 1, 1, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
];
const L_B: [[u8; 5]; 5] = [
    [1, 1, 1, 1, 0],
    [1, 0, 0, 0, 1],
    [1, 1, 1, 1, 0],
    [1, 0, 0, 0, 1],
    [1, 1, 1, 1, 0],
];
const L_C: [[u8; 5]; 5] = [
    [0, 1, 1, 1, 1],
    [1, 0, 0, 0, 0],
    [1, 0, 0, 0, 0],
    [1, 0, 0, 0, 0],
    [0, 1, 1, 1, 1],
];
const L_D: [[u8; 5]; 5] = [
    [1, 1, 1, 1, 0],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 1, 1, 1, 0],
];
const L_E: [[u8; 5]; 5] = [
    [1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0],
    [1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0],
    [1, 1, 1, 1, 1],
];
const L_F: [[u8; 5]; 5] = [
    [1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0],
    [1, 1, 1, 0, 0],
    [1, 0, 0, 0, 0],
    [1, 0, 0, 0, 0],
];
const L_G: [[u8; 5]; 5] = [
    [0, 1, 1, 1, 1],
    [1, 0, 0, 0, 0],
    [1, 0, 1, 1, 1],
    [1, 0, 0, 0, 1],
    [0, 1, 1, 1, 1],
];
const L_H: [[u8; 5]; 5] = [
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 1, 1, 1, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
];
const L_I: [[u8; 5]; 5] = [
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
];
const L_J: [[u8; 5]; 5] = [
    [0, 0, 1, 1, 1],
    [0, 0, 0, 0, 1],
    [0, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [0, 1, 1, 1, 0],
];
const L_K: [[u8; 5]; 5] = [
    [1, 0, 0, 1, 0],
    [1, 0, 1, 0, 0],
    [1, 1, 1, 1, 0],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
];
const L_L: [[u8; 5]; 5] = [
    [1, 0, 0, 0, 0],
    [1, 0, 0, 0, 0],
    [1, 0, 0, 0, 0],
    [1, 0, 0, 0, 0],
    [1, 1, 1, 1, 1],
];
const L_M: [[u8; 5]; 5] = [
    [1, 0, 0, 0, 1],
    [1, 1, 0, 1, 1],
    [1, 0, 1, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
];
const L_N: [[u8; 5]; 5] = [
    [1, 1, 0, 0, 1],
    [1, 0, 1, 0, 1],
    [1, 0, 1, 0, 1],
    [1, 0, 1, 0, 1],
    [1, 0, 0, 1, 1],
];
const L_O: [[u8; 5]; 5] = [
    [0, 1, 1, 1, 0],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [0, 1, 1, 1, 0],
];
const L_P: [[u8; 5]; 5] = [
    [1, 1, 1, 1, 0],
    [1, 0, 0, 0, 1],
    [1, 1, 1, 1, 0],
    [1, 0, 0, 0, 0],
    [1, 0, 0, 0, 0],
];
const L_Q: [[u8; 5]; 5] = [
    [0, 1, 1, 0, 0],
    [1, 0, 0, 1, 0],
    [1, 0, 0, 1, 0],
    [1, 0, 0, 1, 0],
    [0, 1, 1, 1, 1],
];
const L_R: [[u8; 5]; 5] = [
    [1, 1, 1, 1, 0],
    [1, 0, 0, 0, 1],
    [1, 1, 1, 1, 0],
    [1, 0, 0, 1, 0],
    [1, 0, 0, 0, 1],
];
const L_S: [[u8; 5]; 5] = [
    [0, 1, 1, 1, 1],
    [1, 0, 0, 0, 0],
    [0, 1, 1, 1, 0],
    [0, 0, 0, 0, 1],
    [1, 1, 1, 1, 0],
];
const L_T: [[u8; 5]; 5] = [
    [1, 1, 1, 1, 1],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
];
const L_U: [[u8; 5]; 5] = [
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [0, 1, 1, 1, 0],
];
const L_V: [[u8; 5]; 5] = [
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [0, 1, 0, 1, 0],
    [0, 1, 0, 1, 0],
    [0, 0, 1, 0, 0],
];
const L_W: [[u8; 5]; 5] = [
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 1, 0, 1],
    [1, 1, 0, 1, 1],
    [1, 0, 0, 0, 1],
];
const L_X: [[u8; 5]; 5] = [
    [1, 0, 0, 0, 1],
    [0, 1, 0, 1, 0],
    [0, 0, 1, 0, 0],
    [0, 1, 0, 1, 0],
    [1, 0, 0, 0, 1],
];
const L_Y: [[u8; 5]; 5] = [
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [0, 1, 1, 1, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
];
const L_Z: [[u8; 5]; 5] = [
    [1, 1, 1, 1, 1],
    [0, 0, 0, 0, 1],
    [0, 1, 1, 1, 0],
    [1, 0, 0, 0, 0],
    [1, 1, 1, 1, 1],
];

// ----------------------------------------------
// Small letters
// ----------------------------------------------

const L_a: [[u8; 5]; 5] = [
    [1, 1, 1, 1, 0],
    [0, 0, 0, 0, 1],
    [0, 1, 1, 1, 1],
    [1, 0, 0, 0, 1],
    [0, 1, 1, 1, 1],
];
const L_b: [[u8; 5]; 5] = [
    [1, 0, 0, 0, 0],
    [1, 1, 1, 1, 0],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 1, 1, 1, 0],
];
const L_c: [[u8; 5]; 5] = [
    [0, 1, 1, 1, 0],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 0],
    [1, 0, 0, 0, 1],
    [0, 1, 1, 1, 0],
];
const L_d: [[u8; 5]; 5] = [
    [0, 0, 0, 0, 1],
    [0, 1, 1, 1, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [0, 1, 1, 1, 1],
];
const L_e: [[u8; 5]; 5] = [
    [0, 1, 1, 1, 0],
    [1, 0, 0, 0, 1],
    [1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0],
    [0, 1, 1, 1, 1],
];
const L_f: [[u8; 5]; 5] = [
    [0, 1, 1, 1, 1],
    [1, 0, 0, 0, 0],
    [1, 1, 1, 0, 0],
    [1, 0, 0, 0, 0],
    [1, 0, 0, 0, 0],
];
const L_g: [[u8; 5]; 5] = [
    [0, 1, 1, 1, 0],
    [1, 0, 0, 0, 1],
    [0, 1, 1, 1, 1],
    [0, 0, 0, 0, 1],
    [1, 1, 1, 1, 0],
];
const L_h: [[u8; 5]; 5] = [
    [1, 0, 0, 0, 0],
    [1, 1, 1, 1, 0],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
];
const L_i: [[u8; 5]; 5] = [
    [0, 0, 1, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
];
const L_j: [[u8; 5]; 5] = [
    [0, 0, 0, 0, 1],
    [0, 0, 0, 0, 1],
    [0, 0, 0, 0, 1],
    [0, 0, 0, 0, 1],
    [1, 1, 1, 1, 0],
];
const L_k: [[u8; 5]; 5] = [
    [1, 0, 0, 0, 1],
    [1, 0, 0, 1, 0],
    [1, 1, 1, 0, 0],
    [1, 0, 0, 1, 0],
    [1, 0, 0, 0, 1],
];
const L_l: [[u8; 5]; 5] = [
    [1, 0, 0, 0, 0],
    [1, 0, 0, 0, 0],
    [1, 0, 0, 0, 0],
    [1, 0, 0, 0, 0],
    [0, 1, 1, 1, 1],
];
const L_m: [[u8; 5]; 5] = [
    [0, 1, 0, 1, 0],
    [1, 0, 1, 0, 1],
    [1, 0, 1, 0, 1],
    [1, 0, 1, 0, 1],
    [1, 0, 1, 0, 1],
];
const L_n: [[u8; 5]; 5] = [
    [1, 1, 1, 1, 0],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
];
const L_o: [[u8; 5]; 5] = [
    [0, 1, 1, 1, 0],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [0, 1, 1, 1, 0],
];
const L_p: [[u8; 5]; 5] = [
    [1, 1, 1, 1, 0],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 1, 1, 1, 0],
    [1, 0, 0, 0, 0],
];
const L_q: [[u8; 5]; 5] = [
    [0, 1, 1, 1, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [0, 1, 1, 1, 1],
    [0, 0, 0, 0, 1],
];
const L_r: [[u8; 5]; 5] = [
    [1, 0, 1, 1, 0],
    [1, 1, 0, 0, 1],
    [1, 0, 0, 0, 0],
    [1, 0, 0, 0, 0],
    [1, 0, 0, 0, 0],
];
const L_s: [[u8; 5]; 5] = [
    [0, 1, 1, 1, 1],
    [1, 0, 0, 0, 0],
    [0, 1, 1, 1, 0],
    [0, 0, 0, 0, 1],
    [1, 1, 1, 1, 0],
];
const L_t: [[u8; 5]; 5] = [
    [1, 0, 0, 0, 0],
    [1, 1, 1, 0, 0],
    [1, 0, 0, 0, 0],
    [1, 0, 0, 0, 1],
    [0, 1, 1, 1, 0],
];
const L_u: [[u8; 5]; 5] = [
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 1, 1],
    [0, 1, 1, 0, 1],
];
const L_v: [[u8; 5]; 5] = [
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [0, 1, 0, 1, 0],
    [0, 1, 0, 1, 0],
    [0, 0, 1, 0, 0],
];
const L_w: [[u8; 5]; 5] = [
    [1, 0, 1, 0, 1],
    [1, 0, 1, 0, 1],
    [1, 0, 1, 0, 1],
    [1, 0, 1, 0, 1],
    [0, 1, 0, 1, 0],
];
const L_x: [[u8; 5]; 5] = [
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [0, 1, 1, 1, 0],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
];
const L_y: [[u8; 5]; 5] = [
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [0, 1, 1, 1, 1],
    [0, 0, 0, 0, 1],
    [1, 1, 1, 1, 0],
];
const L_z: [[u8; 5]; 5] = [
    [1, 1, 1, 1, 1],
    [0, 0, 0, 1, 0],
    [0, 0, 1, 0, 0],
    [0, 1, 0, 0, 0],
    [1, 1, 1, 1, 1],
];

// ----------------------------------------------
// Digits
// ----------------------------------------------

const L_0: [[u8; 5]; 5] = [
    [0, 1, 1, 1, 0],
    [1, 0, 0, 1, 1],
    [1, 0, 1, 0, 1],
    [1, 1, 0, 0, 1],
    [0, 1, 1, 1, 0],
];
const L_1: [[u8; 5]; 5] = [
    [0, 0, 1, 0, 0],
    [0, 1, 1, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 1, 1, 1, 0],
];
const L_2: [[u8; 5]; 5] = [
    [1, 1, 1, 1, 0],
    [0, 0, 0, 0, 1],
    [0, 1, 1, 1, 0],
    [1, 0, 0, 0, 0],
    [1, 1, 1, 1, 1],
];
const L_3: [[u8; 5]; 5] = [
    [1, 1, 1, 1, 1],
    [0, 0, 0, 0, 1],
    [0, 0, 1, 1, 0],
    [0, 0, 0, 0, 1],
    [1, 1, 1, 1, 0],
];
const L_4: [[u8; 5]; 5] = [
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [0, 1, 1, 1, 1],
    [0, 0, 0, 0, 1],
    [0, 0, 0, 0, 1],
];
const L_5: [[u8; 5]; 5] = [
    [1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0],
    [1, 1, 1, 1, 0],
    [0, 0, 0, 0, 1],
    [1, 1, 1, 1, 0],
];
const L_6: [[u8; 5]; 5] = [
    [0, 1, 1, 1, 1],
    [1, 0, 0, 0, 0],
    [1, 1, 1, 1, 0],
    [1, 0, 0, 0, 1],
    [0, 1, 1, 1, 0],
];
const L_7: [[u8; 5]; 5] = [
    [1, 1, 1, 1, 1],
    [0, 0, 0, 0, 1],
    [0, 0, 0, 1, 0],
    [0, 0, 0, 1, 0],
    [0, 0, 0, 1, 0],
];
const L_8: [[u8; 5]; 5] = [
    [0, 1, 1, 1, 0],
    [1, 0, 0, 0, 1],
    [0, 1, 1, 1, 0],
    [1, 0, 0, 0, 1],
    [0, 1, 1, 1, 0],
];
const L_9: [[u8; 5]; 5] = [
    [0, 1, 1, 1, 0],
    [1, 0, 0, 0, 1],
    [0, 1, 1, 1, 1],
    [0, 0, 0, 0, 1],
    [0, 0, 1, 1, 0],
];

// ----------------------------------------------
// Other symbols
// ----------------------------------------------

const L_dot: [[u8; 5]; 5] = [
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 1, 0, 0],
];
const L_qu: [[u8; 5]; 5] = [
    [0, 1, 1, 1, 0],
    [1, 0, 0, 0, 1],
    [0, 0, 1, 1, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 1, 0, 0],
];
const L_ok: [[u8; 5]; 5] = [
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 1],
    [0, 0, 0, 1, 0],
    [1, 0, 1, 0, 0],
    [0, 1, 0, 0, 0],
];
const L_: [[u8; 5]; 5] = [
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
];

// ----------------------------------------------

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
                    // Capital letters
                    'A' => L_A,
                    'B' => L_B,
                    'C' => L_C,
                    'D' => L_D,
                    'E' => L_E,
                    'F' => L_F,
                    'G' => L_G,
                    'H' => L_H,
                    'I' => L_I,
                    'J' => L_J,
                    'K' => L_K,
                    'L' => L_L,
                    'M' => L_M,
                    'N' => L_N,
                    'O' => L_O,
                    'P' => L_P,
                    'Q' => L_Q,
                    'R' => L_R,
                    'S' => L_S,
                    'T' => L_T,
                    'U' => L_U,
                    'V' => L_V,
                    'W' => L_W,
                    'X' => L_X,
                    'Y' => L_Y,
                    'Z' => L_Z,
                    // Small letters
                    'a' => L_a,
                    'b' => L_b,
                    'c' => L_c,
                    'd' => L_d,
                    'e' => L_e,
                    'f' => L_f,
                    'g' => L_g,
                    'h' => L_h,
                    'i' => L_i,
                    'j' => L_j,
                    'k' => L_k,
                    'l' => L_l,
                    'm' => L_m,
                    'n' => L_n,
                    'o' => L_o,
                    'p' => L_p,
                    'q' => L_q,
                    'r' => L_r,
                    's' => L_s,
                    't' => L_t,
                    'u' => L_u,
                    'v' => L_v,
                    'w' => L_w,
                    'x' => L_x,
                    'y' => L_y,
                    'z' => L_z,
                    // Digits
                    '0' => L_0,
                    '1' => L_1,
                    '2' => L_2,
                    '3' => L_3,
                    '4' => L_4,
                    '5' => L_5,
                    '6' => L_6,
                    '7' => L_7,
                    '8' => L_8,
                    '9' => L_9,
                    // Other symbols
                    '.' => L_dot,
                    '?' => L_qu,
                    '^' => L_ok,
                    _ => L_
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