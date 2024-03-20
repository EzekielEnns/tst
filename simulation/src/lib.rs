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
use world::{World, move_player, get_index, Pos, get_pos, IdxActor, IdxBfLen, IdxTeam};
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
        cost: Stats {hp:0.0,sp:5.0,status:[0;1]},
        effect: Stats {hp:5.0,sp:0.0,status:[0;1]},
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
    move_player(&mut WORLD,new);
    WORLD.teams.len() == 0
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
pub unsafe extern "C" fn end_turn() -> bool { 
    WORLD.apply_dmg(IdxActor::PLAYER as usize, 1);
    if WORLD.get_looser() != usize::MAX {
        return true;
    }
    WORLD.start_turn(1);
    WORLD.add_skill(1,0);
    WORLD.apply_dmg(1,IdxActor::PLAYER as usize);
    WORLD.start_turn(0);
    return false;
}

#[no_mangle]
pub unsafe extern "C" fn clean_combat()-> bool {
    let looser_team_index = WORLD.get_looser();
    WORLD.teams.clear();
    if looser_team_index == IdxTeam::HOSTILE as usize{
        WORLD.actors.remove(IdxActor::HOSTILE as usize);
        WORLD.actor_locations.remove(IdxActor::HOSTILE as usize);
    }
    else if looser_team_index == IdxTeam::PLAYER as usize {
        WORLD.actors.remove(IdxActor::PLAYER as usize);
        WORLD.actor_locations.remove(IdxActor::PLAYER as usize);
    }
    
    return looser_team_index != IdxTeam::PLAYER as usize;
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
pub unsafe extern "C" fn render_stats(ptr: *mut u8, size: usize) -> *mut u8{
    WORLD.pack_stats_buff(ptr,size)
}
#[no_mangle]
pub unsafe extern "C" fn get_len_stats() -> usize {
    WORLD.buff_lens[IdxBfLen::STATS as usize]
}

#[cfg(test)]
mod tests {

    use crate::{ WORLD, world::{get_index, Pos, move_player, IdxTeam}, clean_combat};


    #[test]
    fn it_works() {
            unsafe {
                move_player(&mut WORLD, get_index(&WORLD.dim, &Pos{x:1,y:1}));
                println!("{:?}{:?}",WORLD.actors[0].render_value,WORLD.actors[1].render_value);
                WORLD.teams[IdxTeam::HOSTILE as usize].stats.hp = 0.0;
                clean_combat();
                println!("{:?}{:?}",WORLD.actors[0].render_value,WORLD.actors.len());
                panic!()
            }
    }
}
