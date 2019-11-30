mod utils;

use wasm_bindgen::prelude::*;

use std::collections::{HashMap, BTreeMap};

extern crate web_sys;
use web_sys::console;
use wasm_bindgen::__rt::core::cmp::{max, min};

#[macro_use]
extern crate serde_derive;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        console::log_1(&format!( $( $t )* ).into());
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn is_odd(number: &i8) -> bool {
    number & 1 == 1
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
    width: u32,
    height: u32
}

#[wasm_bindgen]
impl Rect {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Rect {
        Rect { left: x, top: y, right: x + width as i32, bottom: y + height as i32, width, height }
    }

    pub fn get_x(&self) -> i32 {
        self.left
    }

    pub fn get_y(&self) -> i32 {
        self.top
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }
}

impl Rect {
    pub fn top_left(&self) -> (i32, i32) {
        (self.left, self.top)
    }

    pub fn bottom_right(&self) -> (i32, i32) {
        (self.right, self.bottom)
    }

    pub fn move_rect(&self, mx: i8, my: i8) -> Rect {
        let top = self.top + my as i32;
        let left = self.left + mx as i32;
        Rect {
            left,
            top,
            right: left + self.width as i32,
            bottom: top + self.height as i32,
            width: self.width,
            height: self.height
        }
    }

    pub fn top_left_delta(&self, other_rect: &Rect) -> (i8, i8) {
        let (left, top) = other_rect.top_left();
        ((left - self.left) as i8, (top - self.top) as i8)
    }
}

#[derive(Serialize, Deserialize)]
pub struct  MapTileData {
    levels: Vec<i8>,
    down_levels: Vec<(i8, u8)>,
    special_levels: Vec<i8>
}

impl MapTileData {
    pub fn new(
        levels: Vec<i8>,
        down_levels: Vec<(i8, u8)>,
        special_levels: Vec<i8>
    ) -> MapTileData {
        MapTileData { levels, down_levels, special_levels }
    }

    pub fn empty() -> MapTileData {
        Self::new(vec![], vec![], vec![])
    }

    pub fn with_levels(levels: Vec<i8>) -> MapTileData {
        Self::new(levels, vec![], vec![])
    }

    pub fn with_down_levels(down_levels: Vec<(i8, u8)>) -> MapTileData {
        Self::new(vec![], down_levels, vec![])
    }

