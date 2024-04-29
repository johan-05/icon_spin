#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
extern crate ogl33 as gl;
use gl::*;

use std::cmp::min;

use beryllium::{
  events::Event,
  init::InitFlags,
  video::{CreateWinArgs, GlContextFlags, GlProfile},
  Sdl,
};



type Vertex = [f32; 3];
const TEST_DATA: [Vertex; 6] = [
    [-1.0, -1.0, 0.0], [ 1.0, -1.0, 0.0], [-1.0, 1.0, 0.0],
    [ 1.0, -1.0, 0.0], [-1.0,  1.0, 0.0], [ 1.0, 1.0, 0.0],
];

const VRTX_SHADER_SOURCE:&str = include_str!("../shaders/test_vrtx.glsl");
const FRGMT_SHADER_SOURCE:&str = include_str!("../shaders/test_frgmt.glsl");


fn main() {

    let instagram_bitmap = {
        let mut f = std::fs::File::open("Instagram_icon.png").unwrap();
        let mut bytes = vec![];
        std::io::Read::read_to_end(&mut f, &mut bytes).unwrap();
        let mut bitmap = imagine::png::parse_png_rgba8(&bytes).unwrap().bitmap;
        bitmap.flip_scanlines();
        bitmap
    };

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
        resizable: true,
        ..Default::default()
    })
    .expect("couldn't make a window and context");
    win.set_swap_interval(beryllium::video::GlSwapInterval::Vsync).unwrap();

    let mut vao = 0;
    let test_program:u32;
    let window_dimension_location:i32;
    unsafe {
        load_gl_with(|f_name| win.get_proc_address(f_name as *const u8));
        glGenVertexArrays(1, &mut vao);
        assert_ne!(vao, 0);
        glBindVertexArray(vao);

        let mut vbo = 0;
        glGenBuffers(1, &mut vbo);
        assert_ne!(vbo, 0);

        glBindBuffer(GL_ARRAY_BUFFER, vbo);
        glBufferData(
            GL_ARRAY_BUFFER,
            std::mem::size_of_val(&TEST_DATA) as isize,
            TEST_DATA.as_ptr().cast(),
            GL_STATIC_DRAW,
        );
        glVertexAttribPointer(
            0,
            3,
            GL_FLOAT,
            GL_FALSE,
            std::mem::size_of::<Vertex>().try_into().unwrap(),
            0 as *const _,
          );
        glEnableVertexAttribArray(0);

        let mut inst_tex = 0;
        glGenTextures(1, &mut inst_tex);
        glBindTexture(GL_TEXTURE_2D, inst_tex);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_BORDER as GLint);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_BORDER as GLint);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR as GLint);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR as GLint);
        glTexImage2D(
            GL_TEXTURE_2D, 
            0, 
            GL_RGBA as GLint,
           instagram_bitmap.width().try_into().unwrap(),
           instagram_bitmap.height().try_into().unwrap(),
           0,
           GL_RGBA,
           GL_UNSIGNED_BYTE,
           instagram_bitmap.pixels().as_ptr().cast()
        );
        glGenerateMipmap(GL_TEXTURE_2D);


        let vrtx_shader = glCreateShader(GL_VERTEX_SHADER);
        assert_ne!(vrtx_shader, 0);
        glShaderSource(
            vrtx_shader,
            1,
            &(VRTX_SHADER_SOURCE.as_bytes().as_ptr().cast()),
            &(VRTX_SHADER_SOURCE.len().try_into().unwrap()),
        );
        glCompileShader(vrtx_shader);
        let mut success = 0;
        glGetShaderiv(vrtx_shader, GL_COMPILE_STATUS, &mut success);

        let frgmt_shader = glCreateShader(GL_FRAGMENT_SHADER);
        assert_ne!(frgmt_shader, 0);
        
        glShaderSource(
            frgmt_shader,
            1,
            &(FRGMT_SHADER_SOURCE.as_bytes().as_ptr().cast()),
            &(FRGMT_SHADER_SOURCE.len().try_into().unwrap()),
        );  
        glCompileShader(frgmt_shader);
        glGetShaderiv(frgmt_shader, GL_COMPILE_STATUS, &mut success);
        if success == 0 {
            panic!("shaders unalived");
        }



        test_program = glCreateProgram();
        glAttachShader(test_program, vrtx_shader);
        glAttachShader(test_program, frgmt_shader);
        glLinkProgram(test_program);
        glDeleteShader(vrtx_shader);
        glDeleteShader(frgmt_shader);

        window_dimension_location = glGetUniformLocation(test_program, "window_dimensions".as_ptr().cast());

        glClearColor(0.9, 0.9, 0.9, 1.0);
    }

    let mut window_width = 800;
    let mut window_height = 600;

  'main_loop: loop {
    while let Some((event, _timestamp)) = sdl.poll_events() {
        match event {
            Event::Quit => break 'main_loop,
            Event::WindowSizeChanged { win_id:_, width, height } => {
                window_width = width;
                window_height = height;
            },
            _ => (),
        }

    }

   unsafe { glViewport(0, 0, window_width, window_height) };

    unsafe {

        glClear(GL_COLOR_BUFFER_BIT);

        glBindVertexArray(vao);
        glUseProgram(test_program);

        glUniform2fv(window_dimension_location, 1, [window_width as f32, window_height as f32].as_ptr());
        glDrawArrays(GL_TRIANGLES, 0, 6);
    }

    win.swap_window();
  }
}

