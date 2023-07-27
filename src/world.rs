#![allow(unused)]

use geometry_2d::geometry::Position;
use ggez::{graphics::{Canvas, DrawParam, Image}, Context};

use crate::textures::{TerrainTex, ImprovementTex, StructureTex};

pub struct World {
    ter_tex:TerrainTex,
    imp_tex:ImprovementTex,
    str_tex:StructureTex,
    terrain_tiles:Vec<Tile>,
    improvement_tiles:Vec<Tile>,
    structure_tiles:Vec<Tile>
}

impl World {
    pub fn new(width:usize, ctx:&mut Context) -> World {
        World {
            ter_tex:TerrainTex::init(ctx),
            imp_tex:ImprovementTex::init(ctx),
            str_tex:StructureTex::init(ctx),
            terrain_tiles:Vec::with_capacity(width*width),
            improvement_tiles:Vec::with_capacity(width*width),
            structure_tiles:Vec::with_capacity(width*width)
        }
    }

    fn set_tile(&mut self, x:f32, y:f32) {
        self.terrain_tiles[x as usize] = Tile::new(TerrainType::Grass, ImprovementType::None, StructureType::None, Position::new(x, y));
    }
    
    fn populate(&mut self) {
        let mut x = 0.0;
        let mut y = 0.0;
        loop{
            if x >= (self.terrain_tiles.len() as f32).sqrt() {//end of line
                x = 0.0;
                y += 1.0;
                break;
            }
            self.set_tile(x, y);
            x += 1.0;
        }
    }

    pub fn render(&self, canvas: &mut Canvas, pos: Position) {
        canvas.draw(self.ter_tex.terrain_grass(), DrawParam::new());
    }

    pub fn get_terrain(&self) -> &Vec<Tile> {
        &self.terrain_tiles
    }

    pub fn get_improvements(&self) -> &Vec<Tile> {
        &self.improvement_tiles
    }

    pub fn get_structures(&self) -> &Vec<Tile> {
        &self.structure_tiles
    }

    pub fn set_terrain(&mut self, pos:Position, terrain:TerrainType) {
        let mut count = 0;
        while count < self.terrain_tiles.len() {
            if self.terrain_tiles[count].get_pos().x == pos.x && self.terrain_tiles[count].get_pos().y == pos.y {
                self.terrain_tiles[count].set_terrain(terrain);
                break;
            }
            count += 1;
        }
    }

    pub fn set_improvement(&mut self, pos:Position, improvement:ImprovementType) {
        let mut count = 0;
        while count < self.improvement_tiles.len() {
            if self.improvement_tiles[count].get_pos().x == pos.x && self.improvement_tiles[count].get_pos().y == pos.y {
                self.improvement_tiles[count].set_improvement(improvement);
                break;
            }
            count += 1;
        }
    }

    pub fn set_structure(&mut self, pos:Position, structure:StructureType) {
        let mut count = 0;
        while count < self.structure_tiles.len() {
            if self.structure_tiles[count].get_pos().x == pos.x && self.structure_tiles[count].get_pos().y == pos.y {
                self.structure_tiles[count].set_structure(structure);
                break;
            }
            count += 1;
        }
    }
}

pub struct Tile {
    terrain: TerrainType,
    improvement: ImprovementType,
    structure: StructureType,
    pos:Position
}

impl Tile {
    pub fn new(terrain:TerrainType, improvement:ImprovementType, structure:StructureType, pos:Position) -> Tile {
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

    pub fn get_pos(&self) -> Position {
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

pub enum TerrainType {
    Grass,
    Sand,
    Dirt,
    Water
}

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