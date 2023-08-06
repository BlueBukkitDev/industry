mod textures;
mod world;
mod client;

use geometry_2d::geometry::Position_i32;
use ggez::input::keyboard::{KeyInput, self};
use ggez::{GameResult, event, conf, Context};
use ggez::graphics::{Color, Canvas};
use world::{World, TerrainType};
use std::{path, env};

pub struct MainState {
    world:World
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
        Ok(MainState{
            world: World::new(2, ctx, 40)
        })
    }

    pub fn get_world(&mut self) -> &mut World {
        &mut self.world
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.world.update();
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        let mut canvas:Canvas = Canvas::from_frame(
            _ctx, 
            Color::from_rgb(0, 0, 0)
        );

        self.get_world().render(&mut canvas, Position_i32::new(0, 0));

        _ctx.gfx.set_window_title(&format!(
            "Industry Sim Game [FPS {}]", _ctx.time.fps()
        ));

        canvas.finish(_ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        if input.keycode == Some(keyboard::KeyCode::Escape) {
            _ctx.request_quit();
        }else if input.keycode == Some(keyboard::KeyCode::A) {
            self.world.get_view().pan(0);
        }else if input.keycode == Some(keyboard::KeyCode::W) {
            self.world.get_view().pan(1);
        }else if input.keycode == Some(keyboard::KeyCode::D) {
            self.world.get_view().pan(2);
        }else if input.keycode == Some(keyboard::KeyCode::S) {
            self.world.get_view().pan(3);
        }
        Ok(())
    }
}

pub fn main() -> GameResult {
    let mode = conf::WindowMode::default().fullscreen_type(conf::FullscreenType::Desktop);
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("res");
        path
    } else {
        path::PathBuf::from("./res")
    };

    let cb:ggez::ContextBuilder = ggez::ContextBuilder::new("Industry", "Blue Dev").window_mode(mode).add_resource_path(resource_dir);
    let (mut ctx, event_loop) = cb.build()?;
    let mut state:MainState = MainState::new(&mut ctx)?;
    state.get_world().set_terrain(Position_i32::new(0, 0), TerrainType::Grass);

    event::run(ctx, event_loop, state)
}
