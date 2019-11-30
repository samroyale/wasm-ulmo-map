//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

extern crate wasm_ulmo_map;
use wasm_ulmo_map::{PlayMap, Rect, MapTileData, PlayMapData, MapTile};

// [4] [S4] [4]  <- level 4
// [X] [S3] [X]  <- top of steps + wall on either side
// [2] [S3] [2]  <- bottom of steps + level 2 on either side
// [2] [S2] [2]  <- level 2
#[cfg(test)]
pub fn an_example_play_map() -> PlayMap {
    let map_tiles = vec![
        MapTileData::with_levels(vec![4]),         // 0
        MapTileData::with_special_levels(vec![4]), // 1
        MapTileData::with_levels(vec![4]),         // 2
        MapTileData::empty(),                      // 3
        MapTileData::with_special_levels(vec![3]), // 4
        MapTileData::empty(),                      // 5
        MapTileData::with_levels(vec![2]),         // 6
        MapTileData::with_special_levels(vec![3]), // 7
        MapTileData::with_levels(vec![2]),         // 8
        MapTileData::with_levels(vec![2]),         // 9
        MapTileData::with_special_levels(vec![2]), // 10
        MapTileData::with_levels(vec![2]),         // 11
    ];

    PlayMap::from_data(PlayMapData::new(4, 3, map_tiles, 16))
}

// [6]  [6]   [6]  <- level 6
// [X] [D6-4] [X]  <- drop level + wall on either side
// [X]  [X]   [X]  <- wall
// [2]  [2]   [2]  <- level 2
pub fn another_example_play_map() -> PlayMap {
    let map_tiles = vec![
        MapTileData::with_levels(vec![6]),           // 0
        MapTileData::with_levels(vec![6]),           // 1
        MapTileData::with_levels(vec![6]),           // 2
        MapTileData::empty(),                        // 3
        MapTileData::with_down_levels(vec![(6, 4)]), // 4
        MapTileData::empty(),                        // 5
        MapTileData::empty(),                        // 6
        MapTileData::empty(),                        // 7
        MapTileData::empty(),                        // 8
        MapTileData::with_levels(vec![2]),           // 9
        MapTileData::with_levels(vec![2]),           // 10
        MapTileData::with_levels(vec![2]),           // 11
    ];

    PlayMap::from_data(PlayMapData::new(4, 3, map_tiles, 16))
}

#[wasm_bindgen_test]
pub fn test_play_map_apply_move_valid() {
    let play_map = an_example_play_map();

    /*
     * spans [4] [S4]
     */
    let (valid, defer, level, mx, my) = play_map.apply_move(0, 0, 4, Rect::new(4, 2, 16, 8)).as_tuple();
    assert!(valid);
    assert_eq!(defer, 0);
    assert_eq!(level, 4);
    assert_eq!(mx, 0);
    assert_eq!(my, 0);

    /*
     * spans [2] [S2]
     */
    let (valid, defer, level, mx, my) = play_map.apply_move(0, 0, 2, Rect::new(4, 50, 16, 8)).as_tuple();
    assert!(valid);
    assert_eq!(defer, 0);
    assert_eq!(level, 2);
    assert_eq!(mx, 0);
    assert_eq!(my, 0);

    /*
     * spans [S4]
     *       [S3]
     */
    let (valid, defer, level, mx, my) = play_map.apply_move(0, 0, 4, Rect::new(20, 2, 8, 16)).as_tuple();
    assert!(valid);
    assert_eq!(defer, 0);
    assert_eq!(level, 4);
    assert_eq!(mx, 0);
    assert_eq!(my, 0);
    let (valid, defer, level, mx, my) = play_map.apply_move(0, 0, 3, Rect::new(20, 2, 8, 16)).as_tuple();
    assert!(valid);
    assert_eq!(defer, 0);
    assert_eq!(level, 4);
    assert_eq!(mx, 0);
    assert_eq!(my, 0);

    /*
     * spans [S3]
     *       [S3]
     */
    let (valid, defer, level, mx, my) = play_map.apply_move(0, 0, 4, Rect::new(20, 20, 8, 16)).as_tuple();
    assert!(valid);
    assert_eq!(defer, 0);
    assert_eq!(level, 3);
    assert_eq!(mx, 0);
    assert_eq!(my, 0);
    let (valid, defer, level, mx, my) = play_map.apply_move(0, 0, 3, Rect::new(20, 20, 8, 16)).as_tuple();
    assert!(valid);
    assert_eq!(defer, 0);
    assert_eq!(level, 3);
    assert_eq!(mx, 0);
    assert_eq!(my, 0);
    let (valid, defer, level, mx, my) = play_map.apply_move(0, 0, 2, Rect::new(20, 20, 8, 16)).as_tuple();
    assert!(valid);
    assert_eq!(defer, 0);
    assert_eq!(level, 3);
    assert_eq!(mx, 0);
    assert_eq!(my, 0);

    /*
     * spans [S3]
     *       [S2]
     */
    let (valid, defer, level, mx, my) = play_map.apply_move(0, 0, 3, Rect::new(20, 36, 8, 16)).as_tuple();
    assert!(valid);
    assert_eq!(defer, 0);
    assert_eq!(level, 2);
    assert_eq!(mx, 0);
    assert_eq!(my, 0);
    let (valid, defer, level, mx, my) = play_map.apply_move(0, 0, 2, Rect::new(20, 36, 8, 16)).as_tuple();
    assert!(valid);
    assert_eq!(defer, 0);
    assert_eq!(level, 2);
    assert_eq!(mx, 0);
    assert_eq!(my, 0);
}

