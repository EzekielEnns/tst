import * as sim from "./dist/wasm32-unknown-unknown/debug/simulation.wasm"
import bencode from 'bencode'
import { init } from "glyplib";

var/** @type {number} */ ptr = sim.get_buffs(),
   /** @type {number} */ len = sim.get_len(),
   /** @type {HTMLCanvasElement} */ cnv = document.getElementById("canvas")

/** @type {WebAssembly.Memory} */
var memeory = sim.memory;

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
//render TODO move init to be higher up/happen automatically
(async ()=> { 
    //inits the layers and webgl
    await init(cnv,"monogram.ttf")
    console.log("hi")
    //https://developer.mozilla.org/en-US/docs/Web/API/window/requestAnimationFrame
    function renderLoop() {
        requestAnimationFrame(renderLoop)
    }
    requestAnimationFrame(renderLoop)
})()


//TODO bind movment  

//TODO make render loop like in testing for glyph lib
//getRenderData
//in loop approche render data based on factor
//repeat

