mod textures;
mod world;
mod client;

use geometry_2d::geometry::Position_i32;
use ggez::event::MouseButton;
use ggez::input::keyboard::{KeyInput, self};
use ggez::{GameResult, event, conf, Context};
use ggez::graphics::{Color, Canvas};
use world::{World, TerrainType};
use std::{path, env};

pub struct MainState {
    world:World,
    mouse_down:bool,
    mouse_start:(f32, f32)
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
        Ok(MainState{
            world: World::new(2, ctx, 40),
            mouse_down: false,
            mouse_start:(0.0, 0.0)
        })
    }

    pub fn get_world(&mut self) -> &mut World {
        &mut self.world
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if _ctx.keyboard.is_key_pressed(keyboard::KeyCode::A) {
            self.world.get_view().pan(0, 10);
        }
        if _ctx.keyboard.is_key_pressed(keyboard::KeyCode::W) {
            self.world.get_view().pan(1, 10);
        }
        if _ctx.keyboard.is_key_pressed(keyboard::KeyCode::D) {
            self.world.get_view().pan(2, 10);
        }
        if _ctx.keyboard.is_key_pressed(keyboard::KeyCode::S) {
            self.world.get_view().pan(3, 10);
        }
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
            "Industry Monopoly Game [FPS {}]", _ctx.time.fps()
        ));

        canvas.finish(_ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {//use for typing or ui, but not gameplay.
        if input.keycode == Some(keyboard::KeyCode::Escape) {
            ctx.request_quit();
        }
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: MouseButton, x: f32, y: f32) -> GameResult {
        self.mouse_down = true;
        self.mouse_start = (x, y);
        Ok(())
    }

    fn mouse_button_up_event(&mut self,_ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32) -> GameResult {
        self.mouse_down = false;
        self.mouse_start = (0.0, 0.0);
        Ok(())
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _xrel: f32, _yrel: f32) -> GameResult {
        //if self.mouse_down {
            // Mouse coordinates are PHYSICAL coordinates, but here we want logical coordinates.

            // If you simply use the initial coordinate system, then physical and logical
            // coordinates are identical.
            //self.pos_x = x;
            //self.pos_y = y;

            // If you change your screen coordinate system you need to calculate the
            // logical coordinates like this:
            /*
            let screen_rect = graphics::screen_coordinates(_ctx);
            let size = graphics::window(_ctx).inner_size();
            self.pos_x = (x / (size.width  as f32)) * screen_rect.w + screen_rect.x;
            self.pos_y = (y / (size.height as f32)) * screen_rect.h + screen_rect.y;
            */
        //}
        if self.mouse_down {
            let x_origin = self.mouse_start.0;
            let y_origin = self.mouse_start.1;
            if x_origin > x {//moved right
                self.world.get_view().pan(2, (x_origin-x) as i32);//pan left
            }else{//moved left
                self.world.get_view().pan(0, (x-x_origin) as i32);//pan right
            }
            if y_origin > y {//moved down
                self.world.get_view().pan(3, (y_origin-y) as i32);//pan up
            }else{//moved up
                self.world.get_view().pan(1, (y-y_origin) as i32);//pan down
            }
        }
        self.mouse_start = (x, y);
        //println!("Mouse motion, x: {x}, y: {y}, x_origin: {x_origin}, y_origin: {y_origin}");
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
