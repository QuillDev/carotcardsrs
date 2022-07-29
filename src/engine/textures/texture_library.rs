use std::collections::HashMap;
use std::ffi::OsStr;
use tetra::Context;
use tetra::graphics::Texture;

pub struct TextureLibrary {
    textures: HashMap<String, Texture>,
    fallback: Texture,
}

impl TextureLibrary {

    pub fn new(ctx: &mut Context) -> tetra::Result<TextureLibrary> {

        let fallback = Texture::new(ctx, "resources/textures/default.png")?;
        let mut textures: HashMap<String, Texture> = HashMap::new();
        for res in glob::glob("resources/textures/**/*.png").expect("Failed to read glob pattern") {
            match res {
                Ok(path) => {
                    let name = path.file_stem().unwrap().to_str().unwrap();
                    let file = path.to_str().unwrap();
                    let texture = Texture::new(ctx, file)?;
                    textures.insert(String::from(name), texture);
                },
                _ => {}
            }
        }

        let texture_library = TextureLibrary{fallback, textures};
        Ok(texture_library)
    }

    /// Gets a texture from the texture library
    pub fn get_texture(&self, name: &str) -> Texture {
        return match self.textures.get(name) {
            None => self.fallback.clone(),
            Some(tex) => tex.clone()
        };
    }
}