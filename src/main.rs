use amethyst::{
    input::is_key_down, prelude::*, utils::application_root_dir, window::WindowBundle,
    winit::VirtualKeyCode,
utils::{
    fps_counter::{FpsCounter, FpsCounterBundle},
}};


struct ExampleState;

impl SimpleState for ExampleState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        println!("Start")
    }
    fn on_stop(&mut self,_data: StateData<'_, GameData<'_, '_>>) {
        println!("Stop")
    }
    fn handle_event(
        &mut self,
        _: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
            Trans::None
    }
    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        println!("Beep");
        Trans::None
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config/display.ron");

    let assets_dir = app_root.join("assets/");

    let game_data = GameDataBuilder::default()
    .with_bundle(WindowBundle::from_config_path(display_config_path))?
    .with_bundle(FpsCounterBundle::default())?;


    let mut game = Application::new(assets_dir, ExampleState, game_data)?;
    game.run();

    Ok(())
}