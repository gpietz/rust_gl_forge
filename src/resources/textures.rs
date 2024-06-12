use shared_lib::opengl::texture_manager::{TextureFlags, TextureManager};

pub(crate) const M016018BG: &str = "M016018BG";
pub(crate) const CRATE8: &str = "CRATE8";
pub(crate) const CRATE8512: &str = "CRATE8512";
pub(crate) const AWESOMEFACE2: &str = "AWESOMEFACE2";

pub(crate) enum Texture {
    M016018BG,
    CRATE8,
    CRATE8512,
    AWESOMEFACE2,
}

impl Texture {
    pub(crate) fn get_path(self) -> &'static str {
        match self {
            Texture::M016018BG => "assets/textures/m-016-018-bg.jpg",
            Texture::CRATE8 => "assets/textures/crate8.jpg",
            Texture::CRATE8512 => "assets/textures/crate8-512.jpg",
            Texture::AWESOMEFACE2 => "assets/textures/awesomeface2.png",
        }
    }
}

struct TextureInfo {
    pub name: String,
    pub path: String,
    pub has_alpha: bool,
    pub flip_vertically: bool,
}

impl TextureInfo {
    pub fn new(name: &str, path: &str, has_alpha: bool, flip_vertically: bool) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_string(),
            has_alpha,
            flip_vertically,
        }
    }
}

pub(crate) fn add_textures(texture_manager: &mut TextureManager) {
    let texture_infos = vec![
        TextureInfo::new(M016018BG, "assets/textures/m-016-018-bg.jpg", false, false),
        TextureInfo::new(CRATE8, "assets/textures/crate8.jpg", false, false),
        TextureInfo::new(CRATE8512, "assets/textures/crate8-512.jpg", false, false),
        TextureInfo::new(AWESOMEFACE2, "assets/textures/awesomeface2.png", true, true),
    ];

    for info in texture_infos {
        texture_manager.add_path(&info.name, &info.path);
        texture_manager.add_texture_flags(
            &info.name,
            TextureFlags {
                has_alpha: info.has_alpha,
                flip_vertically: info.flip_vertically,
            },
        );
    }
}
