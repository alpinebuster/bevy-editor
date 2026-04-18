use std::sync::Arc;

use bevy::{
    prelude::*,
    render::{
        RenderPlugin,
        settings::{RenderCreation, WgpuSettings},
    },
    winit::WinitPlugin,
};
use jackdaw::prelude::*;
use jackdaw_api::prelude::*;

fn headless_app() -> App {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(RenderPlugin {
                render_creation: RenderCreation::Automatic(WgpuSettings {
                    backends: None,
                    ..default()
                }),
                ..default()
            })
            .disable::<WinitPlugin>(),
    )
    .add_plugins(EditorPlugin);
    app
}

#[test]
fn smoke_test_headless_update() {
    let mut app = headless_app();
    app.finish();

    for _ in 0..10 {
        app.update();
    }
}

#[test]
fn can_run_extension() {
    let mut app = headless_app();
    jackdaw_api::register_extension(&mut app, "sample_extension", || Box::new(SampleExtension));
    app.finish();
}

pub struct SampleExtension;

impl JackdawExtension for SampleExtension {
    fn name(&self) -> &str {
        "sample"
    }

    fn register_input_contexts(&self, app: &mut App) {
        app.add_input_context::<SampleContext>();
    }

    fn register(&self, ctx: &mut ExtensionContext) {
        ctx.register_window(WindowDescriptor {
            id: "sample.hello".into(),
            name: "Hello Extension".into(),
            icon: None,
            default_area: None,
            priority: None,
            build: Arc::new(build_hello_panel),
        });

        ctx.register_operator::<HelloOp>();
        ctx.register_operator::<HelloTimeOp>();

        ctx.spawn((
            SampleContext,
            actions!(SampleContext[
                (Action::<HelloOp>::new(), bindings![KeyCode::F9]),
                (Action::<HelloTimeOp>::new(), bindings![KeyCode::F10]),
            ]),
        ));
    }
}

fn build_hello_panel(world: &mut World, parent: Entity) {
    world.spawn((ChildOf(parent), Text::new("Hello from an extension!")));
}

#[derive(Component, Default)]
pub struct SampleContext;

#[operator(
    id = "sample.hello",
    label = "Hello",
    description = "Logs a hello message",
    name = "HelloOp"
)]
fn hello_op() -> OperatorResult {
    info!("Hello from the sample extension operator!");
    OperatorResult::Finished
}

/// Availability check for [`HelloTimeOp`]. Bevy systems returning
/// `bool` can inject any `SystemParam`; here we read `Time` and only
/// allow the operator to run while the clock is advancing.
fn time_is_running(time: Res<Time>) -> bool {
    time.delta_secs() > 0.0
}

#[operator(
    id = "sample.hello_time",
    label = "Hello (Time)",
    description = "Logs a hello message, but only while time is advancing",
    is_available = time_is_running,
    name = "HelloTimeOp"
)]
fn hello_time_op(time: Res<Time>) -> OperatorResult {
    info!(
        "Hello at frame delta {:.3}s from the sample extension",
        time.delta_secs()
    );
    OperatorResult::Finished
}
