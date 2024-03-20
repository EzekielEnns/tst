import * as sim from "./builds/wasm32-unknown-unknown/debug/simulation.wasm";
import bencode from "bencode";
import { getLayer, init, render } from "./src/lib.js";
import { addLayer } from "./src/lib.js";
import font from "/monogram.ttf";

var /** @type {number} */ ptr = sim.get_buffs(),
    /** @type {number} */ len = sim.get_len(),
    /** @type {number} */ skLen = sim.render_skills(),
    /** @type {number} */ skPtr = sim.get_len_skills(),
    /** @type {number} */ stPtr = sim.render_stats(),
    /** @type {number} */ stLen = sim.get_len_stats(),
    /** @type {HTMLCanvasElement} */ cnv = document.getElementById("canvas");
/** @type {WebAssembly.Memory} */
var memeory = sim.memory;
var pull_data = true;
var fistCombat = true;

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
    ptr = sim.get_buffs(ptr, len);
    let buff = new Uint8Array(memeory.buffer, ptr, sim.get_len());
    return bencode.decode(buff);
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
    skPtr = sim.render_skills(skPtr, skLen);
    skLen = sim.get_len_skills();
    let buff = new Uint8Array(memeory.buffer, skPtr, skLen);

    /** @type {Array<any>}*/
    let data = bencode.decode(buff);
    const dt = new TextDecoder();
    for (let i = 0; i < data.length; i++) {
        let raw = data[i][0];
        if (raw) {
            ["cost", "effect"].forEach((e) => {
                raw[e].hp = dt.decode(raw[e].hp);
                raw[e].sp = dt.decode(raw[e].sp);
            });
            raw.name = dt.decode(raw.name);
        }
        data[i] = raw;
    }
    return data;
}

/**
 *
 * @returns {Array<Stat>}
 */
function getStatsData() {
    stPtr = sim.render_stats(stPtr, stLen);
    stLen = sim.get_len_stats();
    let buff = new Uint8Array(memeory.buffer, stPtr, stLen);
    let data = bencode.decode(buff);
    const dt = new TextDecoder();
    for (let i = 0; i < data.length; i++) {
        let raw = data[i];
        if (raw) {
            raw.hp = parseInt(dt.decode(raw.hp));
            raw.sp = parseInt(dt.decode(raw.sp));
        }
    }
    return data;
}

function setStats() {
    let data = getStatsData();
    for (const i of document.querySelectorAll(".stat")) {
        i.classList.remove("hide");
        let index = i.classList.contains("player") ? 0 : 2; //magic numbers :D
        let current = data[index];
        let max = data[index + 1];

        if (i.classList.contains("hp")) {
            i.textContent = current.hp.toString();
            i.setAttribute("style", `width:${Math.max(current.hp / max.hp,0) * 100.0}%`);
        } else {
            i.textContent = data[index].sp.toString();
            i.setAttribute("style", `width:${(current.sp / max.sp) * 100.0}%`);
        }
    }
    return data[0].hp <= 0 || data[2].hp <= 0;
}
function unsetStats() {
    for (const i of document.querySelectorAll(".stat")) {
        i.classList.add("hide");
    }
    document.querySelector("#endturn").classList.add("hide");
}

function setSkills() {
    let data = getSkillData();
    for (let i = 0; i < buttons.length; i++) {
        if (data[i]) {
            buttons[i].textContent = data[i].name;
        }
    }
}

function unsetSkills() {
    for (let i = 0; i < buttons.length; i++) {
        let text = "";
        switch (buttons[i].id) {
            case "w":
                text = "up";
                break;
            case "a":
                text = "left";
                break;
            case "s":
                text = "down";
                break;
            case "d":
                text = "right";
                break;
        }
        buttons[i].textContent = text;
    }
}
await init(cnv, font);
//map layer
addLayer({
    params: {
        columns: 10,
        rows: 10,
        start: { x: -1, y: 1 },
        end: { x: 1, y: -1 },
        noFill: false,
    },
});
//things layer
addLayer(
    {
        params: {
            columns: 10,
            rows: 10,
            start: { x: -1, y: 1 },
            end: { x: 1, y: -1 },
            noFill: true,
        },
    },
    3
); //TODO add a way to add quads and remove quads

function renderLoop() {
    if (pull_data) {
        let rd = getRenderData();
        let map = getLayer(0);
        let ply = getLayer(1);
        for (let i = 0; i < rd.tiles.len; i++) {
            map.setQuadTex(i, String.fromCharCode(rd.tiles.textures[i]));
        }
        for (let i = 0; i < rd.actors.len; i++) {
            ply.setQuad(i, ply.getCell(rd.actors.locations[i]));
            ply.setQuadTex(i, String.fromCharCode(rd.actors.textures[i]));
        }
        let offset = rd.actors.len;
        for (let j = offset; j < rd.items.len + offset; j++) {
            let i = j - offset;
            ply.setQuad(j, ply.getCell(rd.items.locations[i]));
            ply.setQuadTex(j, String.fromCharCode(rd.items.textures[i]));
        }
        //clearing unused cells
        for (let i = offset + rd.items.len; i < ply.getLen(); i++) {
            ply.setQuadTex(i, " ");
        }
        pull_data = false;
    }
    render();
    requestAnimationFrame(renderLoop);
}

/**
 * @type{Array<Element>}
 */
var buttons = [];
for (const i of document.querySelector("#btnHolder").children) {
    i.addEventListener("click", () => {
        passInput(i.id);
    });
    buttons.push(i);
}

//TODO add key up and down so they can hold
window.addEventListener("keydown", (e) => {
    passInput(e.key);
});

function passInput(key) {
    let is_combat = exploration(key);
    if (is_combat) {
        if (fistCombat) {
            //set to true when combat done
            let endturn = document.querySelector("#endturn");
            endturn.classList.remove("hide");
            endturn.addEventListener("click", endTurn);
            fistCombat = false;
        } else {
            turn(key);
        }
        setStats();
        setSkills();
    }
}
function endTurn() {
    let combat_done = sim.end_turn();
    setStats();
    console.log(combat_done);
    if (combat_done) {
        let didPlayerWin = sim.clean_combat();
        unsetSkills();
        unsetStats();
        fistCombat = true;
        pull_data = true;
    } else {
        setSkills();
    }
}

function exploration(key) {
    let dir;
    switch (key) {
        case "w":
            dir = "u";
            pull_data = true;
            break;
        case "a":
            dir = "l";
            pull_data = true;
            break;
        case "s":
            dir = "d";
            pull_data = true;
            break;
        case "d":
            dir = "r";
            pull_data = true;
            break;
    }
    return !sim.move_pc(dir?.charCodeAt());
}

//for combat
function turn(key) {
    let index;
    switch (key) {
        case "a":
            index = 0;
            break;
        case "s":
            index = 1;
            break;
        case "w":
            index = 2;
            break;
        case "d":
            index = 3;
        case " ":
            endTurn();
            return;
    }
    sim.turn(index);
}

//https://developer.mozilla.org/en-US/docs/Web/API/window/requestAnimationFrame
requestAnimationFrame(renderLoop);
