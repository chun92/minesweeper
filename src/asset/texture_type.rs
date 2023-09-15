#[derive(Hash, PartialEq, Eq)]
pub enum TextureAtlasType {
    Cells,
    Smiles,
    Times,
}

impl TextureAtlasType {
    pub fn get_path(&self) -> &str {
        match self {
            TextureAtlasType::Cells => "cells.png",
            TextureAtlasType::Smiles => "smiles.png",
            TextureAtlasType::Times => "times.png",
        }
    }

    pub fn get_cell_size(&self) -> (f32, f32, u32, u32, f32, f32) {
        match self {
            TextureAtlasType::Cells => (16.0, 16.0, 8, 2, 1.0, 1.0),
            TextureAtlasType::Smiles => (24.0, 24.0, 5, 1, 1.0, 0.0),
            TextureAtlasType::Times => (13.0, 23.0, 12, 1, 1.0, 0.0),
        }
    }
}

pub enum CellType {
    Normal = 0,
    Revealed = 1,
    Flag = 2,
    Question = 3,
    QuestionClicked = 4,
    Mine = 5,
    MineBombed = 6,
    WrongMine = 7,
    Revealed1 = 8,
    Revealed2 = 9,
    Revealed3 = 10,
    Revealed4 = 11,
    Revealed5 = 12,
    Revealed6 = 13,
    Revealed7 = 14,
    Revealed8 = 15,
}