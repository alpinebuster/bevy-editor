use bevy::{
    asset::{AssetPlugin, UnapprovedPathMode},
    image::{ImageAddressMode, ImagePlugin, ImageSamplerDescriptor},
    prelude::*,
};
use jackdaw::EditorPlugin;

fn main() -> AppExit {
    let project_root = jackdaw::project::read_last_project()
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());

    App::new()
        // The default error handler panics, which we never *ever* want to happen to the editor.
        // So let's log an error instead.
        .set_error_handler(bevy::ecs::error::error)
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: project_root.join("assets").to_string_lossy().to_string(),
                    unapproved_path_mode: UnapprovedPathMode::Allow,
                    ..default()
                })
                .set(ImagePlugin {
                    default_sampler: ImageSamplerDescriptor {
                        address_mode_u: ImageAddressMode::Repeat,
                        address_mode_v: ImageAddressMode::Repeat,
                        address_mode_w: ImageAddressMode::Repeat,
                        ..ImageSamplerDescriptor::linear()
                    },
                }),
        )
        .add_plugins(EditorPlugin)
        .add_systems(OnEnter(jackdaw::AppState::Editor), spawn_scene)
        .run()
}

fn spawn_scene(mut commands: Commands) {
    commands.queue(|world: &mut World| {
        jackdaw::scene_io::spawn_default_lighting(world);
    });
}
