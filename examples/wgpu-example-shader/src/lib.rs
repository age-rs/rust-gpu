//! Ported to Rust from https://github.com/Tw1ddle/Sky-Shader/blob/master/src/shaders/glsl/sky.fragment

#![no_std]
#![feature(register_attr)]
#![register_attr(spirv)]

use core::panic::PanicInfo;
use spirv_std::{Input, Output, Vec4};

#[spirv(entry = "fragment")]
pub fn main_fs(mut output: Output<Vec4>) {
    output.store(Vec4::new(1.0, 0.0, 0.0, 1.0))
}

#[spirv(entry = "vertex")]
pub fn main_vs(
    #[spirv(builtin = "vertex_index")] mut vert_id: Input<i32>,
    #[spirv(builtin = "position")] mut out_pos: Output<Vec4>,
) {
    let vert_id = vert_id.load();
    out_pos.store(Vec4::new(
        (vert_id - 1) as f32,
        ((vert_id & 1) * 2 - 1) as f32,
        0.0,
        1.0,
    ));
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
