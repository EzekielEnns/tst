#![allow(dead_code)]
mod utils;
use std::{ops, f32};
use serde::{Deserialize, Serialize};

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
    //TODO add projectile
}

/*
 * what if for skills and combos we just use srede and Serialize the data?
 *      we can Deserialize the data when we want to read it derectly off the 
 *      alloc wasm memeory
 *
 *      okay i think im gonna skip wasm bind gen and instead use a serilization 
 *      and deserilation method for this stuff, and then type cast the decoded stuff via 
 *      jsdoc or a type deff file
 *
 *
 *      i can use the following forms of serilization
 *          cbor
 *          https://github.com/paroga/cbor-js
 *          messagePAck
 *          https://serde.rs/
 *
 *
 *      so the projecess would be: heres a pointer/stream of data 
 *          in js get_skills() -> const *a
 *          and then you would alloc that using types arrays
 *              annnndddd thennnnn 
 *                  you would convert it into a type/value
 *
 *      note to self
 *          i might want to just try again with bindgen.....
 *          
 *          but the main issue here is that getting access to the map data,
 *          and dealing with rendering the output properly is just really intense 
 *
 *          the best way to get the memory out it via just throwing it into pointer/
 *          static consts, so there will always be a upper limit when the program starts....
 *          thats a tad annoying but its not to bad
 *
 *
 *          i could use a process of chunking?
 *          where i take the values in one at a time and build a local array 
 *          that is on the heap
 *
 *
 *          oooookay this is actually really goood:
 *          https://radu-matei.com/blog/practical-guide-to-wasm-memory/#passing-arrays-to-rust-webassembly-modules
 *          neeed to read this
 *
 *          short and sweeat is that you can make a alloc and malloc inside wasm
 *          and use them to pass arrays over to eitehr side
 *
 *          we dont need to pass arrays into the machine we just need to pass data 
 *          out, so the nice thing here is when we our glyphs we would access the 3 pointers
 *                  get_colors: u32
 *                  get_char:   u8
 *                  get_alpha:  u8
 *          
 *          what i could do for getting skills is i could allocate a bin rep of a 
 *          skill, via proto buf or something like that,
 *              so:
 *                  js -> get_skills()
 *                        calls a alloc() for skill
 *                  rust -> alloc for binary data,
 *                       -> put bin data in skills pointer and free it 
 *                  js -> calls a delloc() once done, 
 *                          or requests new skill, in that spot
 *
 *              skill struct would be different 
 *                  
 *
 *
 *          i could do protobuf?
 *              i would love to use protobuffs
 *               https://github.com/tokio-rs/prost
 *               use buff cli and generate types
 *
 *
 */

struct Combo {
    combo:Vec<&'static Skill>,
    index: usize
}

//used for rendering/displaying to the screen
#[derive(Serialize, Deserialize,Clone, Copy)]
struct Glyph {
    value:u8,
    color:u32,
    alpha:u8,
}
//TODO add operator overload when array?

trait Entity {
    fn render(&self) -> Glyph;
    //would be used somehow with a render/animtaion function
    fn get_mut(&mut self) -> &mut Glyph;
}


/* these hold all the data for a npc
 * they are incharge of interacting with other enties
 * incharge of generating data for combat state stored in team
 * */
struct Actor { glyph: Glyph }
impl Entity for Actor{
    fn render(&self) -> Glyph { self.glyph }
    fn get_mut(&mut self) -> &mut Glyph {&mut self.glyph}
}
impl Actor { }

struct Tile { glyph:Glyph, collision:bool }
impl Entity for Tile {
    fn get_mut(&mut self) -> &mut Glyph { &mut self.glyph}
    fn render(&self) -> Glyph { self.glyph }
}
struct Item { 
   glyph: Glyph,
   name: &'static str,
   modifyer: Stats,
   consumable:bool,
}
impl Entity for Item {
    fn get_mut(&mut self) -> &mut Glyph { &mut self.glyph}
    fn render(&self) -> Glyph { self.glyph }
}

