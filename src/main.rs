use amethyst::{
    core::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

#[macro_use]
extern crate approx;

mod components;
mod resources;
mod state;
mod systems;
mod utils;

use state::demo::DemoState;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    // View config paths
    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let display_config_path = app_root.join("config").join("display.ron");
    let binding_path = app_root.join("config").join("bindings.ron");

    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

    let mut world = World::empty();

    // App setup
    let mut game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?.with_clear([0, 0, 0, 1]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(input_bundle)?
        // Systems
        .with(
            systems::orders::order_creator::OrderCreatorSystem::default(),
            "OrderCreatorSystem",
            &[],
        )
        .with(
            systems::orders::order_executor::OrderExecutorSystem::default(),
            "OrderExecutorSystem",
            &["OrderCreatorSystem"],
        )
        .with_system_desc(
            systems::rendering::projectiles::ProjectileSystemDesc::default(),
            "ProjectileRenderingSystem",
            &["OrderExecutorSystem"],
        )
        .with_bundle(TransformBundle::new())?;

    // let res = game_data.build(&mut world);

    // #TODO fix state
    let mut game = Application::new(assets_dir, DemoState::default(), game_data)?;
    game.run();

    Ok(())
}
