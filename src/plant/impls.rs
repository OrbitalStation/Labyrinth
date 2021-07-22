// Uses

use crate::{
    tick::{self, Tick},
    field::{self, SizeT},
    creature::Creature,
    tile::*
};
use super::types::{Data, Type};

// Static

static mut DATA: Vec <Data> = Vec::new();

// Fns

fn regrow_cb(arg: tick::Arg) -> Tick {
    unsafe {
        let me = &mut *(arg as *mut Data);
        let cl = field::get(me.x, me.y);
        if cl == PLAYER {
            return crate::PLANT_CHECK_CELL_FREE_IN
        } else if cl == EMPTY {
            field::set(me.x, me.y, PLANT)
        } else {
            DATA.remove(crate::find_by_coords_index(&DATA, me.x, me.y).unwrap());
        }
    }
    0
}

pub unsafe fn add_impl(x: SizeT, y: SizeT, ty: Type) {
    DATA.push(Data { x, y, ty })
}

pub unsafe fn eat_impl(x: SizeT, y: SizeT) {
    let plant = crate::find_by_coords(&mut DATA, x, y).expect("There's no plant with such coords!");
    field::set(plant.x, plant.y, EMPTY);
    let creature = Creature::from_coords(plant.x, plant.y);
    // if plant.ty.poisonous() {
    //     creature.add_effect(effect::EffectType::Poison(effect::PoisonEffect::new(plant.ty.poison_power())), plant.ty.poison_duration())
    // } else {
    //     player::increase_satiety(plant.ty.amount())
    // }
    for i in plant.ty.poison_iterator() {
        creature.add_effect(i.ty, i.duration)
    }
    tick::add(regrow_cb, plant as *mut Data as tick::Arg, plant.ty.reappearance());
}

pub unsafe fn type_by_coords_impl(x: SizeT, y: SizeT) -> Type {
    crate::find_by_coords(&mut DATA, x, y).expect("There's no plant with such coords!").ty
}
