use bevy::{prelude::*, utils::HashMap};

use super::texture_type::TextureAtlasType;

#[derive(Resource, Default)]
pub struct TextureAtlasResource {
    pub handles: HashMap<TextureAtlasType, Handle<TextureAtlas>>,
}

struct AtlasOptions {
    width: f32,
    height: f32,
    columns: u32,
    rows: u32,
    horizontal_space: f32,
    vertical_gap: f32,
}

impl Default for AtlasOptions {
    fn default() -> Self {
        Self {
            width: 1.0,
            height: 1.0,
            columns: 1,
            rows: 1,
            horizontal_space: 0.0,
            vertical_gap: 0.0,
        }
    }
}

fn load_texture_as_atlas(handle: Handle<Image>,
    option: AtlasOptions) -> TextureAtlas {
    let mut texture_atlas = TextureAtlas::new_empty(handle, Vec2::new(option.width, option.height));

    for y in 0..option.rows {
        for x in 0..option.columns {
            let rect_min = Vec2::new(
                x as f32 * (option.width + option.horizontal_space),
                y as f32 * (option.height + option.vertical_gap),
            );
            let rect_max = rect_min + Vec2::new(option.width, option.height);

            let rect = Rect {
                min: rect_min,
                max: rect_max,
            };
            texture_atlas.add_texture(rect);
        }
    }
    texture_atlas
}

fn load_texture(
    atlas_type: TextureAtlasType,
    option: AtlasOptions,
    asset_server: &AssetServer,
    texture_atlases: &mut Assets<TextureAtlas>,
    texture_atlas_resource: &mut TextureAtlasResource
) {
    let texture_handle = asset_server.load(atlas_type.get_path());
    let texture_atlas = load_texture_as_atlas(texture_handle, option);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    texture_atlas_resource.handles.insert(atlas_type, texture_atlas_handle);
}

fn load_texture_with_type(
    asset_server: &AssetServer,
    texture_atlases: &mut Assets<TextureAtlas>,
    texture_atlas_resource: &mut TextureAtlasResource,
    atlas_type: TextureAtlasType
) {
    let width = atlas_type.get_cell_size().0;
    let height = atlas_type.get_cell_size().1;
    let columns = atlas_type.get_cell_size().2;
    let rows = atlas_type.get_cell_size().3;
    let horizontal_space = atlas_type.get_cell_size().4;
    let vertical_gap = atlas_type.get_cell_size().5;
    load_texture(atlas_type, 
        AtlasOptions { width, height, columns, rows, horizontal_space, vertical_gap}, 
        &asset_server, 
        texture_atlases, 
        texture_atlas_resource);
}

pub fn setup(
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut texture_atlas_resource: ResMut<TextureAtlasResource>,
) {
    load_texture_with_type(&asset_server, &mut texture_atlases, &mut texture_atlas_resource, TextureAtlasType::Bombs);
    load_texture_with_type(&asset_server, &mut texture_atlases, &mut texture_atlas_resource, TextureAtlasType::Times);
    load_texture_with_type(&asset_server, &mut texture_atlases, &mut texture_atlas_resource, TextureAtlasType::Smiles);
    load_texture_with_type(&asset_server, &mut texture_atlases, &mut texture_atlas_resource, TextureAtlasType::Numbers);
}