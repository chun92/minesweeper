use strum_macros::EnumIter;

#[derive(Hash, PartialEq, Eq, EnumIter)]
pub enum TextureType {
    /* bases */
    Cells,
    Smiles,
    Numbers,
    /* frame coners */
    CornerLeftBottom,
    CornerLeftTop,
    CornerRightBottom,
    CornerRightTop,
    CornerLeftUpperTop,
    CornerRightUpperTop,
    /* frame edges */
    EdgeLeft,
    EdgeRight,
    EdgeTop,
    EdgeBottom,
    EdgeLeftUpper,
    EdgeRightUpper,
    EdgeTopUpper,
    Background,
    /* systems */
    Number,
    Smile,
}

impl TextureType {
    pub fn get_path(&self) -> &str {
        match self {
            TextureType::Cells => "cells.png",
            TextureType::Smiles => "smiles.png",
            TextureType::Numbers => "numbers.png",
            TextureType::CornerLeftBottom => "corner_left_bottom.png",
            TextureType::CornerLeftTop => "corner_left_top.png",
            TextureType::CornerRightBottom => "corner_right_bottom.png",
            TextureType::CornerRightTop => "corner_right_top.png",
            TextureType::CornerLeftUpperTop => "corner_left_upper_top.png",
            TextureType::CornerRightUpperTop => "corner_right_upper_top.png",
            TextureType::EdgeLeft => "edge_left.png",
            TextureType::EdgeRight => "edge_right.png",
            TextureType::EdgeTop => "edge_top.png",
            TextureType::EdgeBottom => "edge_bottom.png",
            TextureType::EdgeLeftUpper => "edge_left_upper.png",
            TextureType::EdgeRightUpper => "edge_right_upper.png",
            TextureType::EdgeTopUpper => "edge_top_upper.png",
            TextureType::Background => "background.png",
            TextureType::Number => "number.png",
            TextureType::Smile => "smile.png",
        }
    }

    pub fn get_cell_size(&self) -> (f32, f32, u32, u32, f32, f32) {
        match self {
            TextureType::Cells => (16.0, 16.0, 8, 2, 1.0, 1.0),
            TextureType::Smiles => (24.0, 24.0, 5, 1, 1.0, 0.0),
            TextureType::Numbers => (13.0, 23.0, 12, 1, 1.0, 0.0),
            TextureType::CornerLeftBottom => (12.0, 8.0, 1, 1, 0.0, 0.0),
            TextureType::CornerLeftTop => (12.0, 11.0, 1, 1, 0.0, 0.0),
            TextureType::CornerRightBottom => (8.0, 8.0, 1, 1, 0.0, 0.0),
            TextureType::CornerRightTop => (8.0, 11.0, 1, 1, 0.0, 0.0),
            TextureType::CornerLeftUpperTop => (11.0, 11.0, 1, 1, 0.0, 0.0),
            TextureType::CornerRightUpperTop => (7.0, 11.0, 1, 1, 0.0, 0.0),
            TextureType::EdgeLeft => (12.0, 16.0, 1, 1, 0.0, 0.0),
            TextureType::EdgeRight => (8.0, 16.0, 1, 1, 0.0, 0.0),
            TextureType::EdgeTop => (16.0, 12.0, 1, 1, 0.0, 0.0),
            TextureType::EdgeBottom => (16.0, 8.0, 1, 1, 0.0, 0.0),
            TextureType::EdgeLeftUpper => (11.0, 16.0, 1, 1, 0.0, 0.0),
            TextureType::EdgeRightUpper => (7.0, 16.0, 1, 1, 0.0, 0.0),
            TextureType::EdgeTopUpper => (16.0, 11.0, 1, 1, 0.0, 0.0),
            TextureType::Background => (1.0, 1.0, 1, 1, 0.0, 0.0),
            TextureType::Number => (41.0, 25.0, 1, 1, 0.0, 0.0),
            TextureType::Smile => (26.0, 26.0, 1, 1, 0.0, 0.0),
        }
    }
}

pub enum CellType {
    Hidden = 0,
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

impl CellType {
    pub fn get_revealed_num(num: u32) -> Self {
        match num {
            0 => CellType::Revealed,
            1 => CellType::Revealed1,
            2 => CellType::Revealed2,
            3 => CellType::Revealed3,
            4 => CellType::Revealed4,
            5 => CellType::Revealed5,
            6 => CellType::Revealed6,
            7 => CellType::Revealed7,
            8 => CellType::Revealed8,
            _ => panic!("Invalid revealed num: {}", num),
        }
    }
}