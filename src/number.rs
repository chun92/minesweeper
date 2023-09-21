use bevy::prelude::*;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum NumberType {
    MineCount,
    Time
}

#[derive(PartialEq, Eq)]
pub enum NumberIndex {
    First,
    Second,
    Third
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NumberSprite {
    One = 0,
    Two = 1,
    Three = 2,
    Four = 3,
    Five = 4,
    Six = 5,
    Seven = 6,
    Eight = 7,
    Nine = 8,
    Zero = 9,
    Minus = 10,
    Empty = 11,
}

fn get_number_sprite(number: i32) -> NumberSprite {
    match number {
        1 => NumberSprite::One,
        2 => NumberSprite::Two,
        3 => NumberSprite::Three,
        4 => NumberSprite::Four,
        5 => NumberSprite::Five,
        6 => NumberSprite::Six,
        7 => NumberSprite::Seven,
        8 => NumberSprite::Eight,
        9 => NumberSprite::Nine,
        0 => NumberSprite::Zero,
        _ => NumberSprite::Minus,
    }
}

pub fn get_number_sprites(number: i32) -> (NumberSprite, NumberSprite, NumberSprite) {
    let mut number = number;
    let mut first = NumberSprite::Zero;
    let mut second = NumberSprite::Zero;
    let third;
    if number < 0 {
        number = -number;
        if number > 99 {
            number = 99;
        }
        if number > 9 {
            first = NumberSprite::Minus;
        } else {
            second = NumberSprite::Minus;
        }
    }

    if number > 999 {
        number = 999;
    }
    if number > 99 {
        first = get_number_sprite(number / 100);
        number = number % 100;
    }
    if number > 9 {
        second = get_number_sprite(number / 10);
        number = number % 10;
    }
    third = get_number_sprite(number);
    (first, second, third)
}

pub fn get_number_sprites_from_f32(number: f32) -> (NumberSprite, NumberSprite, NumberSprite) {
    get_number_sprites(number as i32)
}

#[derive(Component)]
pub struct NumberTypeComponent(pub NumberType);
impl NumberTypeComponent {
    pub fn new(number_type: NumberType) -> Self {
        Self(number_type)
    }
}

#[derive(Component)]
pub struct NumberIndexComponent(pub NumberIndex);
impl NumberIndexComponent {
    pub fn new(number_index: NumberIndex) -> Self {
        Self(number_index)
    }
}

#[derive(Resource)]
pub struct TotalMine(pub u32);

impl Default for TotalMine {
    fn default() -> Self {
        Self(99)
    }
}

impl TotalMine {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn init(&mut self, num_mines: u32) {
        self.0 = num_mines;
    }
}

#[derive(Resource, Default)]
pub struct RemainingMine(pub i32);

#[derive(Resource, Default)]
pub struct Time(pub f32);