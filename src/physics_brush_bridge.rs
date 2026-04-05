//! Bridge between brush geometry and the avian collider preview cache.
//!
//! Brush entities don't have a standard `Mesh3d` handle — their geometry lives
//! in the runtime-only `BrushMeshCache` component. This module generates
//! colliders from that cache and inserts them into `ColliderPreviewCache` so
//! the physics overlay can render them.

use avian3d::prelude::*;
use bevy::prelude::*;
use jackdaw_avian_integration::ColliderPreviewCache;

use crate::brush::BrushMeshCache;

pub struct PhysicsBrushBridgePlugin;

impl Plugin for PhysicsBrushBridgePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            sync_brush_colliders
                .before(bevy::transform::TransformSystems::Propagate)
                .run_if(in_state(crate::AppState::Editor)),
        );
    }
}

/// Build colliders for brushes whose mesh cache changed (or whose
/// `ColliderConstructor` changed) and stash them in the preview cache.
fn sync_brush_colliders(
    mut cache: ResMut<ColliderPreviewCache>,
    brushes: Query<
        (Entity, &ColliderConstructor, &BrushMeshCache),
        (Or<(Changed<ColliderConstructor>, Changed<BrushMeshCache>)>, Without<Mesh3d>),
    >,
    mut removed: RemovedComponents<BrushMeshCache>,
) {
    for entity in removed.read() {
        cache.remove_entity_collider(entity);
    }

    for (entity, constructor, brush_cache) in &brushes {
        let Some(mesh) = brush_mesh_from_cache(brush_cache) else {
            continue;
        };
        let Some(collider) = Collider::try_from_constructor(constructor.clone(), Some(&mesh)) else {
            continue;
        };
        cache.insert_entity_collider(entity, constructor.clone(), collider);
    }
}

/// Build a triangulated `Mesh` from a `BrushMeshCache`, fan-triangulating each face polygon.
fn brush_mesh_from_cache(cache: &BrushMeshCache) -> Option<Mesh> {
    if cache.vertices.is_empty() {
        return None;
    }
    let positions: Vec<[f32; 3]> = cache.vertices.iter().map(|v| [v.x, v.y, v.z]).collect();
    let mut indices: Vec<u32> = Vec::new();
    for polygon in &cache.face_polygons {
        if polygon.len() >= 3 {
            for i in 1..polygon.len() - 1 {
                indices.push(polygon[0] as u32);
                indices.push(polygon[i] as u32);
                indices.push(polygon[i + 1] as u32);
            }
        }
    }
    let mut m = Mesh::new(
        bevy::mesh::PrimitiveTopology::TriangleList,
        bevy::asset::RenderAssetUsages::default(),
    );
    m.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    m.insert_indices(bevy::mesh::Indices::U32(indices));
    Some(m)
}
