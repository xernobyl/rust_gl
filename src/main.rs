#[allow(dead_code)]
#[allow(improper_ctypes)]
#[allow(unused_variables)]
#[allow(non_upper_case_globals)]

#[link(name = "User32")]
#[link(name = "Gdi32")]
#[link(name = "opengl32")]
#[link(name = "C:\\Users\\xernobyl\\documents\\visual studio 2015\\Projects\\whynot\\x64\\Release\\xgllib")]

mod gl_4_5;
use gl_4_5::*;

extern crate rand;


extern {
	fn WindowMain(loopCallback: extern fn(tick: u64, ticks_per_second: u64), resizeCallback: extern fn(width: u32, height: u32)) -> i32;
}

extern fn gl_loop(tick: u64, ticks_per_second: u64) {
	let time: f64 = tick as f64 / ticks_per_second as f64;

	let r = rand::random::<f32>();
	let g = rand::random::<f32>();
	let b = rand::random::<f32>();
	let a = rand::random::<f32>();

	unsafe {
		glClearColor(r, g, b, a);
		glClear(GL_COLOR_BUFFER_BIT);
	}
}

extern fn gl_resize(width: u32, height: u32) {
	println!("{} x {}", width, height);
}

fn main() {
	unsafe {
		WindowMain(gl_loop, gl_resize);
	}
}
