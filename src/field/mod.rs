mod types;
mod impls;
mod digger;

use crate::tile::*;

// Public interface

pub use types::SizeT;

#[inline]
pub fn new_level() {
    unsafe { impls::new_level_impl() }
}

#[inline]
pub fn width() -> SizeT {
    unsafe { impls::width_impl() }
}

#[inline]
pub fn height() -> SizeT {
    unsafe { impls::height_impl() }
}

#[inline]
pub fn set(x: SizeT, y: SizeT, value: TileType) {
    unsafe { impls::set_impl(x, y, value) }
}

#[inline]
pub fn get(x: SizeT, y: SizeT) -> TileType {
    unsafe { impls::get_impl(x, y) }
}

#[inline]
pub fn is_updated() -> bool {
    unsafe { impls::is_updated_impl() }
}
