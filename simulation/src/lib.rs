#![allow(dead_code)]


use maps::{Generate, first_test_world};
use once_cell::sync::Lazy;
use world::{World, move_player};
mod utils;
mod skills; 
mod world;
mod entities;
mod stats;
mod render;
mod maps;

static mut WORLD: Lazy<World> = Lazy::new(|| {
    let mut wl = World::new(10,10);
    wl.generate(first_test_world);
    wl
});

#[no_mangle]
pub unsafe extern "C" fn move_pc(new: usize) {
    //moves player to specific spot
    //sets player_index after doing move
    move_player(&mut WORLD,new);
}

#[no_mangle]
pub unsafe extern "C" fn get_buffs()-> *mut u8 {
    WORLD.render_alloc()
}
#[no_mangle]
pub unsafe extern "C" fn update() {
    WORLD.render_actors();
    WORLD.render_items();
    WORLD.render_tiles();
}

