#[derive(Hash, PartialEq, Eq)]
pub enum TextureAtlasType {
    Bombs,
    Times,
    Smiles,
    Numbers
}

impl TextureAtlasType {
    pub fn get_path(&self) -> &str {
        match self {
            TextureAtlasType::Bombs => "bombs.png",
            TextureAtlasType::Times => "times.png",
            TextureAtlasType::Smiles => "smiles.png",
            TextureAtlasType::Numbers => "numbers.png",
        }
    }

    pub fn get_cell_size(&self) -> (f32, f32, u32, u32, f32, f32) {
        match self {
            TextureAtlasType::Bombs => (16.0, 16.0, 8, 1, 1.0, 0.0),
            TextureAtlasType::Times => (16.0, 16.0, 8, 1, 1.0, 0.0),
            TextureAtlasType::Smiles => (24.0, 24.0, 5, 1, 1.0, 0.0),
            TextureAtlasType::Numbers => (13.0, 23.0, 12, 1, 1.0, 0.0),
        }
    }
}