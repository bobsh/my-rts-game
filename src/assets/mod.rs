pub mod textures;
pub mod sounds;

#[allow(dead_code)]
pub fn load_assets() {
    textures::load_textures();
    sounds::load_sounds();
}
