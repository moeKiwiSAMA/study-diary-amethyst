use amethyst::{
    input::is_key_down, prelude::*, utils::application_root_dir, window::WindowBundle,
    winit::VirtualKeyCode,
    core::{timing::Time, transform::Transform},
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
utils::{
    fps_counter::{FpsCounter, FpsCounterBundle},
}};


struct ExampleState {
    example_spawn_timer: Option<f32>,
}

impl SimpleState for ExampleState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        println!("Start")
    }
    fn on_stop(&mut self,_data: StateData<'_, GameData<'_, '_>>) {
        println!("Stop")
    }
    fn handle_event(&mut self, _: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        Trans::None

    }
    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        Trans::None
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config/display.ron");

    let assets_dir = app_root.join("assets/");

    let game_data = GameDataBuilder::default()
    .with_bundle(RenderingBundle::<DefaultBackend>::new()
        .with_plugin(
            RenderToWindow::from_config_path(display_config_path)
                .with_clear([0.34, 0.36, 0.52, 1.0]),
        )
        .with_plugin(RenderFlat2D::default()))?;


    let mut game = Application::new(assets_dir, ExampleState{example_spawn_timer:Some(1.0)}, game_data)?;
    game.run();

    Ok(())
}