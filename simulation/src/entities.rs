use crate::render::RenderData;
use crate::stats::Stats;

pub trait Entity {
    fn render(&self) -> RenderData;

    //called when entity will collide with another entity
    //returns weather entity should be 'killed or not'
    fn on_collision<'a>(&mut self, target: &'a Box<dyn Entity>) -> bool;

    //would be used somehow with a render/animtaion function
    fn get_mut(&mut self) -> &mut RenderData;
}

pub struct Actor { pub glyph: RenderData }
impl Entity for Actor{
    fn render(&self) -> RenderData { self.glyph }
    fn get_mut(&mut self) -> &mut RenderData {&mut self.glyph}
    
    //this could trigger combat (a global flag)
    fn on_collision<'a>(&mut self, _trg: &'a Box<dyn Entity>) -> bool { false }
}
impl Actor { }

pub struct Tile { pub glyph:RenderData, pub collision:bool }
impl Entity for Tile {
    fn get_mut(&mut self) -> &mut RenderData { &mut self.glyph}
    fn render(&self) -> RenderData { self.glyph }
    fn on_collision<'a>(&mut self, _trg: &'a Box<dyn Entity>) -> bool { false }
}
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
    fn on_collision<'a>(&mut self, _trg: &'a Box<dyn Entity>) -> bool { false }
}
