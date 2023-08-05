#![allow(unused)]

use geometry_2d::geometry::Position_i32;
use ggez::{graphics::{Canvas, DrawParam, Image}, Context};
use image::{ImageBuffer, GenericImageView};

use crate::{textures::{TerrainTex, ImprovementTex, StructureTex}, client::Viewport};

pub struct World {
    tiles:Vec<Vec<Tile>>,
    view:Viewport,
    size:i8
}

impl World {
    pub fn new(width:usize, ctx:&mut Context, size:i8) -> World {
        let mut world = World {
            tiles:Vec::with_capacity(width*width),
            view:Viewport::new(Position_i32::new(0, 0), Vec::new(), ctx),
            size:size
        };
        world.populate();
        world.view.populate();
        world.view.update(&world.tiles);
        world
    }

    fn set_tile(&mut self, x:usize, y:usize, terrain:TerrainType) {
        self.tiles[x][y] = Tile::new(terrain, ImprovementType::None, StructureType::None, Position_i32::new(x as i32, y as i32));
    }
    
    fn populate(&mut self) {
        let max = self.size as usize;
        println!("World width: {} (fn populate)", max);
        let mut map = image::open("res/map.png").unwrap();
        for y in 0..max {
            self.tiles.push(Vec::new());
        }
        for y in 0..max {
            for x in 0..max {
                let pixel = map.get_pixel(x as u32, y as u32);
                let t:Tile = Tile::new(self.get_terrain_by_color(pixel[0], pixel[1], pixel[2]), ImprovementType::None, StructureType::None, Position_i32::new(x as i32, y as i32));
                self.tiles[y].push(t);
            }
        }
    }

    pub fn get_terrain_by_color(&self, r:u8, g:u8, b:u8) -> TerrainType {
        if(r == 128 && g == 128 && b == 128) {
            TerrainType::Rock
        }else if(r == 255 && g == 255 && b == 0) {
            TerrainType::Sand
        }else if(r == 0 && g == 0 && b == 255) {
            TerrainType::Water
        }else {
            TerrainType::Grass
        }
    }

    pub fn render(&self, canvas: &mut Canvas, pos: Position_i32) {
        self.view.render(canvas);
    }

    pub fn get_tiles(&self) -> &Vec<Vec<Tile>> {
        &self.tiles
    }

    pub fn set_terrain(&mut self, pos:Position_i32, terrain:TerrainType) {
        self.tiles[pos.x as usize][pos.y as usize].set_terrain(terrain);
    }

    pub fn set_improvement(&mut self, pos:Position_i32, improvement:ImprovementType) {
        self.tiles[pos.x as usize][pos.y as usize].set_improvement(improvement);
    }

    pub fn set_structure(&mut self, pos:Position_i32, structure:StructureType) {
        self.tiles[pos.x as usize][pos.y as usize].set_structure(structure);
    }

    pub fn get_tile(&self, pos:Position_i32) -> Tile {
        self.tiles[pos.x as usize][pos.y as usize]
    }
}

/**
 A Tile is a struct to contain all information about a given cell in a world grid. It has a `Position_i32` which should never change, and data about what can be seen at that position such
 as a `TerrainType`, an `ImprovementType`, and a `StructureType`. 
 */
#[derive(Copy, Clone)]
pub struct Tile {
    terrain: TerrainType,
    improvement: ImprovementType,
    structure: StructureType,
    pos:Position_i32
}

impl Tile {
    pub fn new(terrain:TerrainType, improvement:ImprovementType, structure:StructureType, pos:Position_i32) -> Tile {
        Tile{
            terrain:terrain,
            improvement:improvement,
            structure:structure,
            pos:pos
        }
    }

    pub fn get_terrain(&self) -> &TerrainType {
        &self.terrain
    }

    pub fn get_improvement(&self) -> &ImprovementType {
        &self.improvement
    }

    pub fn get_structure(&self) -> &StructureType {
        &self.structure
    }

    pub fn get_pos(&self) -> Position_i32 {
        self.pos
    }

    pub fn set_terrain(&mut self, terrain: TerrainType){
        self.terrain = terrain;
    }

    pub fn set_improvement(&mut self, improvement: ImprovementType){
        self.improvement = improvement;
    }

    pub fn set_structure(&mut self, structure: StructureType){
        self.structure = structure;
    }
}

#[derive(Copy, Clone)]
pub enum TerrainType {
    Grass,
    Sand,
    Rock,
    Water
}

#[derive(Copy, Clone)]
pub enum ImprovementType {
    None,
    Cleared,
    DirtRoad,
    GravelRoad,
    PavedRoad,
    Ore,
    Forest,
    Boulders,
    Herd, 
    Orchard,
    Crop,
    OilSlick
}

#[derive(Copy, Clone)]
pub enum StructureType {
    None,
    CityHall,
    University,
    CustomsOffice,
    Mine,
    Sawmill,
    Farm,
    Quarry,
    OilField
}

pub enum Layer {
    Terrain,
    Improvements,
    Structures
}