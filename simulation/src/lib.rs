#![allow(dead_code)]
/*
turns out for some reason you cant send a array of pointers in wasm to 
js......


this means my current render system dose not work,
and i will have to overcome this some how 
*/


use maps::{Generate, first_test_world};
use once_cell::sync::Lazy;
use skills::Skill;
use stats::Stats;
use world::{World, move_player, get_index, Pos, get_pos, IdxActor, IdxBfLen};
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

static SKILLS: [Skill;1] = [
    Skill {
        name:"DUMMY SKILL",
        cost: Stats {hp:0.0,sp:0.0,status:[0;1]},
        effect: Stats {hp:0.0,sp:0.0,status:[0;1]},
        deffense: false,
        reach: 0,
    }
];

#[no_mangle]
pub unsafe extern "C" fn move_pc(d:u8) -> bool {
    //TODO calculate new postion, based on shift with d
    //add pos to current player pos 
    //get index and send it to move command
    let mut ply = get_pos(WORLD.dim,WORLD.actor_locations[IdxActor::PLAYER as usize]);
    match d {
        b'u' => { if ply.y != 0 { ply.y -= 1; } }
        b'd' => { if ply.y != WORLD.dim.y { ply.y += 1; } }
        b'l' => { if ply.x != 0 { ply.x -= 1; } }
        b'r' => { if ply.x != WORLD.dim.x { ply.x += 1; } }
        _ => {}
    };
    let new = get_index(&WORLD.dim,&ply);
    move_player(&mut WORLD,new)
}

#[no_mangle]
pub unsafe extern "C" fn get_buffs(ptr: *mut u8, size: usize)-> *mut u8 {
    WORLD.pack_buffer(ptr,size)
}
#[no_mangle]
pub unsafe extern "C" fn get_len()-> usize {
    WORLD.buff_lens[IdxBfLen::RENDER as usize] 
}

//REMINDER, all external functions are player focused
#[no_mangle]
pub unsafe extern "C" fn turn(index:usize) {
    WORLD.add_skill(IdxActor::PLAYER as usize, index );
}
#[no_mangle]
pub unsafe extern "C" fn end_turn() { 
    //dose player attack, resets ai, dose ai attack, resets player
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn render_skills(ptr: *mut u8, size: usize)-> *mut u8 {
    WORLD.pack_skill_buff(ptr, size)
}
#[no_mangle]
pub unsafe extern "C" fn get_len_skills() -> usize {
    WORLD.buff_lens[IdxBfLen::SKILLS as usize]
}

#[no_mangle]
pub unsafe extern "C" fn render_stats() -> *mut u8{
    //returns teams stats in array 
    todo!()
}
#[no_mangle]
pub unsafe extern "C" fn get_len_stats() -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::move_pc;


    #[test]
    fn it_works() {
            unsafe {
                move_pc(b'u');
            }
            panic!()
    }
}
