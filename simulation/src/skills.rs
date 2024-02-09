use serde::{Deserialize, Serialize};
use crate::{stats::Stats, world::World};

#[derive(PartialEq)]
pub struct Skill {
    pub cost: Stats,
    pub effect: Stats,
    pub modifer: bool,
    pub deffense: bool,
    pub name: &'static str,
    pub range: i32,
}
/* TODO add for skills
impl Serialize for Buffer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            let mut state = serializer.serialize_struct("Buffer",2)?;
            state.serialize_field("ptr", &format!("{:p}",self.ptr))?;
            state.end()
        }
}
*/


pub struct Combo {
    //None will be used as a single that they are done
    pub combo:Vec<Option<&'static Skill>>,
    pub index: usize
}

pub struct Team {
    pub stats: Stats,
    pub max: Stats,

    pub damage: Vec<&'static Skill>,
    pub deffense: Vec<&'static Skill>,
}

impl Team {
    fn apply_dmg(&mut self,dmg:Stats)-> Stats {
        self.stats -= self.damage.iter().fold(dmg,|a,&b|{
            //TODO deal with modifers
            //TODO deal with overload
            a-b.effect
        });
        self.stats
    }
    fn get_dmg(&mut self)-> Stats {
        self.damage.drain(0..).fold(Stats::default(),|a,b|{
            //TODO deal with modifers
            a+b.effect
        })
    }

    fn rest(&mut self) {
        self.stats.sp = self.max.sp;
        //reseting status effects to zero TODO check for overflow
        self.stats.status.iter_mut().for_each(|a| { if *a != 0 { *a -= 1 } });
    }

    fn add_skill(&mut self, skill: &'static Skill){
        if skill.deffense {
            self.deffense.push(skill);
        }
        else {
            self.damage.push(skill);
        }
        self.stats -= skill.cost;
    }
    fn del_skill(&mut self, skill: &'static Skill){
        if let Some(index) = self.deffense.iter().position(|&x| x == skill) {
            self.deffense.remove(index);
        }
        if let Some(index) = self.damage.iter().position(|&x| x == skill) {
            self.damage.remove(index);
        }
        self.stats += skill.cost;
    }
    fn apply_cost(&mut self, cost:Stats) {
        self.stats -= cost;
    }
}

impl World {
    fn add_skill(){
        //increments combos on entity
        //applys movment cost 
        //adds skill to team
    }
    fn del_skill(){
        //decrmenets combos on entity
        //removes skill from team
    }
    fn find_path()->Stats{
        //will find a path and calcualte cost
        todo!()
    }
    fn apply_path()->Stats{
        //will do a path for given entity
        todo!()
    }
}
