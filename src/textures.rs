use ggez::{graphics::Image, Context};

pub struct TerrainTex {
    terrain_grass:Image
}

impl TerrainTex {
    /**
     * Should only be called once, and then used forever to get the textures. 
     */
    pub fn init(ctx: &mut Context) -> TerrainTex {
        TerrainTex{
            terrain_grass:Image::from_path(ctx, "/grass_terrain.png").expect("Something went wrong loading an image")
        }
    }

    pub fn terrain_grass(&self) -> &Image {
        &self.terrain_grass
    }
}

pub struct ImprovementTex {
    images: Vec<Image>
}

impl ImprovementTex {
    /**
     * Should only be called once, and then used forever to get the textures. 
     */
    pub fn init(ctx: &mut Context) -> ImprovementTex {
        let mut images: Vec<Image> = Vec::new();
        images.insert(0, Image::from_path(ctx, "/grass_terrain.png").expect("Something went wrong loading an image"));
        ImprovementTex{
            images
        }
    }
}

pub struct StructureTex {
    images: Vec<Image>
}

impl StructureTex {
    /**
     * Should only be called once, and then used forever to get the textures. 
     */
    pub fn init(ctx: &mut Context) -> StructureTex {
        let mut images: Vec<Image> = Vec::new();
        images.insert(0, Image::from_path(ctx, "/grass_terrain.png").expect("Something went wrong loading an image"));
        StructureTex{
            images
        }
    }
}