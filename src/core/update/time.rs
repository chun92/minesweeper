use bevy::prelude::*;
use crate::system::timer::platform::Timer;
use crate::component::number::{self, NumberType, NumberTypeComponent, NumberIndex, NumberIndexComponent};

pub fn update(
    mut q_mines: Query<(&NumberTypeComponent, &NumberIndexComponent, &mut TextureAtlasSprite)>,
    timer: Res<Timer>,
) {
    let time = timer.get_sec() as i32;
    let (first, second, third) = number::get_number_sprites(time);
    for (number_type, number_index, mut sprite) in q_mines.iter_mut() {
        if number_type.0 != NumberType::Time {
            continue;
        }
        if number_type.0 == NumberType::Time && number_index.0 == NumberIndex::First {
            sprite.index = first as usize;
        } else if number_type.0 == NumberType::Time && number_index.0 == NumberIndex::Second {
            sprite.index = second as usize;
        } else if number_type.0 == NumberType::Time && number_index.0 == NumberIndex::Third {
            sprite.index = third as usize;
        }
    }
}

pub fn reset(
    mut timer: ResMut<Timer>,
) {
    timer.reset();
}

pub fn start(
    mut timer: ResMut<Timer>,
) {
    timer.start();
}

pub fn stop(
    mut timer: ResMut<Timer>,
) {
    timer.stop();
}