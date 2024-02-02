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
    noFill: false
}})
var pull_data = true;
function renderLoop() {
    if (pull_data) {
        let rd = getRenderData();
        console.log(rd)
        let map = getLayer(0);
        let ply = getLayer(1);
        for (let i = 0; i < rd.tiles.len; i++){
            map.setQuadTex(i,String.fromCharCode(rd.tiles.textures[i]))
        }
        for (let i =0; i<rd.actors.len; i++){
           ply.setQuadTex(rd.actors.locations[i],
               String.fromCharCode(rd.actors.textures[i])) 
        }
        for (let i =0; i<rd.items.len; i++){
           ply.setQuadTex(rd.items.locations[i],
               String.fromCharCode(rd.items.textures[i])) 
        }
        pull_data = false;
    }
    render()
    requestAnimationFrame(renderLoop)
}
//https://developer.mozilla.org/en-US/docs/Web/API/window/requestAnimationFrame
requestAnimationFrame(renderLoop)

//TODO bind movment  

//TODO make render loop like in testing for glyph lib
//getRenderData
//in loop approche render data based on factor
//repeat

