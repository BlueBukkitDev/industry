mod textures;
mod world;

use ggez::{GameResult, event, conf, Context};
use ggez::graphics::{Color, Canvas, DrawParam};
use std::{path, env};
use textures::TerrainTex;

pub struct MainState {
    textures:TerrainTex
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
        Ok(MainState{
            textures: TerrainTex::init(ctx)
        })
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

        canvas.draw(self.textures.terrain_grass(), DrawParam::new());

        //let my_dest = glam::vec2(13.0, 37.0);
        //canvas.draw(&image, DrawParam::default().dest(my_dest));

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
    let state:MainState = MainState::new(&mut ctx)?;

    event::run(ctx, event_loop, state)
}
