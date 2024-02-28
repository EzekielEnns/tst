use crate::{stats::Stats, world::{World, IdxTeam}};
use serde::{ser::SerializeStruct, Serialize};

#[derive(PartialEq)]
pub struct Skill {
    pub cost: Stats,
    pub effect: Stats,
    // pub modifer: bool,
    pub deffense: bool,
    pub name: &'static str,
    //TODO add description
    pub reach: usize,
}
impl Default for Skill {
    fn default() -> Self {
        Skill {
            name: "DUMMY SKILL",
            cost: Stats::default(),
            effect: Stats::default(),
            deffense: false,
            reach: 0,
        }
    }
}
impl Serialize for Skill {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Skill", 2)?;
        state.serialize_field("cost", &self.cost)?;
        state.serialize_field("effect", &self.effect)?;
        state.serialize_field("name", self.name)?;
        state.end()
    }
}

pub struct Combo {
    //None will be used as a single that they are done
    pub combo: Vec<Option<&'static Skill>>,
    pub index: usize,
}
impl Combo {
    pub fn increment(&mut self) -> usize{
        let last = self.index;
        if self.combo[last] != None {
            self.index = if self.index + 1 >= self.combo.len() {
                0
            } else {
                self.index + 1
            };
        }
        last
    }
}

pub struct Team {
    pub stats: Stats,
    pub max: Stats,

    pub damage: Vec<&'static Skill>,
    pub deffense: Vec<&'static Skill>,
}

impl Team {
    fn apply_dmg(&mut self, dmg: Stats) -> Stats {
        self.stats -= self.damage.iter().fold(dmg, |a, &b| {
            //TODO deal with modifers
            //TODO deal with overload
            a - b.effect
        });
        self.stats
    }
    fn get_dmg(&mut self) -> Stats {
        self.damage.drain(0..).fold(Stats::default(), |a, b| {
            //TODO deal with modifers
            a + b.effect
        })
    }

    fn rest(&mut self) {
        self.stats.sp = self.max.sp;
        //reseting status effects to zero TODO check for overflow
        self.stats.status.iter_mut().for_each(|a| {
            if *a != 0 {
                *a -= 1
            }
        });
    }

    pub fn add_skill(&mut self, skill: &'static Skill) {
        if skill.deffense {
            self.deffense.push(skill);
        } else {
            self.damage.push(skill);
        }
        self.stats -= skill.cost;
    }
    pub fn del_skill(&mut self, skill: &'static Skill) {
        if let Some(index) = self.deffense.iter().position(|&x| x == skill) {
            self.deffense.remove(index);
        }
        if let Some(index) = self.damage.iter().position(|&x| x == skill) {
            self.damage.remove(index);
        }
        self.stats += skill.cost;
    }
    fn apply_cost(&mut self, cost: Stats) {
        self.stats -= cost;
    }
}

impl World {
    pub fn add_skill(&mut self, actor: usize, skill: usize) {
        let is_hostile = self.actors[actor].is_hostile;
        if let Some(sk) = self.actors[actor].progress_skill(skill) {
            if is_hostile {
                self.teams[IdxTeam::HOSTILE as usize].add_skill(sk);
            }
            else {
                self.teams[IdxTeam::PLAYER as usize].add_skill(sk);
            }
        }
    }
    fn del_skill() {
        todo!()
        //decrmenets combos on entity
        //removes skill from team
    }
    fn find_path() -> Stats {
        //will find a path and calcualte cost
        todo!()
    }
    fn apply_path() -> Stats {
        //will do a path for given entity
        todo!()
    }
}
