use Sprite;
use Shader::*;
use Sprite::*;
use cgmath::*;
use gl::types::*;
use gl;
type Vec2D = Vector2<GLfloat>;
type Model = Matrix2<GLfloat>;
pub struct Actor{
    positions: Vec<Vector2<GLfloat>>,
    angles: Vec<Rad<GLfloat>>,
    sprites: Vec<Texture>,
    shader2d: Shader2D,
}

impl Actor{
    pub fn new() -> Actor{
        Actor{positions: Vec::new(),angles: Vec::new(),sprites: Vec::new(),shader2d: Shader2D::new()} 
    }
    pub fn create_actor(&mut self,pos: Vec2D, rad: Rad<GLfloat>, sprite: Texture)-> uint{
        self.positions.push(pos);  
        self.angles.push(rad);
        self.sprites.push(sprite);
        self.sprites.len() - 1
    }
    pub  fn create_actor_default(&mut self) -> uint{
        let tex = create_sprite_tex(gl::TEXTURE0,&Path::new("/home/maik/Downloads/lamp.png"));
        self.create_actor(Vector2::new(0.,0.),rad(0.), tex)
    }
    pub  fn render(&self,cam: &Cam2D){
        for s in range(0u,self.sprites.len()){
            let rot = Rotation2::from_angle(self.angles[s]);
            self.shader2d.render(&self.positions[s], rot.to_matrix2(),cam.get_mat(),&self.shader2d.vao);
        } 
    }
}
