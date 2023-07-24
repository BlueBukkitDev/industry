#![allow(unused)]
pub struct World {
    terrain_tiles:Vec<Tile>,
    improvement_tiles:Vec<Tile>,
    structure_tiles:Vec<Tile>
}

impl World {
    pub fn new(width:usize, height:usize) -> World {
        World {
            terrain_tiles:Vec::with_capacity(width*height),
            improvement_tiles:Vec::with_capacity(width*height),
            structure_tiles:Vec::with_capacity(width*height)
        }
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

    pub fn set_terrain(&mut self, pos:&Vec<i16>, terrain:TerrainType) {
        let mut count = 0;
        while count < self.terrain_tiles.len() {
            if self.terrain_tiles[count].get_pos()[0] == pos[0] && self.terrain_tiles[count].get_pos()[1] == pos[1] {
                self.terrain_tiles[count].set_terrain(terrain);
                break;
            }
            count += 1;
        }
    }

    pub fn set_improvement(&mut self, pos:&Vec<i16>, improvement:ImprovementType) {
        let mut count = 0;
        while count < self.improvement_tiles.len() {
            if self.improvement_tiles[count].get_pos()[0] == pos[0] && self.improvement_tiles[count].get_pos()[1] == pos[1] {
                self.improvement_tiles[count].set_improvement(improvement);
                break;
            }
            count += 1;
        }
    }

    pub fn set_structure(&mut self, pos:&Vec<i16>, structure:StructureType) {
        let mut count = 0;
        while count < self.structure_tiles.len() {
            if self.structure_tiles[count].get_pos()[0] == pos[0] && self.structure_tiles[count].get_pos()[1] == pos[1] {
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
    pos:Vec<i16>
}

impl Tile {
    pub fn new(terrain:TerrainType, improvement:ImprovementType, structure:StructureType, pos:Vec<i16>) -> Tile {
        Tile{
            terrain:terrain,
            improvement:improvement,
            structure:structure,
            pos:pos
        }
    }

    pub fn get_pos(&self) -> &Vec<i16> {
        &self.pos
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