/* these are structs used to define a behabior of a projectile,
 * the life time, current step in animation...etc,
 * these aso contain a index to a gloabl array of animation functions that would be 
 * called by the world
#[derive(Default)]
struct Projectile {
    step: i32,
    life: i32,
    index: i32,
    spread: i32,
    //FIXME add area
}
impl Projectile {
    //TODO i could use recursion to update the life time of these files
    pub fn age(&mut self)->bool {
       self.life -= 1;
       return self.life <= 0
    }
    fn on_hit(&self, _target:&Entity){}
    fn render(){}
}
* */

#[derive(PartialEq, Eq)]
struct Pos {
    //TODO enforce being topleft coords 0,0 -> width,height
    //Stop roll over 
    x:i32, //column
    y:i32  //row
}

impl Pos {
    //TODO add a step
    pub fn approch<'a>(&mut self, dest: &'a Pos) {
        let diff: Pos = Pos {x: dest.x - self.x, y:dest.y - self.y};
        self.y += if diff.y > 0 {1} else if diff.y < 0 {-1} else {diff.y};
        self.x += if diff.x > 0 {1} else if diff.x < 0 {-1} else {diff.x};
    }
}

struct MobileEntity {
    entity: Box<dyn Entity>,
    location: Pos,
    destination: Option<Pos>,
}
//cc likes to seprate things to there own stuff 
//items he seprates as well
//
//seprate things out 
//  do one thing and one thing only!!
//
//i.e. i could make items seprate
//when a collison happens they pick it up
//
//tiles dont move, and doors could just be a 
//tiles dont need a dest they dont move 
//
//
//you might feel lag!!!



/* responsible for postions, 
 * storing game sate/like combat
 *
 *
 * world is my combat as well,
 *      how cc dose it::
     *      setup a world base,
     *      this is your main map
     *      then you have certain variousions
     *      changing it 
     *
     *
     *      one combat world, one exploration world
     *      swap based on that....
     *
     *  two structs a combat struct and a exploration struct
     *
     *  they both impl the trait World which has step and 
     *  stuff like that, so copy over a the reffrence to the 
     *  renderable world..... based on the current mode.....
     *
     *  we can also save this world data in a display struct
     *  so like struct Save {exploration, combat};
     *  that can then be serilzed using bencoding
     *
     *  ------
     *  runtime is very important in games
     *  ------
     *  ? how do i render/generate the world for the player
     *
     *  so there is static and procedal generation.....
     *  gneration of world happens at the start of the game only
     *  your loading a saved state of that world
     *
     *  generation-- one time one off event
     *
     *  when player walks through a door, 
     *  i guess im asking for spawning.........
     *  so this is a tile based game::
     *      tiles are difinitive, x and y coord,
     *
     *  walk into a room the trigger will be the entry
     *  the items in the room will spawn items,tiles 
     *  will have a flag that belongs to room nuber one 
     *  
     *  rember to focus on numbers not objects,
     *  so you could assing objects as numbers to rooms
     *  and ranomly spawn items and enemeies
     *
     *  set a room to spawn items and enemeies in room
     *  the moment you go out the door, 
     *          a visted room should not have its 
     *          objects changed/enemies
     *          once visted should not change
     *  how do i design a struct for that, 
     *  virtual struct called room, store this this and 
     *  this tile, make up the room,
     *  this one eneter trigger spawn stuff, then modify 
     *  whats in the room and store that....
     *
     *  cull the objects that need to stay.....
     *  so for const world, store the eneties in the mobile
     *  array and they can live else where, flag 
     *  non culled objects and render/loop throught them
     *
     *  need a buffer for view/render
     *
     *
     *  so are think about how you map is made/load that map
     *  into the world/visible range
     *  do a double buffer, where monsters loose intrest 
*  and then move to static/map placement
*  they go back to numbers
*
*
*  baseclass for entites that can be randomized...
*
*  so you have a static world/thing 
*  bunch of float, or string
*
*
*  the projectiles will just switch modes, when hit/ on collison
*
*
*  when damage happens do the damage based on the classes that 
*  deal with damage, i.e. skill dose dmg but not projectiles
*  only deal with moving postion, and animation and collison
     *
     *
 * */
