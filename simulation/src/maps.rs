//TODO this file is for generating content to store in world
use crate::{entities::{Tile, Item, Actor}, render::RenderValue, stats::Stats, world::{Pos, get_pos, get_index}};


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
    for i in 0..(dim.y*dim.x) {
        //check if on dimentions
        let pos = get_pos(*dim, i as usize);
        if pos.y == 0 || pos.y == dim.y {
            tiles[i] = top.clone();
        }
        else if pos.x == 0 || pos.x == dim.x {
            tiles[i] = side.clone();
        }
        else {
            tiles[i] = floor.clone();
        }
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
    let mut center = Pos{ x:dim.x/2,y:dim.y/2};
    actors[get_index(&dim,&center)] = Some(player);

    let coin = Item {
        render_value: RenderValue {
            text: b'$',
            color: 25,
        },
        name: "coin",
        modifyer: Stats::default(),
        consumable: false,
    };
    center.y += 1; 
    items[get_index(&dim, &center)] = Some(coin);
}
