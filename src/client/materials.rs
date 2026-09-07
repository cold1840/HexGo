use bevy::prelude::*;

const fn srgb(hex: u32) -> Color {
    Color::srgb(
        ((hex >> 16) & 0xFF) as f32 / 255.0,
        ((hex >> 8) & 0xFF) as f32 / 255.0,
        (hex & 0xFF) as f32 / 255.0,
    )
}

const fn srgba(hex: u32) -> Color {
    Color::srgba(
        ((hex >> 24) & 0xFF) as f32 / 255.0,
        ((hex >> 16) & 0xFF) as f32 / 255.0,
        ((hex >> 8) & 0xFF) as f32 / 255.0,
        (hex & 0xFF) as f32 / 255.0,
    )
}

pub const BLACK_STONE: Color = srgb(0x0E0D0B);
pub const WHITE_STONE: Color = srgb(0xFAF2E3);

pub const BLACK_STONE_PREVIEW: Color = srgba(0x0E0D0B80);
pub const WHITE_STONE_PREVIEW: Color = srgba(0xFAF2E3B8);

#[derive(Resource)]
pub struct StoneMaterials {
    pub black: Handle<ColorMaterial>,
    pub white: Handle<ColorMaterial>,
    pub preview_black: Handle<ColorMaterial>,
    pub preview_white: Handle<ColorMaterial>,
}

pub fn setup_stone_materials(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.insert_resource(StoneMaterials {
        black: materials.add(BLACK_STONE),
        white: materials.add(WHITE_STONE),
        preview_black: materials.add(BLACK_STONE_PREVIEW),
        preview_white: materials.add(WHITE_STONE_PREVIEW),
    });
}
