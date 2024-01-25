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
    //TODO add projectile
}


struct Combo {
    pub combo:Vec<&'static Skill>,
    pub index: usize
}
