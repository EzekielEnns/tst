use crate::render::RenderData;
use crate::stats::Stats;


//this class represents how a system gets rendered 
//so things like animations and stuff would use this 
//interface to execute onto the Entities
pub trait Entity {
    fn render(&self) -> RenderData;
    //would be used somehow with a render/animtaion function
    fn get_mut(&mut self) -> &mut RenderData;
}

pub struct Actor { 
    pub glyph: RenderData,
    pub items: Vec<Item>,
    pub is_hostile: bool,
}
impl Entity for Actor{
    fn render(&self) -> RenderData { self.glyph }
    fn get_mut(&mut self) -> &mut RenderData {&mut self.glyph}
    
}
impl Actor { }

pub struct Tile { pub glyph:RenderData, pub collision:bool }
impl Entity for Tile {
    fn get_mut(&mut self) -> &mut RenderData { &mut self.glyph}
    fn render(&self) -> RenderData { self.glyph }
}
#[derive(Clone, Copy)]
pub struct Item { 
   pub glyph: RenderData,
   pub name: &'static str,
   pub modifyer: Stats,
   pub consumable:bool,
}
impl Entity for Item {
    fn get_mut(&mut self) -> &mut RenderData { &mut self.glyph}
    fn render(&self) -> RenderData { self.glyph }
    //this would trigger a pickup and return true
    //moving this item inside the actor that collided with it 
}
