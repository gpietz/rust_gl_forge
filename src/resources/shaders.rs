use shared_lib::gl_shader_manager::ShaderManager;
use std::collections::HashMap;

pub const SIMPLE_RED: &str = "simple_red_shader";
pub const SIMPLE_TRIANGLE: &str = "simple_triangle_shader";
pub const SIMPLE_TEXTURED_TRIANGLE: &str = "simple_textured_triangle_shader";
pub const SIMPLE_TRANSFORM: &str = "simple_transform_shader";
pub const SIMPLE_PROJECTION: &str = "simple_projection_shader";

pub(crate) fn add_shaders(shader_manager: &mut ShaderManager) {
    let mut shader_map: HashMap<String, Vec<String>> = HashMap::new();
    add_shader(
        &mut shader_map,
        SIMPLE_RED,
        vec![
            "assets/shaders/simple/simple_red_shader.vert",
            "assets/shaders/simple/simple_red_shader.frag",
        ],
    );
    add_shader(
        &mut shader_map,
        SIMPLE_TRIANGLE,
        vec![
            "assets/shaders/simple/shader_triangle.vert",
            "assets/shaders/simple/shader_triangle.frag",
        ],
    );
    add_shader(
        &mut shader_map,
        SIMPLE_TEXTURED_TRIANGLE,
        vec![
            "assets/shaders/simple/textured_triangle.vert",
            "assets/shaders/simple/textured_triangle.frag",
        ],
    );
    add_shader(
        &mut shader_map,
        SIMPLE_TRANSFORM,
        vec![
            "assets/shaders/simple/transform.vert",
            "assets/shaders/simple/transform.frag",
        ],
    );
    add_shader(
        &mut shader_map,
        SIMPLE_PROJECTION,
        vec![
            "assets/shaders/simple/projection.vert",
            "assets/shaders/simple/projection.frag"
        ],
    );

    for (key, paths) in shader_map.iter() {
        for path in paths {
            shader_manager.add_shader(key.clone(), path.clone());
        }
    }
}

fn add_shader(map: &mut HashMap<String, Vec<String>>, name: &str, shaders: Vec<&str>) {
    let shader_vec = shaders.into_iter().map(|s| s.to_string()).collect();
    map.insert(name.to_string(), shader_vec);
}
