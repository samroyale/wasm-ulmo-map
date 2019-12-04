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
use wasm_ulmo_map::{PlayMap, Rect, MapTileData, PlayMapData, TileMasks};

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
pub fn an_example_play_map_with_down_levels() -> PlayMap {
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

// [6]  [6]   [6]  <- level 6
// [X] [D6-4] [X]  <- drop level + wall on either side
// [X]  [X]   [X]  <- wall
// [2]  [2]   [2]  <- level 2
pub fn an_example_play_map_with_masks() -> PlayMap {
    let map_tiles = vec![
        MapTileData::empty(),                            // 0
        MapTileData::empty(),                            // 1
        MapTileData::empty(),                            // 2
        MapTileData::with_masks(vec![(1, 4, true, 1)]),  // 3
        MapTileData::with_masks(vec![(1, 4, true, 1)]),  // 4
        MapTileData::with_masks(vec![(1, 4, true, 1)]),  // 5
        MapTileData::with_masks(vec![(0, 2, false, 2)]), // 6
        MapTileData::with_masks(vec![(0, 2, false, 2)]), // 7
        MapTileData::with_masks(vec![(0, 2, false, 2)]), // 8
        MapTileData::empty(),                            // 9
        MapTileData::empty(),                            // 10
        MapTileData::empty(),                            // 11
    ];

    PlayMap::from_data(PlayMapData::new(4, 3, map_tiles, 16))
}

fn get_z_index(rect: Rect, level: i32, tile_size: u8) -> i32 {
    let (_, bottom) = rect.bottom_right();
    return bottom + level * tile_size as i32;
}

#[wasm_bindgen_test]
pub fn test_play_map_get_masks_spright_upright() {
    let play_map = an_example_play_map_with_masks();

    /*
     * spans [] []
     */
    let sprite_rect = Rect::new(8, 2, 16, 8);
    // sprite level 2
    let sprite_z = get_z_index(sprite_rect, 2, 16);
    let js_value = play_map.get_js_sprite_masks(sprite_rect, sprite_z, 2, true);
    let masks: Vec<TileMasks> = js_value.into_serde().unwrap();
    assert_eq!(masks.len(), 0);
    // sprite level 4
    let sprite_z = get_z_index(sprite_rect, 4, 16);
    let js_value = play_map.get_js_sprite_masks(sprite_rect, sprite_z, 4, true);
    let masks: Vec<TileMasks> = js_value.into_serde().unwrap();
    assert_eq!(masks.len(), 0);

    /*
     * spans   []     []
     *       [m4 1] [m4 1]
     */
    let sprite_rect = Rect::new(8, 12, 16, 8);
    // sprite level 2
    let sprite_z = get_z_index(sprite_rect, 2, 16);
    let js_value = play_map.get_js_sprite_masks(sprite_rect, sprite_z, 2, true);
    let masks: Vec<TileMasks> = js_value.into_serde().unwrap();
    assert_eq!(masks.len(), 2);
    let mask1 = masks.get(0).unwrap();
    assert_eq!(mask1, &TileMasks::new(0, 1, vec![1]));
    let mask2 = masks.get(1).unwrap();
    assert_eq!(mask2, &TileMasks::new(1, 1, vec![1]));
    // sprite level 4
    let sprite_z = get_z_index(sprite_rect, 4, 16);
    let js_value = play_map.get_js_sprite_masks(sprite_rect, sprite_z, 4, true);
    let masks: Vec<TileMasks> = js_value.into_serde().unwrap();
    assert_eq!(masks.len(), 0);

    /*
     * spans [m4 1] [m4 1]
     *       [m2 0] [m2 0]
     */
    let sprite_rect = Rect::new(8, 28, 16, 8);
    // sprite level 2
    let sprite_z = get_z_index(sprite_rect, 2, 16);
    let js_value = play_map.get_js_sprite_masks(sprite_rect, sprite_z, 2, true);
    let masks: Vec<TileMasks> = js_value.into_serde().unwrap();
    assert_eq!(masks.len(), 4);
    let mask1 = masks.get(0).unwrap();
    assert_eq!(mask1, &TileMasks::new(0, 1, vec![1]));
    let mask2 = masks.get(1).unwrap();
    assert_eq!(mask2, &TileMasks::new(0, 2, vec![0]));
    let mask3 = masks.get(2).unwrap();
    assert_eq!(mask3, &TileMasks::new(1, 1, vec![1]));
    let mask4 = masks.get(3).unwrap();
    assert_eq!(mask4, &TileMasks::new(1, 2, vec![0]));
    // sprite level 4
    let sprite_z = get_z_index(sprite_rect, 4, 16);
    let js_value = play_map.get_js_sprite_masks(sprite_rect, sprite_z, 4, true);
    let masks: Vec<TileMasks> = js_value.into_serde().unwrap();
    assert_eq!(masks.len(), 0);

    /*
     * spans [m2 0] [m2 0]
     *         []     []
     */
    let sprite_rect = Rect::new(8, 44, 16, 8);
    // sprite level 2
    let sprite_z = get_z_index(sprite_rect, 2, 16);
    let js_value = play_map.get_js_sprite_masks(sprite_rect, sprite_z, 2, true);
    let masks: Vec<TileMasks> = js_value.into_serde().unwrap();
    assert_eq!(masks.len(), 0);
    // sprite level 4
    let sprite_z = get_z_index(sprite_rect, 4, 16);
    let js_value = play_map.get_js_sprite_masks(sprite_rect, sprite_z, 4, true);
    let masks: Vec<TileMasks> = js_value.into_serde().unwrap();
    assert_eq!(masks.len(), 0);
}

#[wasm_bindgen_test]
pub fn test_play_map_get_masks_sprite_flat() {
    let play_map = an_example_play_map_with_masks();

    /*
     * spans [] []
     */
    let sprite_rect = Rect::new(8, 2, 16, 8);
    // sprite level 2
    let sprite_z = get_z_index(sprite_rect, 2, 16);
    let js_value = play_map.get_js_sprite_masks(sprite_rect, sprite_z, 2, false);
    let masks: Vec<TileMasks> = js_value.into_serde().unwrap();
    assert_eq!(masks.len(), 0);
    // sprite level 4
    let sprite_z = get_z_index(sprite_rect, 4, 16);
    let js_value = play_map.get_js_sprite_masks(sprite_rect, sprite_z, 4, false);
    let masks: Vec<TileMasks> = js_value.into_serde().unwrap();
    assert_eq!(masks.len(), 0);

    /*
     * spans   []     []
     *       [m4 1] [m4 1]
     */
    let sprite_rect = Rect::new(8, 12, 16, 8);
    // sprite level 2
    let sprite_z = get_z_index(sprite_rect, 2, 16);
    let js_value = play_map.get_js_sprite_masks(sprite_rect, sprite_z, 2, false);
    let masks: Vec<TileMasks> = js_value.into_serde().unwrap();
    assert_eq!(masks.len(), 2);
    let mask1 = masks.get(0).unwrap();
    assert_eq!(mask1, &TileMasks::new(0, 1, vec![1]));
    let mask2 = masks.get(1).unwrap();
    assert_eq!(mask2, &TileMasks::new(1, 1, vec![1]));
    // sprite level 4
    let sprite_z = get_z_index(sprite_rect, 4, 16);
    let js_value = play_map.get_js_sprite_masks(sprite_rect, sprite_z, 4, false);
    let masks: Vec<TileMasks> = js_value.into_serde().unwrap();
    assert_eq!(masks.len(), 0);

    /*
     * spans [m4 1] [m4 1]
     *       [m2 0] [m2 0]
     */
    let sprite_rect = Rect::new(8, 28, 16, 8);
    // sprite level 2
    let sprite_z = get_z_index(sprite_rect, 2, 16);
    let js_value = play_map.get_js_sprite_masks(sprite_rect, sprite_z, 2, false);
    let masks: Vec<TileMasks> = js_value.into_serde().unwrap();
    assert_eq!(masks.len(), 4);
    let mask1 = masks.get(0).unwrap();
    assert_eq!(mask1, &TileMasks::new(0, 1, vec![1]));
    let mask2 = masks.get(1).unwrap();
    assert_eq!(mask2, &TileMasks::new(0, 2, vec![0]));
    let mask3 = masks.get(2).unwrap();
    assert_eq!(mask3, &TileMasks::new(1, 1, vec![1]));
    let mask4 = masks.get(3).unwrap();
    assert_eq!(mask4, &TileMasks::new(1, 2, vec![0]));
    // sprite level 4
    let sprite_z = get_z_index(sprite_rect, 4, 16);
    let js_value = play_map.get_js_sprite_masks(sprite_rect, sprite_z, 4, false);
    let masks: Vec<TileMasks> = js_value.into_serde().unwrap();
    assert_eq!(masks.len(), 2); // TODO: should be 0?

    /*
     * spans [m2 0] [m2 0]
     *         []     []
     */
    let sprite_rect = Rect::new(8, 44, 16, 8);
    // sprite level 2
    let sprite_z = get_z_index(sprite_rect, 2, 16);
    let js_value = play_map.get_js_sprite_masks(sprite_rect, sprite_z, 2, false);
    let masks: Vec<TileMasks> = js_value.into_serde().unwrap();
    assert_eq!(masks.len(), 2);
    let mask1 = masks.get(0).unwrap();
    assert_eq!(mask1, &TileMasks::new(0, 2, vec![0]));
    let mask2 = masks.get(1).unwrap();
    assert_eq!(mask2, &TileMasks::new(1, 2, vec![0]));
    // sprite level 4
    let sprite_z = get_z_index(sprite_rect, 4, 16);
    let js_value = play_map.get_js_sprite_masks(sprite_rect, sprite_z, 4, false);
    let masks: Vec<TileMasks> = js_value.into_serde().unwrap();
    assert_eq!(masks.len(), 0);
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
    let play_map = an_example_play_map_with_down_levels();

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

#[wasm_bindgen_test]
pub fn test_map_tile_add_new_level() {
    let map_tile_data = MapTileData::with_levels(vec![1, 2, 3]);
    let mut map_tile = MapTile::from_data(map_tile_data);
    let (inc, level) = map_tile.get_validity_of(3);
    assert_eq!(inc, 1);
    assert!(level.is_none());
    let (inc, level) = map_tile.get_validity_of(4);
    assert_eq!(inc, 0);
    assert!(level.is_none());

    map_tile.add_level(4);
    let (inc, level) = map_tile.get_validity_of(4);
    assert_eq!(inc, 1);
    assert!(level.is_none());

    map_tile.rollback();
    let (inc, level) = map_tile.get_validity_of(4);
    assert_eq!(inc, 0);
    assert!(level.is_none());
}
*/