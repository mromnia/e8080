pub mod disassembler;
pub mod emulator;
pub mod opcode_decoder;

use emulator::*;
use std::mem::transmute;

static OPCODES: &'static str = include_str!("../data/opcodes.txt");
static ROM: &'static [u8] = include_bytes!("../data/invaders.rom");

#[no_mangle]
pub extern "C" fn decoder_new() -> opcode_decoder::OpcodeDecoder {
    opcode_decoder::OpcodeDecoder::new(&OPCODES)
}

#[no_mangle]
pub extern "C" fn am_new() -> *mut ArcadeMachine {
    let ptr = unsafe { transmute(Box::new(ArcadeMachine::new(decoder_new(), ROM))) };
    ptr
}

#[no_mangle]
pub extern "C" fn am_run(am: *mut ArcadeMachine) -> u32 {
    unsafe { (*am).run(1.0 / 60.0 / 2.0) }
}

#[no_mangle]
pub extern "C" fn am_get_render_buffer(am: *mut ArcadeMachine) -> *const u8 {
    unsafe { (*am).get_render_buffer().as_ptr() }
}

#[no_mangle]
pub extern "C" fn am_signal_half_render(am: *mut ArcadeMachine) {
    unsafe { (*am).signal_half_render() }
}

#[no_mangle]
pub extern "C" fn am_signal_finish_render(am: *mut ArcadeMachine) {
    unsafe { (*am).signal_finish_render() }
}

#[no_mangle]
pub extern "C" fn am_coin_key_toggle(am: *mut ArcadeMachine, down: bool) {
    unsafe { (*am).coin_key_toggle(down) }
}

#[no_mangle]
pub extern "C" fn am_start_p1_key_toggle(am: *mut ArcadeMachine, down: bool) {
    unsafe { (*am).start_p1_key_toggle(down) }
}

#[no_mangle]
pub extern "C" fn am_fire_p1_key_toggle(am: *mut ArcadeMachine, down: bool) {
    unsafe { (*am).fire_p1_key_toggle(down) }
}

#[no_mangle]
pub extern "C" fn am_left_p1_key_toggle(am: *mut ArcadeMachine, down: bool) {
    unsafe { (*am).left_p1_key_toggle(down) }
}

#[no_mangle]
pub extern "C" fn am_right_p1_key_toggle(am: *mut ArcadeMachine, down: bool) {
    unsafe { (*am).right_p1_key_toggle(down) }
}
