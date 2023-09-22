use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct WindowSize(pub Vec2);

// pub fn resize_window(
//     grid: ResMut<Grid>,
//     mut window_size: ResMut<WindowSize>,
// ) {
//     let width = grid.width as f32 * asset::texture_type::TextureAtlasType::Cells.get_cell_size().0;
//     let height = grid.height as f32 * asset::texture_type::TextureAtlasType::Cells.get_cell_size().1;
//     window_size.0 = Vec2::new(width, height);
    
// }