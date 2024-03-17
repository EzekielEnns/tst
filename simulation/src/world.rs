use std::borrow::BorrowMut;

use crate::{
    entities::{Actor, Entity, Item, Tile},
    maps::Generate,
    render::{RenderBuffers, RenderData, RenderValue},
    skills::Team,
};
//TODO add animated entity
#[derive(Clone, Copy)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

//gets the postion based on a index
pub fn get_pos(b: Pos, i: usize) -> Pos {
    let column = i % b.x;
    let row = (i - (i % b.x)) / b.x;
    Pos {
        x: column,
        y: if row > b.y { row % b.y } else { row },
    }
}

//gets the index based on a postion
pub fn get_index(b: &Pos, p: &Pos) -> usize {
    p.y * b.y + p.x
}

pub fn move_player(wld: &mut World, new: usize) -> bool {
    //TODO impl https://doc.rust-lang.org/std/ops/trait.Index.html
    wld.move_actor(IdxActor::PLAYER as usize, new)
}

/*
 * mvp needs to have movment and combat
 * there dosnet need to be any items, things can be hard coded and worked in
 * also theres world generation....
 */
pub struct World {
    pub actors: Vec<Actor>,
    pub actor_locations: Vec<usize>,
    pub items: Vec<Item>,
    pub item_locations: Vec<usize>,
    pub tiles: Vec<Tile>,
    pub dim: Pos,
    pub teams: Vec<Team>,
    pub buff_lens: [usize; 3],
}

#[repr(usize)]
pub enum IdxBfLen {
    RENDER = 0,
    STATS,
    SKILLS,
}
#[repr(usize)]
pub enum IdxActor {
    PLAYER = 0,
}
#[repr(usize)]
pub enum IdxTeam {
    PLAYER = 0,
    HOSTILE,
}

enum RenderType {
    ACTORS = 0,
    ITEMS,
    TILES,
}
impl World {
    pub fn new(width: usize, height: usize) -> World {
        let len = width * height;
        World {
            dim: Pos {
                y: height,
                x: width,
            },
            actors: Vec::<Actor>::with_capacity(len),
            actor_locations: Vec::<usize>::with_capacity(len),
            items: Vec::<Item>::with_capacity(len),
            item_locations: Vec::<usize>::with_capacity(len),
            tiles: Vec::<Tile>::with_capacity(len),
            buff_lens: [0; 3],
            teams: Vec::<Team>::with_capacity(2),
        }
    }
    fn len(&self) -> usize {
        self.dim.x * self.dim.y
    }

    //this needs some testing
    fn move_actor(&mut self, index: usize, new: usize) -> bool {
        if self.teams.len() > 1 {
            return false;
        }
        if self.tiles[new].collision {
            return false;
        } else if let Some(collision) = self.actor_locations.iter().position(|&v| v == new) {
            let actor = &self.actors[collision];
            if actor.is_hostile {
                for _ in 0..2 {
                    self.teams.push(Team::default());
                }
                for i in self.actors.iter() {
                    if i.is_hostile {
                        i.add_to_team(&mut self.teams[IdxTeam::HOSTILE as usize]);
                    }
                    else {
                        i.add_to_team(&mut self.teams[IdxTeam::PLAYER as usize]);
                    }
                }
                return false;
            }
        } else if let Some(collision) = self.item_locations.iter().position(|&v| v == new) {
            let actor = &mut self.actors[index];
            actor.items.push(self.items.remove(collision));
            self.item_locations.remove(collision);
        }
        self.actor_locations[index] = new;
        return true;
    }

    pub unsafe fn pack_buffer(&mut self, old: *mut u8, size: usize) -> *mut u8 {
        if old != std::ptr::null_mut() && size != 0 {
            std::mem::drop(Vec::from_raw_parts(old, size, size));
        }
        let mut items = RenderBuffers::new();
        items.len = self.items.len();
        for (i, elm) in self.items.iter().enumerate() {
            let render = elm.render_value;
            items.colors.push(render.color);
            items.textures.push(render.text);
            items.locations.push(self.item_locations[i] as u8);
        }
        let mut tiles = RenderBuffers::new();
        tiles.len = self.tiles.len();
        for (i, elm) in self.tiles.iter().enumerate() {
            let render = elm.render_value;
            tiles.colors.push(render.color);
            tiles.textures.push(render.text);
            tiles.locations.push(i as u8);
        }
        let mut actors = RenderBuffers::new();
        actors.len = self.actors.len();
        for (i, elm) in self.actors.iter().enumerate() {
            let render = elm.render_value;
            actors.colors.push(render.color);
            actors.textures.push(render.text);
            actors.locations.push(self.actor_locations[i] as u8);
        }

        let data = RenderData {
            actors,
            items,
            tiles,
        };
        let mut buf = bendy::serde::to_bytes(&data).unwrap();
        self.buff_lens[IdxBfLen::RENDER as usize] = buf.len();
        let ptr = buf.as_mut_ptr();
        std::mem::forget(buf);
        return ptr;
    }

    pub unsafe fn pack_skill_buff(&mut self, old: *mut u8, size: usize) -> *mut u8 {
        if old != std::ptr::null_mut() && size != 0 {
            std::mem::drop(Vec::from_raw_parts(old, size, size));
        }
        let (ptr, len) = self.actors[IdxActor::PLAYER as usize].render_active_skills();
        self.buff_lens[IdxBfLen::SKILLS as usize] = len;
        ptr
    }
}

impl Generate for World {
    fn generate(&mut self, func: fn(&mut World)) {
        func(self);
    }
}
