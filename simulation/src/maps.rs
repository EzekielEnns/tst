//TODO this file is for generating content to store in world
use crate::{entities::{Tile, Item, Actor}, render::RenderValue, stats::Stats, world::{Pos, get_pos, get_index, World}};


pub trait Generate {
    fn generate(&mut self, func: fn(&mut World));
}

pub fn first_test_world(w: &mut World){
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
    for i in 0..(w.dim.x * w.dim.y) {
        let pos = get_pos(w.dim, i as usize);
        if pos.y == 0 || pos.y == w.dim.y {
            w.tiles.push(top);
        }
        else if pos.x == 0 || pos.x == w.dim.x {
            w.tiles.push(side);
        }
        else {
            w.tiles.push(floor);
        }
    }
    let player = Actor {
        render_value: RenderValue {
            color: 80,
            text: b'@',
        },
        items: Vec::new(),
        is_hostile: false,
    };
    let mut center = Pos{ x:w.dim.x/2,y:w.dim.y/2};
    let index = get_index(&w.dim,&center);
    w.actors.push(player);
    w.actor_locations.push(index);

    let coin = Item {
        render_value: RenderValue {
            text: b'$',
            color: 25,
        },
        name: "coin",
        modifyer: Stats::default(),
        consumable: false,
    };
    center.y +=1;
    let index = get_index(&w.dim,&center);
    w.items.push(coin);
    w.item_locations.push(index);

}
