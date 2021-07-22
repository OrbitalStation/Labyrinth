mod types;
mod impls;

use crate::{
    field::SizeT,
    effect::{EffectType, EffectWithDuration, PoisonEffect, BlindnessEffect}
};

macro_rules! impls {
    ($ty:ident where {
        Reappearance = $rep:expr,
        Eat To Explore = $eat:expr,
        Min By Digger = $min:expr,
        Max By Digger = $max:expr,
        Digger Coefficient = $cff:expr,
        Output = { symbol: $sym:expr, color: $col:expr },
        Clustering = $clu:expr, //< 0 - the most, higher - less
        Effects = { $($eff:expr,)* },
    }) => {
        impl crate::plant::types::PlantTrait for crate::plant::types::Impl <{ crate::plant::types::Type::$ty }> {
            #[inline(always)]
            fn reappearance() -> crate::tick::Tick { $rep }

            #[inline(always)]
            fn eat_to_explore() -> u8 { $eat }

            #[inline(always)]
            fn min_by_digger() -> u8 { $min }

            #[inline(always)]
            fn max_by_digger() -> u8 { $max }

            #[inline(always)]
            fn digger_coefficient() -> u8 { $cff }

            #[inline(always)]
            fn output() -> &'static str { concat!("[color=", $col, ']', $sym) }

            #[inline(always)]
            fn clustering() -> u8 { $clu }

            #[inline(never)]
            fn poison_iterator() -> crate::plant::types::PoisonIterator { crate::plant::types::PoisonIterator::new(vec![ $($eff,)* ]) }
        }
    };
}

// Public interface

pub use types::Type;

#[inline]
pub fn add(x: SizeT, y: SizeT, ty: Type) {
    unsafe { impls::add_impl(x, y, ty) }
}

#[inline]
pub fn eat(x: SizeT, y: SizeT) {
    unsafe { impls::eat_impl(x, y) }
}

#[inline]
pub fn type_by_coords(x: SizeT, y: SizeT) -> Type {
    unsafe { impls::type_by_coords_impl(x, y) }
}

// Plants

impls! {
    Blackberry where {
        Reappearance = 5,
        Eat To Explore = 10,
        Min By Digger = 10,
        Max By Digger = 20,
        Digger Coefficient = 1,
        Output = { symbol: 'b', color: "light blue" },
        Clustering = 0,
        Effects = { },
    }
}

impls! {
    Belladonna where {
        Reappearance = 5,
        Eat To Explore = 10,
        Min By Digger = 10,
        Max By Digger = 20,
        Digger Coefficient = 1,
        Output = { symbol: 'b', color: "light red" },
        Clustering = 5, //< 0 - the most, higher - less
        Effects = {
            EffectWithDuration::new(EffectType::Blindness(BlindnessEffect::new(1)), 3),
            EffectWithDuration::new(EffectType::Poison(PoisonEffect::new(1)), 3),
        },
    }
}
