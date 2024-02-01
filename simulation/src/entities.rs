use crate::render::RenderValue;
use crate::stats::Stats;


//this class represents how a system gets rendered 
//so things like animations and stuff would use this 
//interface to execute onto the Entities
pub trait Entity {
    fn render(&self) -> RenderValue;
    //would be used somehow with a render/animtaion function
    fn get_mut(&mut self) -> &mut RenderValue;
}

#[derive(Clone)]
pub struct Actor { 
    pub render_value: RenderValue,
    pub items: Vec<Item>,
    pub is_hostile: bool,
}
impl Entity for Actor{
    fn render(&self) -> RenderValue { self.render_value }
    fn get_mut(&mut self) -> &mut RenderValue {&mut self.render_value}
    
}
impl Actor { }

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
