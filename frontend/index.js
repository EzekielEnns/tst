import * as sim from "./builds/wasm32-unknown-unknown/debug/simulation.wasm"
import bencode from 'bencode'
import { getLayer, init, render } from "./src/lib.js"
import { addLayer } from "./src/lib.js";
import font from "/monogram.ttf"

var/** @type {number} */ ptr = sim.get_buffs(),
   /** @type {number} */ len = sim.get_len(),
   /** @type {number} */ skLen = sim.render_skills(),
   /** @type {number} */ skPtr = sim.get_len_skills(),
   /** @type {number} */ stPtr = sim.render_stats(),
   /** @type {number} */ stLen = sim.get_len_stats(),
   /** @type {HTMLCanvasElement} */ cnv = document.getElementById("canvas")
/** @type {WebAssembly.Memory} */
var memeory = sim.memory;
var pull_data = true;

/**
 * @typedef {Object} RenderBuffers
 * @property {Uint8Array} textures
 * @property {Uint8Array} colors
 * @property {Uint8Array} locations
 * @property {number} len
 * @typedef {Object} RenderDataRaw
 * @property {RenderBuffers} actors
 * @property {RenderBuffers} items
 * @property {RenderBuffers} tiles
 *
 * @returns {RenderDataRaw} rd
 */
function getRenderData() {
    ptr = sim.get_buffs(ptr, len)
    let buff = new Uint8Array(memeory.buffer, ptr, sim.get_len())
    return bencode.decode(buff)
}
/**
 * @typedef {Object} Stat
 * @property {string} hp
 * @property {string} sp
 * @typedef {Object} Skill
 * @property {Stat} cost
 * @property {Stat} effect
 * @property {string} name
 *
 * @returns {Array<Skill|undefined>}
 */
function getSkillData() {
    skPtr = sim.render_skills(skPtr, skLen)
    skLen = sim.get_len_skills()
    let buff = new Uint8Array(memeory.buffer, skPtr, skLen)

    /** @type {Array<any>}*/
    let data = bencode.decode(buff)
    const dt = new TextDecoder();
    for (let i = 0; i < data.length; i++) {
        let raw = data[i][0]
        if (raw) {
            ["cost", "effect"].forEach((e) => {
                raw[e].hp = dt.decode(raw[e].hp)
                raw[e].sp = dt.decode(raw[e].sp)
            })
            raw.name = dt.decode(raw.name)
        }
        data[i] = raw
    }
    return data
}

/**
 *
 * @returns {Array<Stat>}
 */
function getStatsData() {
    stPtr = sim.render_stats(stPtr, stLen);
    stLen = sim.get_len_stats();
    let buff = new Uint8Array(memeory.buffer, stPtr, stLen);
    let data = bencode.decode(buff)
    const dt = new TextDecoder();
    for (let i = 0; i < data.length; i++) {
        let raw = data[i]
        if (raw) {
            raw.hp = dt.decode(raw.hp)
            raw.sp = dt.decode(raw.sp)
        }
    }
    return data
}

/* rendering combat/interfacing 
 *  update stats/pull stats
 *      div class stat
 *  update skills/skill buttons....
 *
 *
 *
 *  so combat will be selfcontained,
 *  on combat start we will trigger the get combat info function,
 *
 *  this will call the getStats() function
 *  that updates 
 *
 *
 */

function getStats(){
    //sets all stats to visible
    //pulls stats
    //update par widths
    for (const i of document.querySelectorAll('.stat') ) {
        //TODO display all stats/go through class list
        //add unhidden stuff tooo/remove hide class
        i.classList.remove('hide')
    }
}

function getSkills(){
    //updates text notes inside buttons/display
    //update skill array (global to file)
}

function combat_init(){
    getStats();
    getSkills()
}

await init(cnv, font)
//map layer
addLayer({
    params: {
        columns: 10,
        rows: 10,
        start: { x: -1, y: 1 },
        end: { x: 1, y: -1 },
        noFill: false
    }
})
//things layer
addLayer({
    params: {
        columns: 10,
        rows: 10,
        start: { x: -1, y: 1 },
        end: { x: 1, y: -1 },
        noFill: true
    }
}, 3) //TODO add a way to add quads and remove quads

function renderLoop() {
    if (pull_data) {
        let rd = getRenderData();
        let map = getLayer(0);
        let ply = getLayer(1);
        for (let i = 0; i < rd.tiles.len; i++) {
            map.setQuadTex(i, String.fromCharCode(rd.tiles.textures[i]))
        }
        for (let i = 0; i < rd.actors.len; i++) {
            ply.setQuad(i, ply.getCell(rd.actors.locations[i]));
            ply.setQuadTex(i,
                String.fromCharCode(rd.actors.textures[i]))
        }
        let offset = rd.actors.len;
        for (let j = offset; j < rd.items.len + offset; j++) {
            let i = j - offset
            ply.setQuad(j, ply.getCell(rd.items.locations[i]));
            ply.setQuadTex(j,
                String.fromCharCode(rd.items.textures[i]))
        }
        //clearing unused cells
        for (let i = offset + rd.items.len; i < ply.getLen(); i++) {
            ply.setQuadTex(i, ' ');
        }
        pull_data = false;
    }
    render()
    requestAnimationFrame(renderLoop)
}

for (const i of document.querySelector('#btnHolder').children) {
    i.addEventListener("click",()=>{
        passInput(i.id)
    })
}



//TODO add key up and down so they can hold
window.addEventListener("keydown", (e) => {
    passInput(e.key)
})


function passInput(key){
    let is_combat = exploration(key);
    if (is_combat) {
        //check if combet is running
        combat_init();
        /*
         * write a promise maybe to wait for player turn to be procssed 
         *      1. get moves from player
         *      2. update stats, on adding a turn
         *      3. end turn
         *      4. update stats // should also check for win (zero hp)
         *      5. do enemy turn
         */
    }

}

function exploration(key){
    let dir
    switch (key) {
        case "w":
            dir = 'u'
            pull_data = true;
            break;
        case "a":
            dir = 'l'
            pull_data = true;
            break;
        case "s":
            dir = 'd'
            pull_data = true;
            break;
        case "d":
            dir = 'r'
            pull_data = true;
            break;
    }
    return !sim.move_pc(dir?.charCodeAt())
}

//for combat
function getKeyIndex(key) {
    switch (key) {
        case "w":
            return 0
        case "a":
            return 1
        case "s":
            return 2
        case "d":
            return 3
    }
}

//https://developer.mozilla.org/en-US/docs/Web/API/window/requestAnimationFrame
requestAnimationFrame(renderLoop)
