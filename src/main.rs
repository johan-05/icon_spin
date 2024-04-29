extern crate ogl33 as gl;

use beryllium::{
  events::Event,
  init::InitFlags,
  video::{CreateWinArgs, GlContextFlags, GlProfile},
  Sdl,
};


fn main() {
  let sdl = Sdl::init(InitFlags::EVERYTHING);
  sdl.set_gl_context_major_version(3).unwrap();
  sdl.set_gl_context_minor_version(3).unwrap();
  sdl.set_gl_profile(GlProfile::Core).unwrap();
  let mut flags = GlContextFlags::default();

  if cfg!(debug_asserts) {
    flags |= GlContextFlags::DEBUG;
  }
  sdl.set_gl_context_flags(flags).unwrap();

  let win = sdl
    .create_gl_window(CreateWinArgs {
      title: "Icon-Spin",
      width: 800,
      height: 600,
      ..Default::default()
    })
    .expect("couldn't make a window and context");

    unsafe {
        gl::load_gl_with(|f_name| win.get_proc_address(f_name as *const u8));
    }

  'main_loop: loop {

    while let Some((event, _timestamp)) = sdl.poll_events() {
      match event {
        Event::Quit => break 'main_loop,
        _ => (),
      }
    }
    unsafe {
        gl::glClear(gl::GL_COLOR_BUFFER_BIT);
    }

    unsafe {
        gl::glClearColor(0.2, 0.3, 0.3, 1.0);
    }

    win.swap_window();

  }
}