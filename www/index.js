import { WasmPlayMap, WasmRect } from "wasm-ulmo-map";

// construct a test map
const playMap = new WasmPlayMap({
    rows: 4,
    cols: 3,
    tileData: [{
        levels: [4],
        downLevels: [],
        specialLevels: [],
        masks: []
    }, {
        levels: [],
        downLevels: [],
        specialLevels: [4],
        masks: []
    }, {
        levels: [4],
        downLevels: [],
        specialLevels: [],
        masks: []
    }, {
        levels: [],
        downLevels: [],
        specialLevels: [],
        masks: []
    }, {
        levels: [],
        downLevels: [],
        specialLevels: [3],
        masks: []
    }, {
        levels: [],
        downLevels: [],
        specialLevels: [],
        masks: []
    }, {
        levels: [2],
        downLevels: [],
        specialLevels: [],
        masks: []
    }, {
        levels: [],
        downLevels: [],
        specialLevels: [3],
        masks: []
    }, {
        levels: [2],
        downLevels: [],
        specialLevels: [],
        masks: []
    }, {
        levels: [2],
        downLevels: [],
        specialLevels: [],
        masks: []
    }, {
        levels: [],
        downLevels: [],
        specialLevels: [2],
        masks: []
    }, {
        levels: [2],
        downLevels: [],
        specialLevels: [],
        masks: []
    }],
    tileSize: 16
});

// construct a test map with events
const playMapWithEvents = new WasmPlayMap({
    rows: 2,
    cols: 3,
    tileData: [{
        levels: [6],
        downLevels: [],
        specialLevels: [],
        masks: []
    }, {
        levels: [6],
        downLevels: [],
        specialLevels: [],
        masks: []
    }, {
        levels: [6],
        downLevels: [],
        specialLevels: [],
        masks: []
    }, {
        levels: [],
        downLevels: [],
        specialLevels: [],
        masks: []
    }, {
        levels: [],
        downLevels: [[6, 4]],
        specialLevels: [],
        masks: []
    }, {
        levels: [],
        downLevels: [],
        specialLevels: [],
        masks: []
    }],
    tileSize: 16
});

// construct a test map with masks
const playMapWithMasks = new WasmPlayMap({
    rows: 4,
    cols: 2,
    tileData: [{
        levels: [],
        downLevels: [],
        specialLevels: [],
        masks: []
    }, {
        levels: [],
        downLevels: [],
        specialLevels: [],
        masks: []
    }, {
        levels: [],
        downLevels: [],
        specialLevels: [],
        masks: [[1, 4, true, 1]]
    }, {
        levels: [],
        downLevels: [],
        specialLevels: [],
        masks: [[1, 4, true, 1]]
    }, {
        levels: [],
        downLevels: [],
        specialLevels: [],
        masks: [[0, 2, false, 2]]
    }, {
        levels: [],
        downLevels: [],
        specialLevels: [],
        masks: [[0, 2, false, 2]]
    }, {
        levels: [],
        downLevels: [],
        specialLevels: [],
        masks: []
    }, {
        levels: [],
        downLevels: [],
        specialLevels: [],
        masks: []
    }],
    tileSize: 16
});

/*
const fps = new class {
    constructor() {
        this.fps = document.getElementById("fps");
        this.frames = [];
        this.lastFrameTimeStamp = performance.now();
    }

    render() {
        // Convert the delta time since the last frame render into a measure
        // of frames per second.
        const now = performance.now();
        const delta = now - this.lastFrameTimeStamp;
        this.lastFrameTimeStamp = now;
        const fps = 1 / delta * 1000;

        // Save only the latest 100 timings.
        this.frames.push(fps);
        if (this.frames.length > 100) {
            this.frames.shift();
        }

        // Find the max, min, and mean of our 100 latest timings.
        let min = Infinity;
        let max = -Infinity;
        let sum = 0;
        for (let i = 0; i < this.frames.length; i++) {
            sum += this.frames[i];
            min = Math.min(this.frames[i], min);
            max = Math.max(this.frames[i], max);
        }
        let mean = sum / this.frames.length;

        // Render the statistics.
        this.fps.textContent = `
Frames per Second:
         latest = ${Math.round(fps)}
avg of last 100 = ${Math.round(mean)}
min of last 100 = ${Math.round(min)}
max of last 100 = ${Math.round(max)}
`.trim();
    }
};
*/

const testUlmoButton = document.getElementById("test-ulmo");
const testResults = document.getElementById("test-results");

const zIndex = (rectBottom, level, tileSize) => rectBottom + level * tileSize;

const formatMoveResult = result => {
    let { valid, deferral, level, mx, my } = result;
    return `MoveResult { valid: ${valid}, deferral: ${deferral}, level: ${level}, mx: ${mx}, my: ${my} }\n`;
};

const runApplyMove = () => {
    let results = "";

    // valid
    let result = playMap.applyMove(2, 0, 4, new WasmRect(2, 2, 16, 8));
    results += formatMoveResult(result);

    // diagonal
    result = playMap.applyMove(2, 2, 4, new WasmRect(2, 0, 16, 8));
    results += formatMoveResult(result);

    // shuffle
    result = playMap.applyMove(2, 0, 4, new WasmRect(0, 12, 16, 8));
    results += formatMoveResult(result);

    // slide
    result = playMap.applyMove(2, 2, 2, new WasmRect(0, 44, 16, 8));
    results += formatMoveResult(result);

    // invalid
    result = playMap.applyMove(2, 0, 2, new WasmRect(0, 34, 16, 8));
    results += formatMoveResult(result);

    return results;
};

const formatMapEvent = event => {
    if (event) {
        let { eventType, value } = event;
        return `MapEvent { eventType: ${eventType}, value: ${value} }\n`;
    }
    return "NULL\n";
};

const runGetEvent = () => {
    let results = "";

    let event = playMapWithEvents.getEvent(6, new WasmRect(20, 6, 8, 16));
    results += formatMapEvent(event);

    event = playMapWithEvents.getEvent(6, new WasmRect(20, 16, 8, 16));
    results += formatMapEvent(event);

    return results;
};

const formatMasks = masks => {
    let masksJson = JSON.stringify(masks);
    return `TileMasks { ${masksJson} }\n`;
};

const runGetSpriteMasks = () => {
    let results = "";

    let masks = playMapWithMasks.getSpriteMasks(
        new WasmRect(8, 2, 16, 8),
        zIndex(10, 2, 16),
        2,
        true
    );
    results += formatMasks(masks);

    masks = playMapWithMasks.getSpriteMasks(
        new WasmRect(8, 28, 16, 8),
        zIndex(36, 2, 16),
        2,
        true
    );
    results += formatMasks(masks);

    return results;
};

const runTest = () => {
    let results = [
      runApplyMove(),
      runGetEvent(),
      runGetSpriteMasks()
    ];
    testResults.textContent = results.join("\n");
};

testUlmoButton.addEventListener("click", event => {
    runTest()
});
