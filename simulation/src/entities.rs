use crate::render::RenderValue;
use crate::skills::{Combo, Skill, Team};
use crate::stats::Stats;

//this class represents how a system gets rendered
//so things like animations and stuff would use this
//interface to execute onto the Entities
pub trait Entity {
    fn render(&self) -> RenderValue;
    //would be used somehow with a render/animtaion function
    fn get_mut(&mut self) -> &mut RenderValue;
}

#[derive(Debug)]
pub struct Actor {
    pub render_value: RenderValue,
    pub items: Vec<Item>,
    pub is_hostile: bool,
    pub combos: Vec<Combo>,
}

impl Actor {
    pub fn render_active_skills(&self) -> (*mut u8, usize) {
        let mut skills: [Option<&Skill>; 4] = [None; 4];
        for i in 0..if self.combos.len() < 4 {
            self.combos.len()
        } else {
            4
        } {
            let cb = &self.combos[i];
            skills[i] = cb.combo[cb.index];
        }
        let mut buf = bendy::serde::to_bytes(&skills).unwrap();
        let ptr = buf.as_mut_ptr();
        let len = buf.len();
        std::mem::forget(buf);
        (ptr, len)
    }

    pub fn progress_skill(&mut self, index: usize) -> Option<&'static Skill> {
        let combo = &mut self.combos[index];
        let cb_index = combo.increment(); //note this is the last index
        combo.combo[cb_index]
    }
    
    //TODO move to teams
    pub fn add_to_team(&self, team: &mut Team){
        for i in self.items.iter() {
            //TODO bad code ik
            team.stats += i.modifyer;
            team.max += i.modifyer;
        }
    }
}
impl Entity for Actor {
    fn render(&self) -> RenderValue {
        self.render_value
    }
    fn get_mut(&mut self) -> &mut RenderValue {
        &mut self.render_value
    }
}

#[derive(Clone, Copy,Debug)]
pub struct Tile {
    pub render_value: RenderValue,
    pub collision: bool,
}
impl Entity for Tile {
    fn get_mut(&mut self) -> &mut RenderValue {
        &mut self.render_value
    }
    fn render(&self) -> RenderValue {
        self.render_value
    }
}
#[derive(Clone, Copy,Debug)]
pub struct Item {
    pub render_value: RenderValue,
    pub name: &'static str,
    pub modifyer: Stats,
    pub consumable: bool,
}
impl Entity for Item {
    fn get_mut(&mut self) -> &mut RenderValue {
        &mut self.render_value
    }
    fn render(&self) -> RenderValue {
        self.render_value
    }
    //this would trigger a pickup and return true
    //moving this item inside the actor that collided with it
}
