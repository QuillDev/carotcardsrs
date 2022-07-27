use tetra::Context;
use tetra::graphics::Texture;

pub struct TextureLibrary {
    pub DIRT: Texture,
}

impl TextureLibrary {
    pub fn new(ctx: &mut Context) -> tetra::Result<TextureLibrary> {
        let texture_library = TextureLibrary {
            DIRT: Texture::new(ctx, "./resources/dirt.png")?
        };

        Ok(texture_library)
    }
}