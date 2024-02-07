use serde::{Deserialize, Serialize};
use crate::stats::Stats;

#[derive(Serialize, Deserialize)]
pub struct Skill {
    pub cost: Stats,
    pub effect: Stats,
    pub modifer: bool,
    pub deffense: bool,
    pub name: &'static str,
    pub range: i32,
}


struct Combo {
    pub combo:Vec<&'static Skill>,
    pub index: usize
}

struct Team {
    pub stats: Stats,
    pub max: Stats,

    pub damage: Vec<&'static Skill>,
    pub deffense: Vec<&'static Skill>,
}

impl Team {
    fn apply_dmg(&mut self)-> Stats {
        //deques deffense buffer
        todo!()
    }
    fn get_dmg(&mut self)-> Stats {
        //deques damage buffer
        todo!()
    }

    fn rest(&mut self) {
        todo!()
    }
}
