mod textures;
mod world;

use geometry_2d::geometry::Position;
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
            world: World::new(2, ctx)
        })
    }

    pub fn get_world(&mut self) -> &mut World {
        &mut self.world
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        let mut canvas:Canvas = Canvas::from_frame(
            _ctx, 
            Color::from_rgb(0, 0, 0)
        );

        self.get_world().render(&mut canvas, Position::new(0.0, 0.0));

        canvas.finish(_ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let mode = conf::WindowMode::default();
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
    state.get_world().set_terrain(Position::new(0.0, 0.0), TerrainType::Grass);

    event::run(ctx, event_loop, state)
}