#[wasm_bindgen_test]
pub fn test_play_map_apply_move_shuffle() {
    let play_map = an_example_play_map();

    /*
     * spans [X] [S3]
     */
    let (valid, defer, level, mx, my) = play_map.apply_move(0, 0, 3, Rect::new(4, 18, 16, 8)).as_tuple();
    assert!(valid);
    assert_eq!(defer, 1);
    assert_eq!(level, 3);
    assert_eq!(mx, 2);
    assert_eq!(my, 0);
    let (valid, defer, level, mx, my) = play_map.apply_move(0, 0, 2, Rect::new(4, 18, 16, 8)).as_tuple();
    assert!(valid);
    assert_eq!(defer, 1);
    assert_eq!(level, 3);
    assert_eq!(mx, 2);
    assert_eq!(my, 0);
    /*
     * spans [4] [S4]
     *       [X] [S3]
     */
    let (valid, defer, level, mx, my) = play_map.apply_move(2, 0, 4, Rect::new(0, 12, 16, 8)).as_tuple();
    assert!(valid);
    assert_eq!(defer, 1);
    assert_eq!(level, 4);
    assert_eq!(mx, 0);
    assert_eq!(my, -2);

    /*
     * spans [2] [S3]
     */
    let (valid, defer, level, mx, my) = play_map.apply_move(0, 0, 3, Rect::new(4, 34, 16, 8)).as_tuple();
    assert!(valid);
    assert_eq!(defer, 1);
    assert_eq!(level, 3);
    assert_eq!(mx, 2);
    assert_eq!(my, 0);
    let (valid, defer, level, mx, my) = play_map.apply_move(0, 0, 2, Rect::new(4, 34, 16, 8)).as_tuple();
    assert!(valid);
    assert_eq!(defer, 1);
    assert_eq!(level, 2);
    assert_eq!(mx, -2);
    assert_eq!(my, 0);
    let (valid, defer, level, mx, my) = play_map.apply_move(0, 0, 2, Rect::new(12, 34, 16, 8)).as_tuple();
    assert!(valid);
    assert_eq!(defer, 1);
    assert_eq!(level, 3);
    assert_eq!(mx, 2);
    assert_eq!(my, 0);
    /*
     * spans [2] [S3]
     *       [2] [S2]
     */
    let (valid, defer, level, mx, my) = play_map.apply_move(2, 0, 2, Rect::new(0, 44, 16, 8)).as_tuple();
    assert!(valid);
    assert_eq!(defer, 1);
    assert_eq!(level, 2);
    assert_eq!(mx, 0);
    assert_eq!(my, 2);
}

