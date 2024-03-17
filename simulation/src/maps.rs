//TODO this file is for generating content to store in world
use crate::{
    entities::{Actor, Item, Tile},
    render::RenderValue,
    skills::Combo,
    stats::Stats,
    world::{get_index, get_pos, Pos, World},
    SKILLS,
};

pub trait Generate {
    fn generate(&mut self, func: fn(&mut World));
}

pub fn first_test_world(w: &mut World) {
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
        collision: false,
        render_value: RenderValue {
            text: b'.',
            color: 255 / 2,
        },
    };
    for i in 0..(w.dim.x * w.dim.y) {
        let pos = get_pos(w.dim, i as usize);
        if pos.y == 0 || pos.y == w.dim.y - 1 {
            w.tiles.push(top);
        } else if pos.x == 0 || pos.x == w.dim.x - 1 {
            w.tiles.push(side);
        } else {
            w.tiles.push(floor);
        }
    }
    //these are base stats, since items are stats kinda
    let base_item = Item {
        render_value: RenderValue {
            text: b'$',
            color: 25,
        },
        name: "base",
        modifyer: Stats{ hp:100.0, sp:100.0, status:[0;1]}.to_owned(),
        consumable: false,
    };
    let player = Actor {
        combos: vec![Combo {
            index: 0,
            combo: vec![Some(&SKILLS[0]); 1],
        }],
        skills: vec![&SKILLS[0]],
        render_value: RenderValue {
            color: 80,
            text: b'@',
        },
        items: vec![base_item.clone()],
        is_hostile: false,
    };

    let mut center = Pos {
        x: w.dim.x / 2,
        y: w.dim.y / 2,
    };
    let index = get_index(&w.dim, &center);
    w.actors.push(player);
    w.actor_locations.push(index);

    let enemy = Actor {
        combos: vec![Combo {
            index: 0,
            combo: vec![Some(&SKILLS[0]); 1],
        }],
        is_hostile: true,
        skills: vec![&SKILLS[0]],
        render_value: RenderValue {
            color: 80,
            text: b'*',
        },
        items: vec![base_item.clone()],
    };
    w.actors.push(enemy);
    w.actor_locations.push(get_index(
        &w.dim,
        &Pos {
            x: 1,
            y: 1,
        },
    ));

    let coin = Item {
        render_value: RenderValue {
            text: b'$',
            color: 25,
        },
        name: "coin",
        modifyer: Stats::default().to_owned(),
        consumable: false,
    };
    center.y += 1;
    let index = get_index(&w.dim, &center);
    w.items.push(coin);
    w.item_locations.push(index);
}
