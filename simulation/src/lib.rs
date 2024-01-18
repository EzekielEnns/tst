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
    sp: f32,
    status:[i32;1]
    //FIXME add speed/enum
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

//TODO maybe remove serde here and make special deisplay objects or json?
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
#[derive(Serialize, Deserialize,Clone, Copy)]
struct Glyph {
    value:char,
    color:[f32;4]
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
    max_stats: Stats,
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
    //FIXME add area
}
impl Projectile {
    pub fn age(&mut self)->bool {
       self.life -= 1;
       return self.life <= 0
    }
    fn on_hit(&self, _target:&Entity){}
    fn render(){}
}

struct Pos {
    x:i32, //column
    y:i32  //row
}


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
    v_width:usize,
    v_height:usize,
}
const PLAYER_ENTITY_INDEX:usize = 0;
impl<'a> World<'a> {
    //this represents a change in the simulation
    //so a action would happen
    //this would return a renderable secene 
    //and the frontend would animate to this point
    //i.e. move_to(), sim = step(), renderloop(=> animate to sim)
    //think i frames form ds, the input is read first and the animation comes second
    
    //note other changes to the World happen
    //around this function, this is just stepping through
    //claims that were already made
    //so interactions can happen else where, and its processed
    //else where
    fn step (&mut self) {
        //Phase 1 running simulation
        //process destinations
        //use entity movemnt speed
        //FIXME add speed to stats

        //check for collision with projectiles
        //FIXME add area
        //kill projectiles VVVVVV
        if self.projectiles[0].0.age() {
            //kill
        }

        //Phase 2 creating rendered area
        //detemin area based on player location,
        //collecet tiles for that area, add them to the array (single d array)
        //add entites, projectile, items ontop/replace tiles
    }
    /* example fn
    fn find_path(){}
    fn progress(){} //progress's all movments/destinations
    fn render() {} //outputs the visable render section of world
    fn move_to(){} //add postion to destinations
    fn get_area(){} //gets entites within an area
    fn in_range(){} //gets entites within an area
    fn do_skill(){} //checks if skill is in range&'a mut 
    */
}

pub fn get_stats(){}
pub fn get_preview(){}
pub fn get_skills(){}
pub fn get_combos(){}
pub fn step(){}

static TILES: [Tile;3] = [
    Tile{
        glyph:Glyph{value:'.',color:[0.5,0.5,0.5,1.0]},
        collision: false
    },
    Tile{
        glyph:Glyph{value:'|',color:[0.5,0.5,0.5,1.0]},
        collision: true
    },
    Tile{
        glyph:Glyph{value:'-',color:[0.5,0.5,0.5,1.0]},
        collision: true
    }
];

//TODO maybe make a struct just for display purposeses?

type Animation= fn(i32,&mut Pos);
fn ani_example(_step:i32,_pos:&mut Pos) { }
static ANIMATIONS: [Animation;1] = [ ani_example ];

static SKILLS: [Skill;1] = [
    Skill {
        cost:  Stats{hp:0.0,sp:0.0,status:[0]},       
        name: "test",
        range: 1,
        effect: Stats{hp:0.0,sp:0.0,status:[0]},
        deffense:false,
        modifer:false,
        projectile: Projectile{
            step:0,
            life:0,
            index:0 //index in the ANIMATIONS const
        },
    }
];

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, simulation!");
}
