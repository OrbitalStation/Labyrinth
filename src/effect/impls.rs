use super::types::{EffectType, Effect, PlayerEffectsIterator};
use crate::{
    tick::{Tick, Arg},
    creature::CreatureType
};

static mut EFFECTS: Vec <Effect> = Vec::new();

pub fn effect_check_cb(_: Arg) -> Tick {
    unsafe {
        if EFFECTS.is_empty() { return crate::EFFECT_CHECK_IN }
        let mut i = 0;
        while i < EFFECTS.len() {
            if EFFECTS[i].duration == 0 {
                match EFFECTS[i].effect {
                    EffectType::Poison(_) => { },
                    EffectType::Blindness(data) => {
                        EFFECTS[i].obj.set_visibility(data.was_vis);
                    }
                }
                EFFECTS.remove(i);
            } else {
                match EFFECTS[i].effect {
                    EffectType::Poison(data) => {
                        EFFECTS[i].obj.decrease_health(data.power)
                    },
                   EffectType::Blindness(_) => { }
                }
                EFFECTS[i].duration -= crate::EFFECT_CHECK_IN;
                i += 1
            }
        }
    }
    crate::EFFECT_CHECK_IN
}

pub unsafe fn add_effect_impl(effect: Effect) {
    let mut i = 0;
    while i < EFFECTS.len() {
        if EFFECTS[i] == effect {
            EFFECTS[i].add(effect);
            return
        }
        i += 1
    }
    EFFECTS.push(effect);
}

// Implementation is here because `next` needs accesses to `EFFECTS`
impl Iterator for PlayerEffectsIterator {
    type Item = EffectType;

    fn next(&mut self) -> Option <Self::Item> {
        unsafe {
            while self.0 < EFFECTS.len() {
                self.0 += 1;
                if EFFECTS[self.0 - 1].obj.r#type() == CreatureType::Player {
                    return Some(EFFECTS[self.0 - 1].effect)
                }
            }
        }
        None
    }
}

pub fn roman(x: u8) -> &'static str {
    match x {
        1 => "I",
        2 => "II",
        3 => "III",
        4 => "IV",
        5 => "V",
        _ => unimplemented!()
    }
}