#[wasm_bindgen_test]
pub fn test_play_map_apply_move_slide() {
    let play_map = an_example_play_map();

    /*
     * spans [4] [S4]
     *       [X] [S3]
     */
    let (valid, defer, level, mx, my) = play_map.apply_move(2, 2, 4, Rect::new(12, 0, 8, 16)).as_tuple();
    assert!(valid);
    assert_eq!(defer, 1);
    assert_eq!(level, 4);
    assert_eq!(mx, 2);
    assert_eq!(my, 0);
    let (valid, defer, level, mx, my) = play_map.apply_move(-2, 2, 4, Rect::new(12, 0, 8, 16)).as_tuple();
    assert!(valid);
    assert_eq!(defer, 1);
    assert_eq!(level, 4);
    assert_eq!(mx, -2);
    assert_eq!(my, 0);

    /*
     * spans [2] [S3]
     *       [2] [S2]
     */
    let (valid, defer, level, mx, my) = play_map.apply_move(2, 2, 2, Rect::new(0, 44, 16, 8)).as_tuple();
    assert!(valid);
    assert_eq!(defer, 1);
    assert_eq!(level, 2);
    assert_eq!(mx, 0);
    assert_eq!(my, 2);
    let (valid, defer, level, mx, my) = play_map.apply_move(2, -2, 2, Rect::new(0, 44, 16, 8)).as_tuple();
    assert!(valid);
    assert_eq!(defer, 1);
    assert_eq!(level, 2);
    assert_eq!(mx, 0);
    assert_eq!(my, -2);
}

#[wasm_bindgen_test]
pub fn test_play_map_apply_move_invalid() {
    let play_map = an_example_play_map();

    /*
     * spans [2] [S3]
     */
    let (valid, defer, level, mx, my) = play_map.apply_move(2, 0, 2, Rect::new(0, 34, 16, 8)).as_tuple();
    assert_eq!(valid, false);
    assert_eq!(defer, 0);
    assert_eq!(level, 2);
    assert_eq!(mx, 0);
    assert_eq!(my, 0);
    let (valid, defer, level, mx, my) = play_map.apply_move(-2, 0, 3, Rect::new(16, 34, 16, 8)).as_tuple();
    assert_eq!(valid, false);
    assert_eq!(defer, 0);
    assert_eq!(level, 3);
    assert_eq!(mx, 0);
    assert_eq!(my, 0);

    /*
     * spans [4]
     *       [X]
     */
    let (valid, defer, level, mx, my) = play_map.apply_move(0, 2, 4, Rect::new(2, 0, 8, 16)).as_tuple();
    assert_eq!(valid, false);
    assert_eq!(defer, 0);
    assert_eq!(level, 4);
    assert_eq!(mx, 0);
    assert_eq!(my, 0);

    /*
     * spans [X]
     *       [2]
     */
    let (valid, defer, level, mx, my) = play_map.apply_move(0, -2, 2, Rect::new(2, 32, 8, 16)).as_tuple();
    assert_eq!(valid, false);
    assert_eq!(defer, 0);
    assert_eq!(level, 2);
    assert_eq!(mx, 0);
    assert_eq!(my, 0);
}

#[wasm_bindgen_test]
pub fn test_play_map_add_and_rollback() {
    let mut play_map = an_example_play_map();

    /*
     * spans [X]
     *       [2]
     */
    let (valid, defer, level, mx, my) = play_map.apply_move(0, -2, 2, Rect::new(34, 32, 8, 16)).as_tuple();
    assert_eq!(valid, false);
    assert_eq!(defer, 0);
    assert_eq!(level, 2);
    assert_eq!(mx, 0);
    assert_eq!(my, 0);

    // add level 2 to the [X] tile (move is now valid)
    play_map.add_level_to_tile(2, 1, 2);
    let (valid, defer, level, mx, my) = play_map.apply_move(0, -2, 2, Rect::new(34, 32, 8, 16)).as_tuple();
    assert!(valid);
    assert_eq!(defer, 0);
    assert_eq!(level, 2);
    assert_eq!(mx, 0);
    assert_eq!(my, -2);

    // now roll it back (move invalid again)
    play_map.rollback_tile(2, 1);
    let (valid, defer, level, mx, my) = play_map.apply_move(0, -2, 2, Rect::new(34, 32, 8, 16)).as_tuple();
    assert_eq!(valid, false);
    assert_eq!(defer, 0);
    assert_eq!(level, 2);
    assert_eq!(mx, 0);
    assert_eq!(my, 0);
}

