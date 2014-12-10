extern crate gl;
extern crate cgmath;

use gl::types::*;
use glfw::{Context, OpenGlProfileHint, WindowHint, WindowMode,Action,Key};
use std::mem;
use std::ptr;
use std::str;

pub struct Shader{
    handle:GLuint
}
impl Shader{
    pub fn new(src: &str, ty: GLenum)-> Shader{
        let shader;
        unsafe {
            shader = gl::CreateShader(ty);
            // Attempt to compile the shader
            src.with_c_str(|ptr| gl::ShaderSource(shader, 1, &ptr, ptr::null()));
            gl::CompileShader(shader);

            // Get the compile status
            let mut status = gl::FALSE as GLint;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

            // Fail on error
            if status != (gl::TRUE as GLint) {
                let mut len = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = Vec::from_elem(len as uint - 1, 0u8);     // subtract 1 to skip the trailing null character
                gl::GetShaderInfoLog(shader, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
                panic!("{}", str::from_utf8(buf.as_slice()).expect("ShaderInfoLog not valid utf8"));
            }
        }
        Shader{handle: shader}
    }
    pub fn new_vs(src: &str) -> Shader{
        Shader::new(src,gl::VERTEX_SHADER)
    }
    pub fn new_fs(src: &str) -> Shader{
        Shader::new(src,gl::FRAGMENT_SHADER)
    }
}
pub struct Program{
    handle:GLuint
}

impl Program{
    pub fn new(vs: Shader, fs: Shader) -> Program { 
        unsafe {
            let program = gl::CreateProgram();
            gl::AttachShader(program, vs.handle);
            gl::AttachShader(program, fs.handle);
            gl::LinkProgram(program);
            // Get the link status
            let mut status = gl::FALSE as GLint;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

            // Fail on error
            if status != (gl::TRUE as GLint) {
                let mut len: GLint = 0;
                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = Vec::from_elem(len as uint - 1, 0u8);     // subtract 1 to skip the trailing null character
                gl::GetProgramInfoLog(program, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
                panic!("{}", str::from_utf8(buf.as_slice()).expect("ProgramInfoLog not valid utf8"));
            }
            Program{handle:program}
        } 
    }
    pub fn bind(p: &Program){
        unsafe{
            gl::UseProgram(p.handle);
        }
    }
    pub fn get_location(p: &Program, s:&'static str) -> GLint {
        let loc: GLint;
        unsafe{
            loc = s.with_c_str(|ptr| gl::GetAttribLocation(p.handle, ptr));
        }
        loc
    }
    pub fn bind_frag_data_loc(p:&Program, s:&'static str, loc_id: GLuint){
        unsafe{
            s.with_c_str(|ptr| gl::BindFragDataLocation(p.handle, loc_id, ptr));
        }
    }
    pub fn get_uniform_location(p: &Program, s:&'static str) -> GLint {
        let loc: GLint;
        unsafe{
            loc = s.with_c_str(|ptr| gl::GetUniformLocation(p.handle, ptr));
        }
        loc
    }
    pub fn uniform1i(loc :GLint, value: GLint){
        unsafe{
            gl::Uniform1i(loc,value);
        }
    }
    pub fn uniform_mat2(loc :GLint, mat: &cgmath::Matrix2<GLfloat>){
        use cgmath::*;
        unsafe{
            gl::UniformMatrix2fv(loc,1,gl::FALSE,mem::transmute(&mat.as_fixed()[0]));
        }
    }
    pub fn uniform_mat4(loc :GLint, mat: cgmath::Matrix4<GLfloat>){
        use cgmath::*;
        unsafe{
            gl::UniformMatrix4fv(loc,1,gl::FALSE,mem::transmute(&mat.as_fixed()[0]));
        }
    }
}
pub struct Buffer{
    handle:GLuint
}
impl Buffer{
    pub fn new(vertex_data: Vec<GLfloat>,draw: GLenum){
        let mut vbo: GLuint = 0;
        unsafe{
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER,
                           (vertex_data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                           mem::transmute(&vertex_data[0]),
                           gl::STATIC_DRAW);
        }
    }
}
pub struct ArrayBuffer{
    handle:GLuint
}
impl ArrayBuffer{
    pub fn new() -> ArrayBuffer{
        let mut vao: GLuint = 0;
        unsafe{
            // Create Vertex Array Object
            gl::GenVertexArrays(1, &mut vao);
        }
        ArrayBuffer{handle: vao}
    }
    pub fn unbind(){
        unsafe{
            gl::BindVertexArray(0);
        }
    }
    pub fn bind(v: &ArrayBuffer){
        unsafe{
            gl::BindVertexArray(v.handle);
        }
    }
    pub fn EnableVertexAttribArray(pos: GLuint){
        unsafe{
            gl::EnableVertexAttribArray(pos);
        }
    }
    pub fn vertex_attrib_pointer(pos: GLuint,count: GLint, stride: GLint){
        unsafe{
            gl::VertexAttribPointer(pos, count, gl::FLOAT,
                                    gl::FALSE as GLboolean, stride, ptr::null());
        }
    }
}
pub struct Texture{
    mode: GLenum,
    handle:GLuint
}
impl Texture{
    pub fn new(ty : GLenum) -> Texture{
        let mut handle = 0;
        unsafe {
            gl::GenTextures(1,&mut handle);
        }
        Texture{mode:ty,handle:handle}
    }
    pub fn active_texture(ty: GLenum){
        unsafe{
            gl::ActiveTexture(ty);
        }
    }
    pub fn bind_texture(t: &Texture){
        unsafe{
            gl::BindTexture(t.mode,t.handle);
        }
    }
    pub fn tex_parameter_i(tex : GLenum,mode1:GLenum,mode2:GLuint){
        unsafe{
            gl::TexParameteri(tex,mode1,mode2 as GLint); 
        }

    }
    pub fn tex_image_2d(target: GLenum, level: GLint,iformat: GLint,
                        width: GLsizei, height: GLsizei, border: GLint,
                        format: GLenum, ty: GLenum, data: Vec<u8>){
        unsafe{
            gl::TexImage2D(target,level,iformat, width,height,border,format,ty,mem::transmute(&data[0]));
        } 
    }


}
pub struct Draw;

impl Draw{
    pub fn arrays(ty: GLenum,from: GLint,to:GLint){
        unsafe{
            gl::DrawArrays(ty,from,to);
        }
    }
    pub fn clear(ty: GLenum){
        unsafe{
            gl::Clear(ty); 
        }
    }
    pub fn clear_color(x: GLfloat,y: GLfloat,z: GLfloat,a: GLfloat,){
        unsafe{
            gl::ClearColor(x,y,z,a); 
        }
    }
}
