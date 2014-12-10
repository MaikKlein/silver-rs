
extern crate gl;
extern crate image;
extern crate cgmath;
use Shader::*;
use cgmath::*;
use gl::types::*;
// Shader sources
static width: f32 = 1920.0;
static height: f32 = 1080.0;
static VS_SRC: &'static str =
"#version 150\n\
    uniform mat2 model;\n\
    uniform mat4 view;\n\
    in vec2 position;\n\
    in vec2 uv;\n\
    out vec2 out_uv;\n\
    void main() {\n\
       out_uv = uv;
       gl_Position =  view * vec4(model * position, 0.0, 1.0);\n\
    }";

    static FS_SRC: &'static str =
        "#version 150\n\
    uniform sampler2D tex;\n\
    out vec4 out_color;\n\
    in vec2 out_uv;\n\
    void main() {\n\
       out_color = texture(tex,out_uv);\n\
    }";
    fn create_sprite_program() -> Program{
        let vs = Shader::new_vs(VS_SRC);
        let fs = Shader::new_fs(FS_SRC);
        Program::new(vs, fs)
    }
    fn create_sprite_tex(ty: GLenum, path: &Path) -> Texture{
        use image::{GenericImage,ImageBuf};
        let tex = Texture::new(gl::TEXTURE_2D);
        Texture::active_texture(gl::TEXTURE0);
        Texture::bind_texture(&tex);

        Texture::tex_parameter_i(gl::TEXTURE_2D,gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE);
        Texture::tex_parameter_i(gl::TEXTURE_2D,gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE);
        Texture::tex_parameter_i(gl::TEXTURE_2D,gl::TEXTURE_MIN_FILTER, gl::LINEAR);
        Texture::tex_parameter_i(gl::TEXTURE_2D,gl::TEXTURE_MAG_FILTER, gl::LINEAR);

        let image = image::open(path).unwrap();
        let (w,h) = image.dimensions();
        Texture::tex_image_2d(gl::TEXTURE_2D, 0, gl::RGBA as i32,
                              w as i32, h as i32, 0,
                              gl::RGBA, gl::UNSIGNED_BYTE, image.raw_pixels());
        tex
    }
    fn create_sprite_vao(program: &Program)-> ArrayBuffer{
        let v  = vec![
            1.0, 1.0,
            -1.0, 1.0,
            1.0,-1.0,
            -1.0,-1.0,
        ];
        let uv  = vec![
            1.0, 0.0,
            0.0, 0.0,
            1.0, 1.0,
            0.0, 1.0,
        ];
        let vao = ArrayBuffer::new();
        ArrayBuffer::bind(&vao);
        // Create Vertex Array Object
        // Create a Vertex Buffer Object and copy the vertex data to it
        Buffer::new(v,gl::STATIC_DRAW);
        // Use shader program
        Program::bind(program);
        Program::bind_frag_data_loc(program,"out_color",0);

        // Specify the layout of the vertex data
        let pos_attr = Program::get_location(program,"position");
        ArrayBuffer::EnableVertexAttribArray(pos_attr as GLuint);
        ArrayBuffer::vertex_attrib_pointer(pos_attr as GLuint,2,0);

        Buffer::new(uv,gl::STATIC_DRAW);
        let uv_attr = Program::get_location(program,"uv");
        ArrayBuffer::EnableVertexAttribArray(uv_attr as GLuint);
        ArrayBuffer::vertex_attrib_pointer(uv_attr as GLuint,2,0);
        ArrayBuffer::unbind();
        return vao;
    }
    pub struct Sprite{
        program : Program,
        tex : Texture,
        vao: ArrayBuffer,
        model: cgmath::Matrix2<GLfloat>
    }
    impl Sprite{
        pub fn new()-> Sprite{
            let program = create_sprite_program();
            let vao = create_sprite_vao(&program);
            let tex = create_sprite_tex(gl::TEXTURE0,&Path::new("/home/maik/Downloads/wood.png"));
            let id = cgmath::Matrix2::identity();
            Sprite{program: program, tex: tex, vao: vao, model: id}
        }
        pub fn rotate(&mut self, rad: Rad<GLfloat>){
            self.model = Matrix2::from_angle(rad) * self.model; 
        }
        pub fn render(&self, cam: &Cam2D){
            use cgmath::*;
            Program::bind(&self.program);
            ArrayBuffer::bind(&self.vao);
            let loc = Program::get_uniform_location(&self.program,"tex");
            let loc_model = Program::get_uniform_location(&self.program,"model");
            let loc_view = Program::get_uniform_location(&self.program,"view");
            Program::uniform1i(loc,0);
            Program::uniform_mat2(loc_model,&self.model);
            Program::uniform_mat4(loc_view, cam.get_mat());
            Draw::arrays(gl::TRIANGLE_STRIP,0,4);
            ArrayBuffer::unbind();
        }
    }
    pub struct Cam2D{
        ortho: cgmath::Matrix4<GLfloat>,
        pos  : cgmath::Vector2<GLfloat>
    }
    impl Cam2D{
        pub fn new(pos : cgmath::Vector2<GLfloat>) -> Cam2D{
            use cgmath::*;
            let aspect_ratio = width/height;
            let o = ortho(-aspect_ratio,aspect_ratio,-1.0,1.0, 0.0, 1.0);
            Cam2D{ortho: o, pos: pos }
        }

        pub fn translate(&mut self, v:cgmath::Vector2<GLfloat> ){
            let new_pos = self.pos + v;
            self.pos = new_pos;
        }
        pub fn set_pos(&mut self, v:cgmath::Vector2<GLfloat> ){
            self.pos = v;
        }

        pub fn get_mat(&self) -> cgmath::Matrix4<GLfloat>{
            use cgmath::*;
            let view = Matrix4::look_at(&Point3::new(self.pos.x,self.pos.y,0.),
            &Point3::new(self.pos.x,self.pos.y,1.),
            &Vector3::new(0.,1.,0.));
            view * self.ortho
        }

    }
