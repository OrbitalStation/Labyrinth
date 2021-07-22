#![allow(incomplete_features)]
#![feature(stmt_expr_attributes)]
#![feature(const_panic)]
#![feature(array_methods)]
#![feature(const_generics)]

pub mod tile;
pub mod tick;
pub mod book;
pub mod field;
//pub mod animal;
pub mod plant;
pub mod player;
pub mod effect;
pub mod creature;

use bear_lib_terminal::terminal;
use field::SizeT;
use tick::Tick;

const MIN_WIDTH: SizeT = 100; //< Minimal width of field
const MAX_WIDTH: SizeT = 200; //< Maximal width of field

const MIN_HEIGHT: SizeT = 100; //< Minimal height of field
const MAX_HEIGHT: SizeT = 200; //< Maximal height of field

const MIN_WALL_SHARES: u8 = 2; //< Minimal shares of walls in process of choosing next tile by digger
const MAX_WALL_SHARES: u8 = 7; //< Maximal shares of walls in process of choosing next tile by digger

const MIN_EMPTY_SHARES: u8 = 1; //< Minimal shares of empty tiles in process of choosing next tile by digger
const MAX_EMPTY_SHARES: u8 = 6; //< Maximal shares of empty tiles in process of choosing next tile by digger

const MIN_DIGGER_PLACING_OFFSET: u8 = 5; //< Minimal index of tile can be chosen to place digger
const MAX_DIGGER_PLACING_OFFSET: u8 = 5; //< Maximal index of tile can be chosen to place digger (FIELD.width\height - MAX_DIGGER_PLACING_OFFSET)

const PLANT_CHECK_CELL_FREE_IN: Tick = 3; //< How often does plant check that cell it is on is free

const DIGGER_EMPTY_COEFFICIENT: u8 = 8; //< Chance that cell will be empty

const DIGGER_PLANT_NON_ZERO_FIND_END_IN: u8 = 50; //< After N fails, treat as plants are over

const DIGGERS_COUNT: u8 = 6; //< Number of diggers; Cannot be changed because it is most optimal value

const PLAYER_START_VISIBILITY: SizeT = 10; //< Visibility player starts with
const PLAYER_MAX_VISIBILITY: SizeT = 10;   //< Max player's visibility

const PLAYER_INTERFACE_START: i32 = 15; //< Start of interface X in terminal
const PLAYER_INTERFACE_HEALTH_ROW: i32 = 1; //< Terminal row where health is located
const PLAYER_INTERFACE_SATIETY_ROW: i32 = 3; //< Terminal row where satiety is located
const PLAYER_EFFECTS_START_ROW: i32 = 5; //< Terminal row where effects started

const PLAYER_HAND_FIND_ON_FAILURE_N_TIMES: u8 = 50; //< N in case that if random-based strategy of finding place for player failed N times, then use hand-based strategy

const PLAYER_MAX_HEALTH: u8 = 100; //< Maximal health value
const PLAYER_MAX_SATIETY: u8 = 100; //< Maximal satiety value

const PLAYER_SATIETY_COUNT: u8 = 10; //< How many actions does player need to waste one satiety point
const PLAYER_SATIETY_IN:    Tick =  3; //< In how many ticks player loses his satiety

const TERMINAL_WIDTH: u32 = 80;
const TERMINAL_HEIGHT: u32 = 30;

const EFFECT_CHECK_IN: Tick = 1; //< In how many ticks effects are checked

///
/// File format:
/// 1st byte <- number of supported plants(for compatibility with old versions)
/// 2nd byte <- number of bytes in data(for compatibility with old versions)
/// 3rd..sizeof(book::herbarium::ActionsToRead) + 2 bytes <- data
/// And so on...
///
const BOOK_HERBARIUM_PATH: &'static str = "data/herbarium.dat";
const BOOK_HERBARIUM_SAVE_IN: Tick = 100; //< How often is herbarium saved

pub fn init() {
    terminal::open("Labyrinth", TERMINAL_WIDTH, TERMINAL_HEIGHT);
    terminal::set(terminal::config::Window::empty().fullscreen(true).resizeable(true));
    terminal::refresh();

    player::init();
    field::new_level();
    effect::init();

    unsafe {
        book::herbarium::HERBARIUM.init()
    }
}

pub fn safe_exit() -> ! {
    unsafe { book::herbarium::HERBARIUM.save() }
    std::process::exit(0);
}

pub trait Findable {
    fn x(&self) -> SizeT;
    fn y(&self) -> SizeT;
}

pub fn find_by_coords <T> (c: &'static mut Vec <T>, x: SizeT, y: SizeT) -> Option <&'static mut T> where T: Findable {
    for i in c.iter_mut() {
        if i.x() == x && i.y() == y { return Some(i) }
    }
    None
}

pub fn find_by_coords_index <T> (c: &'static Vec <T>, x: SizeT, y: SizeT) -> Option <usize> where T: Findable {
    for (idx, i) in c.iter().enumerate() {
        if i.x() == x && i.y() == y { return Some(idx) }
    }
    None
}
