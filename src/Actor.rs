use Sprite;
use Shader::*;
use Sprite::*;
use cgmath::*;
use gl::types::*;
use gl;
type Vec2D = Vector2<GLfloat>;
type Model = Matrix2<GLfloat>;
pub struct Actor{
    pub positions: Vec<Vector2<GLfloat>>,
    pub angles: Vec<Rad<GLfloat>>,
    pub sprites: Vec<Texture>,
}

impl Actor{
    pub fn new() -> Actor{
        Actor{positions: Vec::new()
             ,angles:    Vec::new()
             ,sprites:   Vec::new()} 
    }
    pub fn create_actor(&mut self,pos: Vec2D, rad: Rad<GLfloat>, sprite: Texture)-> uint{
        self.positions.push(pos);  
        self.angles.push(rad);
        self.sprites.push(sprite);
        self.sprites.len() - 1
    }
}
