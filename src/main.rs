#![deny(clippy::all)]
#![forbid(unsafe_code)]

use lazy_static::lazy_static;
use std::sync::{Mutex, Arc};
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use std::rc::Rc;


#[allow(unused_macros)]
macro_rules! opcrel {
    ($a: expr) => {
        $a
    }
}

lazy_static!{
    static ref EMULATOR_STATE: Mutex<EmulatorState> = Mutex::new(EmulatorState::default());
}

struct EmulatorState {
    op_code: u8,
    op_crel: u8,
    temp8: u8,
    operand: u8,
    carry: u8,
    negative: u8,
    rom0: u8,
    rom1: u8,
    io: [u8; 512],
    video_ram: [u8; 8192],
    work_ram: [u8; 16384],
    external_ram: u8,
    external_ram_bank: u8,
    reg16: [u16; 16],
    ime: u8,
    halt: u8,
    key_state: u8,
    /* i16 (2 byte) variables */
    program_counter: i16,

}

impl Default for EmulatorState {
    fn default() -> Self {
        EmulatorState{
            op_code: 0,
            op_crel: 0,
            temp8: 0,
            operand: 0,
            carry: 0,
            negative: 0,
            rom0: 0,
            rom1: 0,
            io: [0; 512],
            video_ram: [0; 8192],
            work_ram: [0; 16384],
            external_ram: 0,
            external_ram_bank: 0,
            reg16: [19, 0, 216, 0, 77, 1, 176, 1, 0, 0, 0, 0 ,0 ,0, 0, 0], /* F=reg8[6] A=reg8[7]*/
            ime: 0,
            halt: 0,
            key_state: 0,
            program_counter: 0
        }
    }
}

impl EmulatorState {

    fn reg8(&self) -> [u8; 8] {
        [
            self.reg16[0] as u8,
            self.reg16[1] as u8,
            self.reg16[2] as u8,
            self.reg16[3] as u8,
            self.reg16[4] as u8,
            self.reg16[5] as u8,
            self.reg16[6] as u8,
            self.reg16[7] as u8,
        ]
    }

    fn f(&self) -> u8 {
        self.reg8()[6]
    }

    fn a(&self) -> u8 {
        self.reg8()[7]
    }

    fn i_f(&self) -> u8 {
        self.io[271]
    }

    fn l_c_d_c(&self) -> u8 {
        self.io[320]
    }

    fn l_y(&self) -> u8 {
        self.io[324]
    }

    fn get_h_l(&self) -> u16 {
        self.reg16[2]
    }

    fn set_h_l(&mut self, value: u16) {
        self.reg16[2] = value
    }


}



fn main() {

    println!("{:?}", EMULATOR_STATE.lock().unwrap().reg16);

    EMULATOR_STATE.lock().unwrap().set_h_l(200);

    println!("{:?}", EMULATOR_STATE.lock().unwrap().reg16)

}


fn tick(cycles: &i32) -> i32 {
    return cycles + 4
}