    pub fn with_special_levels(special_levels: Vec<i8>) -> MapTileData {
        Self::new(vec![], vec![], special_levels)
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct MapTile {
    levels: Vec<i8>,
    down_levels: HashMap<i8, u8>,
    special_levels: BTreeMap<i8, i8>,
    old_levels: Option<Vec<i8>>
}

impl MapTile {
    pub fn new(
        levels: Vec<i8>,
        down_levels: HashMap<i8, u8>,
        special_levels: BTreeMap<i8, i8>
    ) -> MapTile {
        MapTile { levels, down_levels, special_levels, old_levels: None }
    }

    pub fn from_data(map_tile_data: MapTileData) -> MapTile {
        MapTile::new(
            map_tile_data.levels,
            map_tile_data.down_levels.into_iter().collect(),
            map_tile_data.special_levels.into_iter()
                .flat_map(|l| {
                    if is_odd(&l) {
                        vec![(l - 1, l), (l + 1, l)]
                    }
                    else {
                        vec![(l, l)]
                    }
                })
                .collect()
        )
    }

    fn get_special_level(&self, level: &i8) -> Option<i8> {
        if let Some(l) = self.special_levels.get(level) {
            return Some(*l);
        }
        if is_odd(level) {
            let level_range = (level - 1)..(level + 2);
            for (_, l) in self.special_levels.range(level_range) {
                return Some(*l);
            }
        }
        None
    }

    fn get_down_level(&self, level: &i8) -> Option<u8> {
        if let Some(dl) = self.down_levels.get(level) {
            return Some(*dl)
        }
        None
    }

    pub fn get_validity_of(&self, level: i8) -> (u8, Option<i8>) {
        if self.levels.contains(&level) {
            return (1, None);
        }
        if self.down_levels.contains_key(&level) {
            return (1, None);
        }
        let special_level = self.get_special_level(&level);
        if let Some(v) = special_level {
            return if v == level {
                (1, Some(v))
            }
            else {
                (0, Some(v))
            }
        }
        (0, None)
    }

    pub fn add_level(&mut self, level: i8) {
        //self.levels;
        self.old_levels = Some(self.levels.clone());
        self.levels.push(level);
    }

    pub fn rollback(&mut self) {
        if let Some(levels) = &self.old_levels {
            self.levels = levels.clone(); // could we do this with a pointer or something?
            self.old_levels = None;
        }
    }
}

// enum with explicit discriminator
enum Deferral {
    NONE = 0,
    DEFAULT = 1,
    DIAGONAL = 2,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct MoveResult {
    valid: bool,
    deferral: u8,
    level: i8,
    mx: i8,
    my: i8
}

#[wasm_bindgen]
impl MoveResult {
    pub fn new(valid: bool, deferral: u8, level: i8, mx: i8, my: i8) -> MoveResult {
        MoveResult { valid, deferral, level, mx, my }
    }

    pub fn is_valid(&self) -> bool {
        self.valid
    }

    pub fn get_deferral(&self) -> u8 {
        self.deferral
    }

    pub fn get_level(&self) -> i8 {
        self.level
    }

    pub fn get_mx(&self) -> i8 {
        self.mx
    }

    pub fn get_my(&self) -> i8 {
        self.my
    }
}

impl MoveResult {
    pub fn as_tuple(&self) -> (bool, u8, i8, i8, i8) {
        (self.valid, self.deferral, self.level, self.mx, self.my)
    }
}

// enum with explicit discriminator
enum EventType {
    NONE = 0,
    FALLING = 1,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct MapEvent {
    event_type: u8,
    value: u8,
}

#[wasm_bindgen]
impl MapEvent {
    pub fn new(event_type: u8, value: u8) -> MapEvent {
        MapEvent { event_type, value }
    }

    pub fn get_event_type(&self) -> u8 {
        self.event_type
    }

    pub fn get_value(&self) -> u8 {
        self.value
    }
}

impl MapEvent {
    pub fn as_tuple(&self) -> (u8, u8) {
        (self.event_type, self.value)
    }
}

#[derive(Serialize, Deserialize)]
pub struct PlayMapData {
    rows: u8,
    cols: u8,
    tile_data: Vec<MapTileData>,
    tile_size: u32
}

// for testing
impl PlayMapData {
    pub fn new(rows: u8, cols: u8, tile_data: Vec<MapTileData>, tile_size: u32) -> PlayMapData {
        PlayMapData { rows, cols, tile_data, tile_size }
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct PlayMap {
    rows: u8,
    cols: u8,
    tiles: Vec<MapTile>,
    tile_size: u32
}

#[wasm_bindgen]
impl PlayMap {
    pub fn from_js_data(val: &JsValue) -> PlayMap {
        let play_map_data: PlayMapData = val.into_serde().unwrap();
        PlayMap::from_data(play_map_data)
    }

    pub fn apply_move(&self, mx: i8, my: i8, level: i8, base_rect: Rect) -> MoveResult {
        log!("apply_move: received {} {} {} {:?}", mx, my, level, base_rect);
        let new_base_rect = base_rect.move_rect(mx, my);
        let (span_tiles, verticals, horizontals) = self.get_span_tiles_with_stripes(&new_base_rect);
        log!("span_tiles: {:?}", span_tiles);

        let (valid, new_level) = self.is_span_valid(level, &span_tiles);
        if valid {
            let (mx_delta, my_delta) = base_rect.top_left_delta(&new_base_rect);
            return if mx == 0 || my == 0 {
                MoveResult::new(true, Deferral::NONE as u8, new_level, mx_delta, my_delta)
            }
            else {
                MoveResult::new(true, Deferral::DIAGONAL as u8, new_level, mx_delta, my_delta)
            }
        }
        log!("primary move not valid");

        // movement invalid but we might be able to slide or shuffle
        if mx == 0 {
            // attempt shuffle left/right
            let (new_level, new_base_rect) = self.is_vertical_valid(level, &base_rect, &verticals);
            log!("mx=0: {} {:?}", new_level, new_base_rect);
            if let Some(rect) = new_base_rect {
                let (mx_delta, my_delta) = base_rect.top_left_delta(&rect);
                return MoveResult::new(true, Deferral::DEFAULT as u8, new_level, mx_delta, my_delta);
            }
        }
        else if my == 0 {
            // attempt shuffle up/down
            let (new_level, new_base_rect) = self.is_horizontal_valid(level, &base_rect, &horizontals);
            log!("my=0: {} {:?}", new_level, new_base_rect);
            if let Some(rect) = new_base_rect {
                let (mx_delta, my_delta) = base_rect.top_left_delta(&rect);
                return MoveResult::new(true, Deferral::DEFAULT as u8, new_level, mx_delta, my_delta);
            }
        }
        else {
            // diagonal movement - attempt slide
            let (new_level, new_base_rect) = self.is_slide_valid(mx, my, level, &base_rect);
            log!("diagonal: {} {:?}", new_level, new_base_rect);
            if let Some(rect) = new_base_rect {
                let (mx_delta, my_delta) = base_rect.top_left_delta(&rect);
                return MoveResult::new(true, Deferral::DEFAULT as u8, new_level, mx_delta, my_delta);
            }
        }

        MoveResult::new(false, Deferral::NONE as u8, level, 0, 0)
    }

    pub fn get_event(&self, level: i8, base_rect: Rect) -> MapEvent {
        log!("get_event: received {} {:?}", level, base_rect);
        let span_tiles = self.get_span_tiles(&base_rect);
        let falling = span_tiles.iter().all(| tile | {
            tile.get_down_level(&level).is_some()
        });
        if falling {
            let down_level = span_tiles.get(0).unwrap().get_down_level(&level).unwrap();
            MapEvent::new(EventType::FALLING as u8, down_level)
        }
        else {
            MapEvent::new(EventType::NONE as u8, 0)
        }
    }

    pub fn add_level_to_tile(&mut self, tx: u8, ty: u8, level: i8) {
        let index = self.get_index(tx, ty);
        if let Some(tile) = self.tiles.get_mut(index) {
            tile.add_level(level);
        }
    }

    pub fn rollback_tile(&mut self, tx: u8, ty: u8) {
        let index = self.get_index(tx, ty);
        if let Some(tile) = self.tiles.get_mut(index) {
            tile.rollback();
        }
    }
}

pub struct Shuffle {
    index1: usize,
    shuffle1: i8,
    index2: usize,
    shuffle2: i8
}

const MIN_SHUFFLE: Shuffle = Shuffle { index1: 0, shuffle1: -2, index2: 1, shuffle2: 2 };
const MAX_SHUFFLE: Shuffle = Shuffle { index1: 1, shuffle1: 2, index2: 0, shuffle2: -2 };

impl Shuffle {
    pub fn get_shuffle(min_diff: i32, max_diff: i32) -> Shuffle {
        if max_diff < min_diff {
            MIN_SHUFFLE
        }
        else {
            MAX_SHUFFLE
        }
    }
}

impl PlayMap {
    pub fn new(rows: u8, cols: u8, tiles: Vec<MapTile>, tile_size: u32) -> PlayMap {
        PlayMap { rows, cols, tiles, tile_size }
    }

    pub fn from_data(play_map_data: PlayMapData) -> PlayMap {
        PlayMap::new(
            play_map_data.rows,
            play_map_data.cols,
            play_map_data.tile_data.into_iter().map(|t| {
                MapTile::from_data(t)
            }).collect(),
            play_map_data.tile_size
        )
    }

    pub fn is_move_valid(&self, level: i8, base_rect: Rect) -> (bool, i8) {
        self.is_span_valid(level, &self.get_span_tiles(&base_rect))
    }

    pub fn is_span_valid(&self, level: i8, span_tiles: &[&MapTile]) -> (bool, i8) {
        let mut same_level_count = 0;
        let mut special_levels = vec![];
        for tile in span_tiles.iter() {
            let (inc, level) = tile.get_validity_of(level);
            same_level_count += inc;
            if let Some(v) = level {
                special_levels.push(v)
            }
        }
        if same_level_count as usize == span_tiles.len() {
            return (true, level);
        }
        if special_levels.len() == span_tiles.len() {
            let min_special = special_levels.iter().min().unwrap();
            let max_special = special_levels.iter().max().unwrap();
            if (max_special - min_special) < 2 {
                return if is_odd(min_special) {
                    (true, *max_special)
                }
                else {
                    (true, *min_special)
                }
            }
        }
        (false, level)
    }

    fn is_shuffle_valid(&self, level: i8, stripes: &[Vec<&MapTile>], shuffle: Shuffle) -> (bool, i8, i8) {
        let stripe = stripes.get(shuffle.index1).unwrap();
        let (valid, new_level) = self.is_span_valid(level, stripe);
        if valid {
            return (valid, new_level, shuffle.shuffle1)
        }
        let stripe = stripes.get(shuffle.index2).unwrap();
        let (valid, new_level) = self.is_span_valid(level, stripe);
        return (valid, new_level, shuffle.shuffle2)
    }

    fn is_stripe_valid(&self, level: i8, stripes: &[Vec<&MapTile>], min: i32, max: i32) -> (bool, i8, i8) {
        if stripes.len() < 2 {
            return (false, level, 0);
        }
        let min_diff = self.tile_size as i32 - (min % self.tile_size  as i32);
        let max_diff = max % self.tile_size as i32;
        self.is_shuffle_valid(level, stripes, Shuffle::get_shuffle(min_diff, max_diff))
    }

    fn is_vertical_valid(&self, level: i8, base_rect: &Rect, verticals: &[Vec<&MapTile>]) -> (i8, Option<Rect>) {
        let (valid, new_level, shuffle) = self.is_stripe_valid(level, verticals, base_rect.left, base_rect.right);
        if valid {
            return (new_level, Some(base_rect.move_rect(shuffle, 0)))
        }
        (0, None)
    }

    fn is_horizontal_valid(&self, level: i8, base_rect: &Rect, horizontals: &[Vec<&MapTile>]) -> (i8, Option<Rect>) {
        let (valid, new_level, shuffle) = self.is_stripe_valid(level, horizontals, base_rect.top, base_rect.bottom);
        if valid {
            return (new_level, Some(base_rect.move_rect(0, shuffle)))
        }
        (0, None)
    }

    fn is_slide_valid(&self, mx: i8, my: i8, level: i8, base_rect: &Rect) -> (i8, Option<Rect>) {
        let mx_base_rect = base_rect.move_rect(mx, 0);
        let (valid, new_level) = self.is_move_valid(level, mx_base_rect);
        if valid {
            return (new_level, Some(mx_base_rect));
        }
        let my_base_rect = base_rect.move_rect(0, my);
        let (valid, new_level) = self.is_move_valid(level, my_base_rect);
        if valid {
            return (new_level, Some(my_base_rect));
        }
        (0, None)
    }

    fn get_index(&self, x: u8, y: u8) -> usize {
        y as usize * self.cols as usize + x as usize
    }

    fn convert_rect(&self, rect: &Rect) -> (u8, u8, u8, u8) {
        let (left, top) = rect.top_left();
        let tx1 = max(0, left / self.tile_size as i32);
        let ty1 = max(0, top / self.tile_size as i32);
        let (right, bottom) = rect.bottom_right();
        let tx2 = min((self.cols - 1) as i32, (right - 1) / self.tile_size as i32) + 1;
        let ty2 = min((self.rows - 1) as i32, (bottom - 1) / self.tile_size as i32) + 1;
        log!("{} {} {} {}", tx1, ty1, tx2, ty2);
        (tx1 as u8, ty1 as u8, tx2 as u8, ty2 as u8)
    }

    fn get_span_tiles(&self, rect: &Rect) -> Vec<&MapTile> {
        let (tx1, ty1, tx2, ty2) = self.convert_rect(rect);
        let mut span_tiles = vec![];
        for x in tx1..tx2 {
            for y in ty1..ty2 {
                if let Some(tile) = self.tiles.get(self.get_index(x, y)) {
                    span_tiles.push(tile)
                }
            }
        }
        span_tiles
    }

    fn get_span_tiles_with_stripes(&self, rect: &Rect) -> (Vec<&MapTile>, Vec<Vec<&MapTile>>, Vec<Vec<&MapTile>>) {
        let (tx1, ty1, tx2, ty2) = self.convert_rect(rect);
        let mut span_tiles = vec![];
        let mut verticals = vec![];
        for x in tx1..tx2 {
            let mut vertical = vec![];
            for y in ty1..ty2 {
                if let Some(tile) = self.tiles.get(self.get_index(x, y)) {
                    span_tiles.push(tile);
                    vertical.push(tile);
                }
            }
            verticals.push(vertical);
        }
        // TODO: combine this loop into the one above?
        let mut horizontals = vec![];
        for y in ty1..ty2 {
            let mut horizontal = vec![];
            for x in tx1..tx2 {
                if let Some(tile) = self.tiles.get(self.get_index(x, y)) {
                    horizontal.push(tile);
                }
            }
            horizontals.push(horizontal);
        }
        (span_tiles, verticals, horizontals)
    }
}
