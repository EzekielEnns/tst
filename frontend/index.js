import * as sim from "./dist/wasm32-unknown-unknown/debug/simulation.wasm"
import bencode from 'bencode'
import {getLayer, init, render} from "./src/lib.js"
import { addLayer } from "./src/lib.js";

var/** @type {number} */ ptr = sim.get_buffs(),
   /** @type {number} */ len = sim.get_len(),
   /** @type {HTMLCanvasElement} */ cnv = document.getElementById("canvas")
/** @type {WebAssembly.Memory} */
var memeory = sim.memory;

console.log("hi")
console.log(memeory.buffer)
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
function getRenderData(){
    ptr = sim.get_buffs(ptr,len)
    let buff = new Uint8Array(memeory.buffer,ptr,sim.get_len())
    return bencode.decode(buff)
}

await init(cnv,"/monogram.ttf")
//map layer
addLayer({params:{
    columns:10,
    rows:10,
    start:{x:-1,y:1},
    end:{x:1,y:-1},
    noFill: false
}})
//things layer
addLayer({params:{
    columns:10,
    rows:10,
    start:{x:-1,y:1},
    end:{x:1,y:-1},
    noFill: true
}},2)
var pull_data = true;

function renderLoop() {
    if (pull_data) {
        let rd = getRenderData();
        console.log(rd)
        let map = getLayer(0);
        let ply = getLayer(1);
        console.log(ply)
        for (let i = 0; i < rd.tiles.len; i++){
            map.setQuadTex(i,String.fromCharCode(rd.tiles.textures[i]))
        }
        for (let i =0; i<rd.actors.len; i++){
           ply.setQuad(i,ply.getCell(rd.actors.locations[i]));
           ply.setQuadTex(i,
               String.fromCharCode(rd.actors.textures[i])) 
        }
        let offset = rd.actors.len;
        for (let j =offset; j<rd.items.len+offset; j++){
           let i = j-offset
           ply.setQuad(j,ply.getCell(rd.items.locations[i]));
           ply.setQuadTex(j,
               String.fromCharCode(rd.items.textures[i])) 
        }
        pull_data = false;
    }
    render()
    requestAnimationFrame(renderLoop)
}

window.addEventListener("keydown",(e)=>{
    switch (e.key) {
        case "w":
            console.log(sim.move_pc('u'.charCodeAt()))
            pull_data = true;
            break;
        case "a":
            sim.move_pc('l'.charCodeAt())
            pull_data = true;
            break;
        case "s":
            sim.move_pc('d'.charCodeAt())
            pull_data = true;
            break;
        case "d":
            sim.move_pc('r'.charCodeAt())
            pull_data = true;
            break;
    }
})



//https://developer.mozilla.org/en-US/docs/Web/API/window/requestAnimationFrame
requestAnimationFrame(renderLoop)

//TODO bind movment  

//TODO make render loop like in testing for glyph lib
//getRenderData
//in loop approche render data based on factor
//repeat

