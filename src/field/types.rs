use crate::tile::TileType;

pub type SizeT = u8;

#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum Step {
    Done,
    Stop
}

pub struct Digger {
    pub x: SizeT,
    pub y: SizeT
}

pub struct Data {
    pub width: SizeT,
    pub height: SizeT,
    pub tiles: Vec <TileType>,
    pub updated: bool
}

#[derive(Copy, Clone)]
pub struct DiggerPlant {
    pub remain: u8,
    pub wait: u8
}
