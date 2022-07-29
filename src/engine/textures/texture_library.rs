use std::collections::HashMap;
use std::error::Error;
use std::ffi::OsStr;

use macroquad::prelude::{load_texture, Texture2D};

pub struct TextureLibrary {
    textures: HashMap<String, Texture2D>,
    fallback: Texture2D,
}

impl TextureLibrary {

    pub async fn new() -> Result<TextureLibrary, Box<dyn Error>> {

        let fallback = load_texture("resources/textures/default.png").await?;
        let mut textures: HashMap<String, Texture2D> = HashMap::new();

        for res in glob::glob("resources/textures/**/*.png").expect("Failed to read glob pattern") {
            match res {
                Ok(path) => {
                    let name = path.file_stem().unwrap().to_str().unwrap();
                    let file = path.to_str().unwrap();
                    let texture = load_texture(file).await?;
                    textures.insert(String::from(name), texture);
                },
                _ => {}
            }
        }

        let texture_library = TextureLibrary{fallback, textures};
        Ok(texture_library)
    }

    /// Gets a texture from the texture library
    pub fn get_texture(&self, name: &str) -> Texture2D {
        return match self.textures.get(name) {
            None => self.fallback.clone(),
            Some(tex) => tex.clone()
        };
    }
}