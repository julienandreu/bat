use bevy::prelude::*;

use crate::animation::components::{AnimationIndices, AnimationTimer};

use super::components::*;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    query: Query<
        (Entity, &Julien, &Transform),
        Without<AnimationIndices>,
    >,
) {
    if let Ok((e, j, t)) = query.get_single() {
        info!("Setup Julien animation on {:?}", e);
        let texture =
            asset_server.load("sprites/too-bearded/julien.png");
        let layout = TextureAtlasLayout::from_grid(
            UVec2::splat(24),
            7,
            3,
            None,
            None,
        );
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let animation_indices = AnimationIndices::from(j);
        commands.entity(e).despawn_descendants().insert((
            Name::new("Julien"),
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
            AnimationTimer::from(j),
        ));
    }
}

pub fn render(
    time: Res<Time>,
    mut query: Query<
        (&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas),
        With<Julien>,
    >,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}
