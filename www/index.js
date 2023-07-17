import testWasm from "./wasm-test";
import RpgMapService from './services/RpgMaps';
import Stage from "./play/Stage";
import { viewWidth, viewHeight } from "./config";

// == TEST WASM ========================================================================================================

const testWasmButton = document.getElementById("test-wasm");

testWasmButton.addEventListener("click", () => {
    testWasm();
});

// == PLAY ULMO ========================================================================================================

const WAITING_STATE = 1;
const PLAY_CLICKED_STATE = 2;
const PLAYING_STATE = 3;

const FPS = 60;

const LIVE_MODE = true;

const rpgMapService = new RpgMapService();

const playerPositions = {
    "cave": {
        tx: 4,
        ty: 4,
        level: 1
    },
    "drops": {
        tx: 9,
        ty: 3,
        level: 1
    },
    "forest": {
        tx: 3,
        ty: 5,
        level: 3
    }
}

let state = WAITING_STATE;

let requestId = null; // only set if using requestAnimationFrame
let intervalId = null; // only set if using setInterval

const onEachFrame = cb => {
    // FULL SPEED
    // const _cb = () => {
    //     requestId = window.requestAnimationFrame(_cb);
    //     cb();
    // };
    // _cb();

    const interval = 1000/FPS;
    let then;

    const _cb = timestamp => {
        requestId = window.requestAnimationFrame(_cb);
        if (!then) {
            then = timestamp;
        }
        const delta = timestamp - then;

        if (delta > interval) {
            then = timestamp - (delta % interval);
            cb();
        }
    };
    _cb();
}

const startGame = async (rpgMap, playerPosition) => {
    const canvas = document.getElementById("ulmo-canvas");
    canvas.width = viewWidth;
    canvas.height = viewHeight;

    const { tx, ty, level } = playerPosition;
    const stage = new Stage(rpgMap, level, tx, ty, LIVE_MODE);
    await stage.initPlay();
    //let onEachFrame = assignOnEachFrame();
    onEachFrame(() => stage.executeMain(canvas));
    document.onkeydown = evt => {
        evt.preventDefault();
        stage.keyDown(evt.keyCode);
    };
    document.onkeyup = evt => {
        evt.preventDefault();
        stage.keyUp(evt.keyCode);
    }
};

const stopGame = () => {
    if (requestId) {
        window.cancelAnimationFrame(requestId);
        requestId = null;
    }
    if (intervalId) {
        clearInterval(intervalId);
        intervalId = null;
    }
};

const mapSelected = async mapName => {
    const result = await rpgMapService.loadMap(mapName)
    // alert(result.map.getName());
    await startGame(result.map, playerPositions[mapName]);
};

const playButton = document.getElementById("play-ulmo");

playButton.addEventListener("click", async () => {
    if (state == PLAY_CLICKED_STATE) {
        return;
    }
    else if (state == WAITING_STATE) {
        state = PLAY_CLICKED_STATE;
        await mapSelected('forest');
        playButton.textContent = " ⏸ ";
        state = PLAYING_STATE;
        return;
    }
    else if (state == PLAYING_STATE) {
        stopGame();
        playButton.textContent = " ▶ ";
        state = WAITING_STATE;
    }
});