#[wasm_bindgen_test]
pub fn test_play_map_get_event() {
    let play_map = another_example_play_map();

    /*
     * spans [6] [D6-4]
     */
    let (valid, defer, level, mx, my) = play_map.apply_move(0, 2, 6, Rect::new(20, 4, 8, 16)).as_tuple();
    assert!(valid);
    assert_eq!(defer, 0);
    assert_eq!(level, 6);
    assert_eq!(mx, 0);
    assert_eq!(my, 2);

    let (event, value) = play_map.get_event(6, Rect::new(20, 6, 8, 16)).as_tuple();
    assert_eq!(event, 0);
    assert_eq!(value, 0);

    /*
     * spans [D6-4]
     */
    let (valid, defer, level, mx, my) = play_map.apply_move(0, 2, 6, Rect::new(20, 14, 8, 16)).as_tuple();
    assert!(valid);
    assert_eq!(defer, 0);
    assert_eq!(level, 6);
    assert_eq!(mx, 0);
    assert_eq!(my, 2);

    let (event, value) = play_map.get_event(6, Rect::new(20, 16, 8, 16)).as_tuple();
    assert_eq!(event, 1);
    assert_eq!(value, 4);
}

/*
#[wasm_bindgen_test]
pub fn test_play_map_is_move_valid() {
    let play_map = PlayMap::new(4, 3, some_map_tiles());

    let (valid, level) = play_map.is_move_valid(4, Rect::new(4, 2, 16, 8));
    assert!(valid);
    assert_eq!(level, 4);
    let (valid, level) = play_map.is_move_valid(3, Rect::new(4, 18, 16, 8));
    assert_eq!(valid, false);
    assert_eq!(level, 3);
    let (valid, level) = play_map.is_move_valid(3, Rect::new(4, 34, 16, 8));
    assert_eq!(valid, false);
    assert_eq!(level, 3);
    let (valid, level) = play_map.is_move_valid(2, Rect::new(4, 50, 16, 8));
    assert!(valid);
    assert_eq!(level, 2);

    let (valid, level) = play_map.is_move_valid(4, Rect::new(20, 2, 8, 16));
    assert!(valid);
    assert_eq!(level, 4);
    let (valid, level) = play_map.is_move_valid(3, Rect::new(20, 2, 8, 16));
    assert!(valid);
    assert_eq!(level, 4);
    let (valid, level) = play_map.is_move_valid(4, Rect::new(20, 20, 8, 16));
    assert!(valid);
    assert_eq!(level, 3);
    let (valid, level) = play_map.is_move_valid(3, Rect::new(20, 20, 8, 16));
    assert!(valid);
    assert_eq!(level, 3);
    let (valid, level) = play_map.is_move_valid(2, Rect::new(20, 20, 8, 16));
    assert!(valid);
    assert_eq!(level, 3);
    let (valid, level) = play_map.is_move_valid(3, Rect::new(20, 36, 8, 16));
    assert!(valid);
    assert_eq!(level, 2);
    let (valid, level) = play_map.is_move_valid(2, Rect::new(20, 36, 8, 16));
    assert!(valid);
    assert_eq!(level, 2);
}

#[wasm_bindgen_test]
pub fn test_map_tile_levels() {
    let map_tile = MapTile::with_levels(vec![4, 6]);
    let (inc, level) = map_tile.get_validity_of(4);
    assert_eq!(inc, 1);
    assert!(level.is_none());
    let (inc, level) = map_tile.get_validity_of(6);
    assert_eq!(inc, 1);
    assert!(level.is_none());
    let (inc, level) = map_tile.get_validity_of(8);
    assert_eq!(inc, 0);
    assert!(level.is_none());
}

#[wasm_bindgen_test]
pub fn test_map_tile_down_levels() {
    let map_tile = MapTile::with_down_levels(vec![(2, 4)]);
    let (inc, level) = map_tile.get_validity_of(2);
    assert_eq!(inc, 1);
    assert!(level.is_none());
    let (inc, level) = map_tile.get_validity_of(4);
    assert_eq!(inc, 0);
    assert!(level.is_none());
}
*/

//#[wasm_bindgen_test]
//pub fn test_map_tile_add_new_level() {
//    let map_tile_data = MapTileData::with_levels(vec![1, 2, 3]);
//    let mut map_tile = MapTile::from_data(map_tile_data);
//    let (inc, level) = map_tile.get_validity_of(3);
//    assert_eq!(inc, 1);
//    assert!(level.is_none());
//    let (inc, level) = map_tile.get_validity_of(4);
//    assert_eq!(inc, 0);
//    assert!(level.is_none());
//
//    map_tile.add_level(4);
//    let (inc, level) = map_tile.get_validity_of(4);
//    assert_eq!(inc, 1);
//    assert!(level.is_none());
//
//    map_tile.rollback();
//    let (inc, level) = map_tile.get_validity_of(4);
//    assert_eq!(inc, 0);
//    assert!(level.is_none());
//}
