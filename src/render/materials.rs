use std::marker::PhantomData;

use bevy::prelude::*;

use crate::render::prelude::*;

/// Marker [`Component`] for mesh shadows.
#[derive(Component)]
pub(crate) struct MeshShadow;

/// Shadow [`Mesh`] and related data for `T`.
///
/// The size of the [`Mesh`] is meant to be derived from [`CollisionDataCache`](crate::physics::prelude::CollisionDataCache).
#[derive(Resource, Default)]
pub(crate) struct ShadowMesh<T>
where
    T: Visible,
{
    pub(crate) mesh: Handle<Mesh>,
    pub(crate) y_offset: f32,
    pub(crate) _phantom: PhantomData<T>,
}
