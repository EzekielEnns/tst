#![allow(dead_code)]
mod utils;
use std::{ops, f32};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

//TODO add skill and  animation static objects
//TODO fill out all other functions
//TODO add rendering functionality

#[derive(Default, Serialize, Deserialize, Clone, Copy)]
pub struct Stats {
    hp: f32,
    max_hp:f32,
    sp: f32,
    max_sp:f32,
    status: [i32; 10],
}

impl ops::AddAssign for Stats {
    fn add_assign(&mut self, rhs: Self) {
        self.hp += rhs.hp;
        self.sp += rhs.sp;
        for i in 0..=self.status.len() {
            self.status[i] += rhs.status[i]
        }
    }
}

impl ops::MulAssign for Stats {
    fn mul_assign(&mut self, rhs: Self) {
        self.hp *= rhs.hp;
        self.sp *= rhs.sp;
    }
}

impl ops::SubAssign for Stats {
    fn sub_assign(&mut self, rhs: Self) {
        //auto gaurd
        self.hp -= rhs.hp;
        if self.sp - rhs.sp < 0.0 {
            self.hp += self.sp - rhs.sp;
            self.sp = 0.0;
        } 
        else {
            self.sp -= rhs.sp;
        }
        for i in 0..=self.status.len() {
            self.status[i] -= rhs.status[i]
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Skill {
    cost: Stats,
    effect: Stats,
    modifer: bool,
    deffense: bool,
    name: &'static str,
    range: i32,
    #[serde(skip)]
    projectile:Projectile
}

//used for rendering/displaying to the screen
#[derive(Serialize, Deserialize)]
struct Glyph {
    value:char,
    color:[u8;4]
}

struct Combo {
    combo:Vec<&'static Skill>,
    index: usize
}
impl Combo {
    fn render() {} //for displaying to frontend
}

/* responsible for managing combat state of combat
 * */
struct Team<'a> {
    stats: Stats,
    skills: &'a[Skill;4],
    combos: &'a[Combo;4],
    deffenses: Vec<&'static Skill>,
    attacks: Vec<&'static Skill>,
}

impl<'a> Team<'a> {
    fn preview(){} //returns a preview of stats, for team
    fn enque_turn() {}
    fn end_turn() {}
    fn apply_dmg() {}
}


/* these hold all the data for a npc
 * they are incharge of interacting with other enties
 * incharge of generating data for combat state stored in team
 * */
struct Entity {
    glyph: Glyph,
    stats:Stats,
    items:Vec<Item>,
    skills:Vec<&'static Skill>,
    combos:Vec<&'static Combo>,
    eqpt_skills:[&'static Skill;4],
    eqpt_combos:[&'static Combo;4],
    //TODO add dialog
}

impl Entity {
    fn render_items(){}
    fn render_combos(){}
    fn render_skills(){}
    fn render_eqpt(){}
}


struct Tile {
    glyph:Glyph,
    collision:bool
}

struct Item { 
   name: &'static str,
   modifyer: Stats,
   consumable:bool,
}

/* these are structs used to define a behabior of a projectile,
 * the life time, current step in animation...etc,
 * these aso contain a index to a gloabl array of animation functions that would be 
 * called by the world
 * */
#[derive(Default)]
struct Projectile {
    step: i32,
    life: i32,
    index: i32,
}
impl Projectile {
    fn age(){}
    fn on_hit(&self, _target:&Entity){}
    fn render(){}
}

struct Pos {
    x:i32,
    y:i32
}

//these are the types used in that static pointer array
type Animation<'a>= fn(i32,&'a mut Pos);

/* responsible for postions, 
 * storing game sate/like combat
 * */
struct World<'a>{
    //TODO check if muts are needed here
    entites:Vec<(&'a mut Entity,Pos)>,
    tiles:Vec<(&'a Tile,Pos)>,
    items:Vec<(&'a Item,Pos)>,
    t_player:Option<Team<'a>>,
    t_enemy:Option<Team<'a>>,
    projectiles:Vec<(&'a mut Projectile, Pos)>,
    destinations:Vec<(usize,Pos)>,
    v_width:i32,
    v_height:i32,
}

impl<'a> World<'a> {
    fn step() {} //this represents a change in the simulation
                 //so a action would happen
                 //this would return a renderable secene 
                 //and the frontend would animate to this point
                 //i.e. move_to(), sim = step(), renderloop(=> animate to sim)
                 //think i frames form ds, the input is read first and the animation comes second
    fn find_path(){}
    fn progress(){} //progress's all movments/destinations
    fn render() {} //outputs the visable render section of world
    fn move_to(){} //add postion to destinations
    fn get_area(){} //gets entites within an area
    fn in_range(){} //gets entites within an area
    fn do_skill(){} //checks if skill is in range&'a mut 
}

pub fn get_stats(){}
pub fn get_preview(){}
pub fn get_skills(){}
pub fn get_combos(){}


#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, simulation!");
}
