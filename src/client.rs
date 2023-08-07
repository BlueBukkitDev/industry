#![allow(unused)]

use geometry_2d::geometry::Position_i32;
use ggez::{graphics::{Canvas, DrawParam, Image}, Context};
use glam::Vec2;
use crate::{world::{Tile, World, TerrainType, ImprovementType, StructureType}, textures::{TerrainTex, ImprovementTex, StructureTex}};

pub struct Viewport {
    ter_tex:TerrainTex,
    imp_tex:ImprovementTex,
    str_tex:StructureTex,
    pos:Position_i32,
    tiles:Vec<Vec<Tile>>,
    xmax:usize,
    ymax:usize,
    tile_scale:f32,
    tile_size:f32,
    size:(f32, f32)
}

impl Viewport {
    pub fn new(pos:Position_i32, tiles:Vec<Vec<Tile>>, ctx:&mut Context) -> Viewport {
        let mut view = Viewport {
            ter_tex:TerrainTex::init(ctx),
            imp_tex:ImprovementTex::init(ctx),
            str_tex:StructureTex::init(ctx),
            pos:pos,
            tiles:tiles,
            tile_scale:(ctx.gfx.drawable_size().0/24.0)/100.0,//24 is the intended number of tiles to display. 100 is the actual tile image size.
            tile_size:ctx.gfx.drawable_size().0/24.0,
            size:(ctx.gfx.drawable_size().0, ctx.gfx.drawable_size().1),
            xmax:25,//we draw 25 tiles because sometimes you may have portions of tiles rendered on either side, leading to intended tiles +1. 
            ymax:(25 as f32*(ctx.gfx.drawable_size().1/ctx.gfx.drawable_size().0)+1.0) as usize//this should do? Might not round up.
        };
        println!("Size: {}, Scale: {}", view.tile_size, view.tile_scale);
        view
    }

    pub fn get_pos(&self) -> Position_i32 {
        self.pos
    }

    pub fn transform(&mut self, pos:Position_i32) {
        if pos.x >= 0 && pos.y >= 0 /*&& pos.x <= SIZE_OF_WORLD-VIEW_SIZE && sAmEfOrThE_yVaR */ {
            self.pos = pos;
        }
    }

    pub fn populate(&mut self) {
        for y in 0..self.ymax {
            self.tiles.push(Vec::new());
        }
        for y in 0..self.ymax {
            for x in 0..self.xmax {
                self.tiles[y].push(Tile::new(TerrainType::Grass, ImprovementType::None, StructureType::None, Position_i32::new(x as i32, y as i32)));
            }
        }
    }

    /**
     Takes in the entire world's tiles to determine which should be rendered based on current position. 
     */
    pub fn update(&mut self, world_tiles:&Vec<Vec<Tile>>) {
        let xmin = (self.pos.x as f32/self.tile_size) as usize;
        let ymin = (self.pos.y as f32/self.tile_size) as usize;
        let mut x = 0;
        let mut y = 0;
        for y in 0..self.ymax {
            for x in 0..self.xmax {
                self.tiles[y][x] = world_tiles[(xmin+x) as usize][(ymin+y) as usize];
            }
        }
    }

    pub fn render(&self, canvas:&mut Canvas) {
        let x_offset = (self.pos.x as f32%self.tile_size) as f32;
        let y_offset = (self.pos.y as f32%self.tile_size) as f32;
        for y in 0..self.ymax {
            for x in 0..self.xmax {
                canvas.draw(self.get_terrain_tex(self.tiles[y][x]), 
                DrawParam::new().dest([(x as f32 * self.tile_size)-x_offset, (y as f32 * self.tile_size)-y_offset]).scale([self.tile_scale, self.tile_scale]));
            }
        }
    }

    pub fn get_terrain_tex(&self, tile:Tile) -> &Image {
        match tile.get_terrain() {
            TerrainType::Grass => self.ter_tex.terrain_grass(),
            TerrainType::Rock => self.ter_tex.terrain_rock(),
            TerrainType::Sand => self.ter_tex.terrain_sand(),
            TerrainType::Water => self.ter_tex.terrain_water()
        }
    }

    /**
     * Takes in `dir` 0-3, where 0 is left, 1 is up, 2 is right, and 3 is down. For safety, anything else will be treated as 3. 
     * Field `amt` is a distance to transform the position.
     */
    pub fn pan(&mut self, dir:u8, amt:i32) {
        if dir == 0 {
            self.transform(Position_i32::new(self.pos.x - amt, self.pos.y));
        }else if dir == 1 {
            self.transform(Position_i32::new(self.pos.x, self.pos.y - amt));
        }else if dir == 2 {
            self.transform(Position_i32::new(self.pos.x + amt, self.pos.y));
        }else {
            self.transform(Position_i32::new(self.pos.x, self.pos.y + amt));
        }
    }
}