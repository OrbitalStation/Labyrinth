// Uses

use crate::{
    tile::*,
    player
};
use super::{
    types::{Data, SizeT},
    digger
};
use rand::Rng;

// Static

static mut DATA: Data = Data {
    width:  0,
    height: 0,
    tiles: Vec::new(),
    updated: true
};

// Fns

#[inline(never)]
unsafe fn idx(x: SizeT, y: SizeT) -> usize {
    x as usize + y as usize * DATA.width as usize
}

pub unsafe fn new_level_impl() {
    let mut rng = rand::thread_rng();
    DATA.width = rng.gen_range(crate::MIN_WIDTH..crate::MAX_WIDTH);
    DATA.height = rng.gen_range(crate::MIN_HEIGHT..crate::MAX_HEIGHT);
    DATA.tiles.resize(DATA.width as usize * DATA.height as usize, WALL);
    DATA.tiles.shrink_to_fit();

    digger::dig();

    player::generate_position()
}

#[inline]
pub unsafe fn width_impl() -> SizeT {
    DATA.width
}

#[inline]
pub unsafe fn height_impl() -> SizeT {
    DATA.height
}

pub unsafe fn set_impl(x: SizeT, y: SizeT, value: TileType) {
    DATA.updated = true;
    DATA.tiles[idx(x, y)] = value
}

pub unsafe fn get_impl(x: SizeT, y: SizeT) -> TileType {
    DATA.tiles[idx(x, y)]
}

pub unsafe fn is_updated_impl() -> bool {
    let u = DATA.updated;
    DATA.updated = false;
    u
}
