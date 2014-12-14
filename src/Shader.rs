extern crate gl;
extern crate cgmath;

use gl::types::*;
use glfw::{Context, OpenGlProfileHint, WindowHint, WindowMode,Action,Key};
use std::mem;
use std::ptr;
use std::str;
use cgmath::*;

static VS_SRC: &'static str =
"#version 150\n\
    uniform vec2 trans;\n\
    uniform mat2 rot;\n\
    uniform mat4 view;\n\
    in vec2 position;\n\
    in vec2 uv;\n\
    out vec2 out_uv;\n\
    void main() {\n\
       out_uv = uv;
       vec2 new_pos = (rot * position) + trans;
       gl_Position =  view * vec4(new_pos, 0.0, 1.0);\n\
    }";

    static FS_SRC: &'static str =
        "#version 150\n\
    uniform sampler2D tex;\n\
    out vec4 out_color;\n\
    in vec2 out_uv;\n\
    void main() {\n\
       out_color = texture(tex,out_uv);\n\
    }";
pub struct Shader2D{
    program: Program,
    loc_trans: GLint,
    loc_tex:   GLint,
    loc_view:  GLint,
    loc_rot:  GLint,
    pub vao: ArrayBuffer
}
impl Shader2D{
    pub fn new() -> Shader2D{
        use Sprite::*;
        let vs = Shader::new_vs(VS_SRC);
        let fs = Shader::new_fs(FS_SRC);
        let p = Program::new(vs, fs);
        Program::bind(&p);
        let loc_tex = Program::get_uniform_location(&p,"tex");
        let loc_trans = Program::get_uniform_location(&p,"trans");
        let loc_view = Program::get_uniform_location(&p,"view");
        let loc_rot = Program::get_uniform_location(&p,"rot");
        let vao = create_sprite_vao(&p);
        Shader2D{program:p, loc_trans: loc_trans, loc_view: loc_view, loc_tex: loc_tex, vao: vao,loc_rot:loc_rot}
    }
    pub fn render(&self
              ,trans: &Vector2<GLfloat>
              ,rot:   Matrix2<GLfloat>
              ,view:  Matrix4<GLfloat>
              ,vao:   &ArrayBuffer){
        Program::bind(&self.program);
        ArrayBuffer::bind(vao);
        Program::uniform1i(self.loc_tex,0);
        Program::uniform2f(self.loc_trans,trans);
        Program::uniform_mat4(self.loc_view, view);
        Program::uniform_mat2(self.loc_rot, &rot);
        Draw::arrays(gl::TRIANGLE_STRIP,0,4);
        ArrayBuffer::unbind();
    }
}

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
    pub fn uniform2f(loc :GLint, v: &cgmath::Vector2<GLfloat>){
        unsafe{
            gl::Uniform2f(loc,v.x,v.y);
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
