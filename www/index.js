import { PlayMap, Rect } from "wasm-ulmo-map";

// construct an example map
// const playMap = PlayMap.an_example()
const playMap = PlayMap.from_js_data({
    rows: 4,
    cols: 3,
    tile_data: [{
        levels:[4],
        down_levels:[],
        special_levels:[],
        masks: []
    }, {
        levels:[],
        down_levels:[],
        special_levels:[],
        masks: []
    }, {
        levels:[4],
        down_levels:[],
        special_levels:[],
        masks: []
    }, {
        levels:[],
        down_levels:[],
        special_levels:[],
        masks: []
    }, {
        levels:[],
        down_levels:[],
        special_levels:[],
        masks: []
    }, {
        levels:[],
        down_levels:[],
        special_levels:[],
        masks: []
    }, {
        levels:[2],
        down_levels:[],
        special_levels:[],
        masks: []
    }, {
        levels:[],
        down_levels:[],
        special_levels:[],
        masks: []
    }, {
        levels:[2],
        down_levels:[],
        special_levels:[],
        masks: []
    }, {
        levels:[2],
        down_levels:[],
        special_levels:[],
        masks: []
    }, {
        levels:[],
        down_levels:[],
        special_levels:[],
        masks: []
    }, {
        levels:[2],
        down_levels:[],
        special_levels:[],
        masks: []
    }],
    tile_size: 16
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

const formatResult = result => {
    return `Result { valid: ${result.is_valid()}, deferral: ${result.get_deferral()}, level: ${result.get_level()}, mx: ${result.get_mx()}, my: ${result.get_my()} }\n`;
};

const runTest = () => {
   let results = "";

    // valid
    let result = playMap.apply_move(0, 0, 4, Rect.new(4, 2, 16, 8));
    results += formatResult(result);

    // shuffle
    result = playMap.apply_move(2, 0, 4, Rect.new(0, 12, 16, 8));
    results += formatResult(result);

    // slide
    result = playMap.apply_move(2, 2, 2, Rect.new(0, 44, 16, 8));
    results += formatResult(result);

    // invalid
    result = playMap.apply_move(2, 0, 2, Rect.new(0, 34, 16, 8));
    results += formatResult(result);

    testResults.textContent = results;
};

testUlmoButton.addEventListener("click", event => {
    runTest()
});