struct World<'a>{
    //TODO check if muts are needed here
    //
    //FIXME i dont like haveing all this extra stuff
    entites:Vec<&'a mut MobileEntity>,
    //TODO refresh on how to change types

    //TODO make a view struct i.e. u8,u8,u32

    //dimentions for world view and overall stuff
    width:usize,
    height:usize,
}
const PLAYER_ENTITY_INDEX:usize = 0;
impl<'a> World<'a> {
    fn step (&mut self) {
        /*
         * in game dev anything that dose not fall into 
         * players view, they get turned off 
         *
         * in sim games they use numbers
         *      empire a & b are fighting
         *      those ships/missles
         *          they exist as a number
         *          basic struts.... numbers will play
         *          while they fight, and get a battle outcome
         *  obj culling -> its useally reffered to visally 
         *  but it works in code 
         *
         *  when the players, rng whats in the room,
         *  based on inputs..........
         *
         *  check if rust manages the functions in 
         *  look up rust garbage disposal
         *
         *  CCNOTES 
         *  collison based system/hit based 
         *      (number based on chance)
         *
         *  ? how do i manage combat system
         *  make sure you have, a max value 
         *  @ inside the combat
         *
         *   
         */
        for _i in 0..self.entites.len() {
            let mut _e = &mut self.entites[_i];
            if let Some(dest) = &_e.destination {
                if _e.location != *dest {
                    _e.location.approch(dest);
                }
            }
        }
    }

    fn get_view() {

    }
}

pub fn get_preview(){}
pub fn get_skills(){}
pub fn get_combos(){}
pub fn step(){}

static TILES: [Tile;3] = [
    Tile{
        glyph:Glyph{value:b'.',color:0xb9a8a4,alpha:1},
        collision: false
    },
    Tile{
        glyph:Glyph{value:b'|',color:0x0,alpha:1},
        collision: true
    },
    Tile{
        glyph:Glyph{value:b'-',color:0x0 , alpha:1}, //TODO make consts for color values
        collision: true
    }
];

//TODO maybe make a struct just for display purposeses?
//so these are good, this dosent harm or hinder anything
//rember the differnce between static and cosntant, 
//the base skill should be static, they wont change
static SKILLS: [Skill;1] = [
    Skill {
        cost:  Stats{hp:0.0,sp:0.0,status:[0]},       
        name: "test",
        range: 1,
        effect: Stats{hp:0.0,sp:0.0,status:[0]},
        deffense:false,
        modifer:false,
    }
];


/*
 * lets define our rendering output/pipeline 
 * we will output a u8 buffer and a u32 buffer
 * i.e. let color:u32 = 0XFFFF
 *      let value:u8  = b'h'
 */
//
// pub unsafe fn greet() {
//     alert("Hello, simulation!");
// }

#[no_mangle]
pub unsafe extern "C" fn hello_world() -> *mut u8 {
    let mut buf = bendy::serde::to_bytes(&SKILLS[0]).unwrap();
    let ptr = buf.as_mut_ptr();

    std::mem::forget(buf);
    return ptr;
}

#[no_mangle]
pub unsafe extern "C" fn len() -> usize {
    let buf = bendy::serde::to_bytes(&SKILLS[0]).unwrap();
    return buf.len();
}

/* using 
//std::mem::drop we can deollocate the vec
//the nice thing here is that we only have to allocate a 
//struct for displaying the current skills  once per combat encournter 
//or per equiptment 
//ooooooooh i can recylce!!!!!!!!!
//
//take the pointer in ->>> you can resize the vec too so that works out as well
//do a Vec::from_raw_parts() 
*/
