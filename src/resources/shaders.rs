use shared_lib::gl_shader_manager::ShaderManager;
use std::collections::HashMap;

pub const SIMPLE_RED: &str = "simple_red_shader";
pub const SIMPLE_TRIANGLE: &str = "simple_triangle_shader";
pub const SIMPLE_TEXTURED_TRIANGLE: &str = "simple_textured_triangle_shader";
pub const SIMPLE_TRANSFORM: &str = "simple_transform_shader";
pub const SIMPLE_PROJECTION: &str = "simple_projection_shader";
pub const LIGHT_CUBE: &str = "light_cube_shader";

pub(crate) fn add_shaders(shader_manager: &mut ShaderManager) {
    let mut shader_map: HashMap<&'static str, Vec<&'static str>> = HashMap::new();
    shader_map.insert(SIMPLE_RED, vec![
        "assets/shaders/simple/simple_red_shader.vert",
        "assets/shaders/simple/simple_red_shader.frag",
    ]);
    shader_map.insert(SIMPLE_TRIANGLE, vec![
        "assets/shaders/simple/shader_triangle.vert",
        "assets/shaders/simple/shader_triangle.frag",
    ]);
    shader_map.insert(SIMPLE_TEXTURED_TRIANGLE, vec![
        "assets/shaders/simple/textured_triangle.vert",
        "assets/shaders/simple/textured_triangle.frag",
    ]);
    shader_map.insert(SIMPLE_TRANSFORM, vec![
        "assets/shaders/simple/transform.vert",
        "assets/shaders/simple/transform.frag",
    ]);
    shader_map.insert(SIMPLE_PROJECTION, vec![
        "assets/shaders/simple/projection.vert",
        "assets/shaders/simple/projection.frag",
    ]);
    shader_map.insert(LIGHT_CUBE, vec![
        "assets/shaders/light/light_cube.vert",
        "assets/shaders/light/light_cube.frag",
    ]);

    for (key, paths) in &shader_map {
        for path in paths {
            shader_manager.add_shader(key.to_string(), path.to_string());
        }
    }
}
