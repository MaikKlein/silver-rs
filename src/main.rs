#![feature(globs)]


extern crate gl;
extern crate glfw;
extern crate image;
extern crate cgmath;

use gl::types::*;
use glfw::{Context, OpenGlProfileHint, WindowHint, WindowMode,Action,Key};
use std::mem;
use std::ptr;
use std::str;
use image::{GenericImage,ImageBuf};

// Vertex data

mod Shader;
mod Sprite;

// Shader sources




fn main() {
    use Shader::*;
    use Sprite::*;
    use cgmath::*;
    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // Choose a GL profile that is compatible with OS X 10.7+
    glfw.window_hint(WindowHint::ContextVersion(3, 2));
    glfw.window_hint(WindowHint::OpenglForwardCompat(true));
    glfw.window_hint(WindowHint::OpenglProfile(OpenGlProfileHint::Core));

    let (window, events) = glfw.create_window(1920, 1080, "OpenGL", WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // It is essential to make the context current before calling `gl::load_with`.
    window.make_current();

    // Load the OpenGL function pointers
    gl::load_with(|s| window.get_proc_address(s));

    window.set_all_polling(true);
    let mut current_time = glfw.get_time();
    let mut last_time  = glfw.get_time();


    let s = Sprite::new();
    let mut s1 = Sprite::new();

    let mut cam = Cam2D::new(Vector2::new(0.,0.));
    let mut angle = 0.0f32;
    while !window.should_close() {
        last_time = current_time;
        current_time = glfw.get_time();
        let dt = (current_time - last_time) as f32;
        s1.rotate(cgmath::rad(1.0f32 * dt));
        // Poll events
        glfw.poll_events();
        for event in glfw::flush_messages(&events) {
            handle_window_event(&window, event);
        }


        Draw::clear_color(0.3, 0.3, 0.3, 1.0);
        Draw::clear(gl::COLOR_BUFFER_BIT);
        //Draw::arrays(gl::TRIANGLE_STRIP,0,4);
        // Swap buffers
        s.render(&cam);
        s1.render(&cam);
        window.swap_buffers();
        println!("{}",1.0/dt);
    }

    unsafe {
        // Cleanup
        //    gl::DeleteProgram(program);
        //    gl::DeleteShader(fs);
        //    gl::DeleteShader(vs);
        //    gl::DeleteBuffers(1, &vbo);
        //    gl::DeleteVertexArrays(1, &vao);
    }
}
fn handle_window_event(window: &glfw::Window, 
                       (time, event): (f64, glfw::WindowEvent)) {
    match event{
        glfw::WindowEvent::Key(key, scancode, action, mods) => {
            println!("Time: {}, Key: {}, ScanCode: {}, Action: {}, Modifiers: [{}]", time, key, scancode, action, mods);
            match (key,action){

                (Key::Escape,Action::Press) => window.set_should_close(true),
                _ => {}
            }

        }
        _ => {}
    }

}
