//TODO this file is for generating content to store in world

use std::collections::VecDeque;

use crate::{entities::{Tile, Item, Actor}, render::RenderValue, stats::Stats, world::Pos};


pub trait Generate {
    fn generate(&mut self, func: fn(dim: &Pos,actors: &mut [Option<Actor>],items:&mut [Option<Item>],tiles:&mut [Tile]));
}

pub fn first_test_world(dim: &Pos,actors: &mut [Option<Actor>],items:&mut [Option<Item>],tiles:&mut [Tile]){
    //TODO fill tiles with . and | or - on the tops of the world 
    let side = Tile {
        collision: true,
        render_value: RenderValue {
            text: b'|',
            color: 255,
        },
    };
    let top = Tile {
        collision: true,
        render_value: RenderValue {
            text: b'-',
            color: 255,
        },
    };
    let floor = Tile {
        collision: true,
        render_value: RenderValue {
            text: b'.',
            color: 255/2,
        },
    };
    //itterate through array see if index is in spot
    for i in 0..tiles.len() {
        //check if on dimentions
        tiles[i] = floor.clone();
    }
    
    //TODO add a player
    let player = Actor {
        render_value: RenderValue {
            color: 80,
            text: b'@',
        },
        items: Vec::new(),
        is_hostile: false,
    };
    //TODO add a coin
    let coin = Item {
        render_value: RenderValue {
            text: b'$',
            color: 25,
        },
        name: "coin",
        modifyer: Stats::default(),
        consumable: false,
    };
}
/*
use serde::{Serialize, Deserialize};

use crate::entities::{Entity, Actor, Item};
use crate::world::{Pos};
use crate::RenderData;

pub fn generate_tiles(_target:&mut Explore) {

}

pub fn populate_room(_target:&mut Explore) {
    //let test = GENFN[ROOMS[0][0].0 as usize]();
}

fn gen_rat(){
    // Actor{
    //     glyph: RenderData{
    //         value:b'S',
    //         color:0xFFFF,
    //         alpha:255,
    //     }
    // }
}
// #[derive(Clone, Copy)]
// enum G { Rat=0, }
// type GActor = fn ()->Actor;
// type GItem = fn ()->Item;
//
// static GEN_ACTOR_FN: [GActor;1] = [gen_rat];
//
// struct Room {
//     actors: u8,
//     a_fn: Vec<G>,
//     items: u8,
//     i_fn: Vec<G>,
//     tiles: u8,
//     t_fn: Vec<G>,
// }
//
// static mut ROOMS:[[(G,u8);1];1] = [
//     [(G::Rat,1)], 
// ];
//
// //storing map state outside of actual placement 
// unsafe fn load_value(enq:&mut Explore,_p:Pos,_room_id:usize,_entity_id:usize) {
//     let e = GENFN[ROOMS[_room_id][_entity_id].0 as usize]();
//     //TODO add a Entity trait enum for weather a 
//     //entity is mobile or not
//     enq.me.push(MobileEntity{
//         entity: e,
//         destination: None,
//         location: _p,
//     });
// }
*/
