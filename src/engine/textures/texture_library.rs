use std::collections::HashMap;
use std::error::Error;
use std::ffi::OsStr;
use std::hash::Hash;

use macroquad::prelude::{load_texture, Texture2D};

pub struct TextureLibrary {
    textures: HashMap<String, Texture2D>,
    animations: HashMap<String, Vec<Texture2D>>,
    fallback: Texture2D,
}

impl TextureLibrary {

    pub async fn new() -> Result<TextureLibrary, Box<dyn Error>> {
        let fallback = load_texture("resources/textures/default.png").await?;
        let mut textures: HashMap<String, Texture2D> = HashMap::new();
        let mut animations: HashMap<String, Vec<Texture2D>> = HashMap::new();

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

        if let Ok(files) = std::fs::read_dir("resources/animations") {
            for file in files {
                match file {
                    Ok(entry) => {
                        let path = entry.path();
                        let anim = TextureLibrary::load_animation(path.to_str().unwrap()).await?;
                        animations.insert(String::from(entry.file_name().to_str().unwrap()), anim);
                    }
                    Err(_) => {}
                }
            }
        }
        println!("{:?}", animations);

        let texture_library = TextureLibrary{fallback, textures, animations};

        Ok(texture_library)
    }

    async fn load_animation(path: &str) -> Result<Vec<Texture2D>, Box<dyn Error>> {

        let mut anim: Vec<Texture2D> = Vec::new();

        if let Ok(files) = std::fs::read_dir(path) {
            for file in files {
                match file {
                    Ok(entry) => {
                        let texture = load_texture(entry.path().to_str().unwrap()).await?;
                        anim.push(texture);
                    }
                    Err(_) => {}
                }
            }
        }

        Ok(anim)
    }

    /// Gets a texture from the texture library
    pub fn get_texture(&self, name: &str) -> Texture2D {
        return match self.textures.get(name) {
            None => self.fallback.clone(),
            Some(tex) => tex.clone()
        };
    }
}