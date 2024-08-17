use bevy::prelude::*;

use crate::animation::components::{AnimationIndices, AnimationTimer};

use super::components::*;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    query: Query<
        (Entity, &Pole, &Transform),
        Without<AnimationIndices>,
    >,
) {
    if let Ok((e, p, t)) = query.get_single() {
        info!("Setup Pole animation on {:?}", e);
        let texture = asset_server.load("sprites/too-bearded/pole.png");
        let layout = TextureAtlasLayout::from_grid(
            UVec2::splat(24),
            8,
            1,
            None,
            None,
        );
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let animation_indices = AnimationIndices::from(p);
        commands.entity(e).despawn_descendants().insert((
            Name::new("Pole"),
            SpriteBundle {
                transform: t.to_owned(),
                texture,
                ..default()
            },
            TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
            animation_indices,
            AnimationTimer::from(p),
        ));
    }
}
