use crate::field::SizeT;
use bear_lib_terminal::terminal;

pub use crate::creature::Health;

// Private

use bitflags::bitflags;

bitflags! {
    pub struct Flags: u8 {
        const SHOW_HEALTH_PLUS   = 0b00001;
        const SHOW_HEALTH_MINUS  = 0b00010;
        const SHOW_SATIETY_PLUS  = 0b00100;
        const SHOW_SATIETY_MINUS = 0b01000;
        const VISIBILITY_CHANGED = 0b10000;
    }
}

pub struct Data {
    pub x: SizeT, //< X position
    pub y: SizeT, //< Y position
    pub visibility: SizeT, //< On how many cells does player see
    pub health: u8,  //< How many HP does player have
    pub satiety: u8, //< How strong is player's satiety (100 - most, 0 - least)
    pub hunger_counter: u8,
    pub flags: Flags
}

pub struct VisibilityLevel {
    cur: SizeT,
    end: SizeT,
    vis: SizeT,
    rev: bool,
    was: bool
}

impl VisibilityLevel {
    pub fn new(vis: SizeT, start: SizeT, end: SizeT) -> Self {
        Self {
            cur: start,
            end,
            vis,
            rev: true,
            was: true
        }
    }
}

impl Iterator for VisibilityLevel {
    type Item = SizeT;

    fn next(&mut self) -> Option <Self::Item> {
        if self.cur == self.end { return None }

        let r = Some(if self.rev { self.cur } else { self.end - self.cur - 1 });

        self.cur += 1;
        if self.cur == self.vis && self.rev {
            return Some(if self.was {
                self.was = false;
                self.cur -= 1;
                self.vis - 1
            } else {
                self.rev = false;
                self.cur += 1;
                self.vis
            })
        }

        r
    }
}

// Public

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left
}

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum Err {
    Ok,
    CannotMove
}

impl Err {
    pub fn show(self) {
        terminal::print_xy(0, 0, format!("[color=red]{}", match self {
            Self::Ok => "",
            Self::CannotMove =>  "Cannot move to that direction!"
        }).as_str());
    }
}
