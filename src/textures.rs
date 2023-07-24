use ggez::{graphics::Image, Context};

pub struct TerrainTex {
    images: Vec<Image>
}

impl TerrainTex {
    /**
     * Should only be called once, and then used forever to get the textures. 
     */
    pub fn init(ctx: &mut Context) -> TerrainTex {
        let mut images: Vec<Image> = Vec::new();
        images.insert(0, Image::from_path(ctx, "/grass_terrain.png").expect("Something went wrong loading an image"));
        TerrainTex{
            images
        }
    }

    pub fn terrain_grass(&self) -> &Image {
        &self.images[0]
    }
}