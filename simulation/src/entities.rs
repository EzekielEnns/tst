use crate::render::RenderValue;
use crate::skills::{Skill, Combo};
use crate::stats::Stats;


//this class represents how a system gets rendered 
//so things like animations and stuff would use this 
//interface to execute onto the Entities
pub trait Entity {
    fn render(&self) -> RenderValue;
    //would be used somehow with a render/animtaion function
    fn get_mut(&mut self) -> &mut RenderValue;
}

pub struct Actor { 
    pub render_value: RenderValue,
    pub items: Vec<Item>,
    pub is_hostile: bool,
    pub skills: Vec<&'static Skill>, // first 4 are the skills equipted
    pub combos: Vec<Combo>,
}

impl Actor {
    pub fn render_active_skills(&self)-> (*mut u8,usize){
        let mut skills:[Option<&Skill>;4] = [None;4];
        for i in 0..if self.combos.len() < 4 {self.combos.len()} else {4}{
            let cb =  &self.combos[i];
            skills[i] = cb.combo[cb.index];
        } 
        let mut buf = bendy::serde::to_bytes(&skills).unwrap();
        //TODO deal with len
        let ptr = buf.as_mut_ptr();
        let len = buf.len();
        std::mem::forget(buf);
        (ptr,len)
    }
}
impl Entity for Actor{
    fn render(&self) -> RenderValue { self.render_value }
    fn get_mut(&mut self) -> &mut RenderValue {&mut self.render_value}
    
}

#[derive(Clone, Copy)]
pub struct Tile { pub render_value:RenderValue, pub collision:bool }
impl Entity for Tile {
    fn get_mut(&mut self) -> &mut RenderValue { &mut self.render_value}
    fn render(&self) -> RenderValue { self.render_value }
}
#[derive(Clone, Copy)]
pub struct Item { 
   pub render_value: RenderValue,
   pub name: &'static str,
   pub modifyer: Stats,
   pub consumable:bool,
}
impl Entity for Item {
    fn get_mut(&mut self) -> &mut RenderValue { &mut self.render_value}
    fn render(&self) -> RenderValue { self.render_value }
    //this would trigger a pickup and return true
    //moving this item inside the actor that collided with it 
}
