
use serde::{Serialize, Deserialize};

use crate::entities::{Entity, Actor};
use crate::world::{Explore, Pos, MobileEntity};
use crate::RenderData;

pub fn generate_tiles(_target:&mut Explore) {

}

pub fn populate_room(_target:&mut Explore) {
    //let test = GENFN[ROOMS[0][0].0 as usize]();
}

fn gen_rat()-> Box<dyn Entity> {
    Box::new(Actor{
        glyph: RenderData{
            value:b'S',
            color:0xFFFF,
            alpha:255,
        }
    })
}
#[derive(Clone, Copy)]
enum G { Rat=0, }
type GEntity = fn ()->Box<dyn Entity>;

static GENFN: [GEntity;1] = [gen_rat];
static mut ROOMS:[[(G,u8);1];1] = [
    [(G::Rat,1)], //one rat in room 0 randomly
];

//storing map state outside of actual placement 
unsafe fn load_value(enq:&mut Explore,_p:Pos,_room_id:usize,_entity_id:usize) {
    let e = GENFN[ROOMS[_room_id][_entity_id].0 as usize]();
    //TODO add a Entity trait enum for weather a 
    //entity is mobile or not
    enq.me.push(MobileEntity{
        entity: e,
        destination: None,
        location: _p,
    });
}
