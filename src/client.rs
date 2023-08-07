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
    ymax:usize
}

impl Viewport {
    pub fn new(pos:Position_i32, tiles:Vec<Vec<Tile>>, ctx:&mut Context) -> Viewport {
        let mut view = Viewport {
            ter_tex:TerrainTex::init(ctx),
            imp_tex:ImprovementTex::init(ctx),
            str_tex:StructureTex::init(ctx),
            pos:pos,
            tiles:tiles,
            xmax:22,
            ymax:12
        };
        view
    }

    pub fn get_pos(&self) -> Position_i32 {
        self.pos
    }

    pub fn transform(&mut self, pos:Position_i32) {
        if pos.x >= 0 && pos.y >= 0 /*&& pos.x <= SIZE_OF_WORLD-VIEW_SIZE && sAmEfOrThE_yVaR */{
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
        let xmin = (self.pos.x/100) as usize;
        let ymin = (self.pos.y/100) as usize;
        let mut x = 0;
        let mut y = 0;
        for y in 0..self.ymax {
            for x in 0..self.xmax {
                self.tiles[y][x] = world_tiles[(xmin+x) as usize][(ymin+y) as usize];
            }
        }
    }

    pub fn render(&self, canvas:&mut Canvas) {
        let x_offset = (self.pos.x%100) as f32;
        let y_offset = (self.pos.y%100) as f32;
        for y in 0..self.ymax {
            for x in 0..self.xmax {
                canvas.draw(self.get_terrain_tex(self.tiles[y][x]), DrawParam::new().dest([x as f32 * 100.0-x_offset, y as f32 * 100.0-y_offset]));
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