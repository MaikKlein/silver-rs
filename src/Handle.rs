use std::default::Default;
use std::cell::RefCell;
use std::rc::Rc;


pub struct Storage<T>{
    counter: Vec<u16>,
    vec: Vec<T>
}
impl<T: Clone + Default> Storage<T>{
    pub fn new() -> Storage<T>{
        Storage{counter: Vec::new(), vec: Vec::new()}
    }
    pub fn get(&self, h: &Handle<T>)-> T{
        let index = h.id;
        self.vec[index].clone()
    }
    pub fn set(&mut self, h: &Handle<T>, t: T){
        let index = h.id;
        self.vec[index] = t;
    }
    pub fn create(&mut self) -> Handle<T>{
        let t: T = Default::default(); 
        self.vec.push(t);
        self.counter.push(0);
        Handle{id: self.vec.len()-1}
    }
}
struct Handle<T>{
    id: uint
}
