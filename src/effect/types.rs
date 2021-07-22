use crate::{
    creature::{Health, Creature},
    tick::Tick
};
use super::impls;
// Player iterator

pub struct PlayerEffectsIterator(pub(crate) usize);

// Effect

#[derive(Copy, Clone, Eq)]
#[repr(u8)]
pub enum EffectType {
    Poison(PoisonEffect),
    Blindness(BlindnessEffect)
}

impl EffectType {
    pub fn as_str(self) -> String {
        format!("[color=orange]{}", match self {
            Self::Poison(data) => {
                format!("Poison {}", impls::roman(data.power))
            },
            Self::Blindness(data) => {
                format!("Blindness {}", impls::roman(data.current))
            }
        })
    }

    pub fn add(&mut self, rhs: EffectType) {
        match self {
            EffectType::Poison(data) => {
                match rhs {
                    EffectType::Poison(rdata) => {
                        data.power = data.power.max(rdata.power)
                    },
                    _ => unimplemented!()
                }
            },
            EffectType::Blindness(data) => {
                match rhs {
                    EffectType::Blindness(rdata) => {
                        data.current = data.current.min(rdata.current)
                    },
                    _ => unimplemented!()
                }
            }
        }
    }

    pub fn as_u8(self) -> u8 {
        match self {
            EffectType::Poison(_)    => { 0 }
            EffectType::Blindness(_) => { 1 }
        }
    }
}

impl PartialEq for EffectType {
    fn eq(&self, other: &Self) -> bool {
        self.as_u8() == other.as_u8()
    }
}

#[derive(Copy, Clone)]
pub struct EffectWithDuration {
    pub ty: EffectType,
    pub duration: Tick
}

impl EffectWithDuration {
    pub fn new(ty: EffectType, duration: Tick) -> Self {
        Self { ty, duration }
    }
}

#[derive(Copy, Clone, Eq)]
pub struct Effect {
    pub effect: EffectType,
    pub duration: Tick,
    pub obj: Creature
}

impl Effect {
    pub fn add(&mut self, rhs: Effect) {
        if *self != rhs { panic!("Cannot add effects of different objs!") }
        self.effect.add(rhs.effect);
        self.duration += rhs.duration
    }
}

impl PartialEq for Effect {
    fn eq(&self, other: &Self) -> bool {
        self.effect == other.effect && self.obj == other.obj
    }
}

// Poison

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PoisonEffect {
    pub power: Health
}

impl PoisonEffect {
    #[inline(always)]
    pub const fn new(power: Health) -> Self {
        Self { power }
    }
}

// Blindness

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BlindnessEffect {
    pub was_vis: u8,
    pub current: u8
}

impl BlindnessEffect {
    #[inline(always)]
    pub const fn new(current: u8) -> Self {
        Self { was_vis: 0, current }
    }
}
