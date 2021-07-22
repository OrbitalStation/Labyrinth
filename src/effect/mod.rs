mod types;
mod impls;

use crate::tick;

// Public interface

pub use types::{EffectType, EffectWithDuration, PoisonEffect, BlindnessEffect, Effect, PlayerEffectsIterator};

pub fn init() {
    tick::add(impls::effect_check_cb, tick::NULLARG, crate::EFFECT_CHECK_IN)
}

#[inline]
pub fn add_effect(effect: Effect) {
    unsafe { impls::add_effect_impl(effect) }
}

#[inline]
pub fn player_effects() -> PlayerEffectsIterator {
    PlayerEffectsIterator { 0: 0 }
}
