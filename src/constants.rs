use const_format::formatcp;

pub const ASSETS: &'static str = "assets";
pub const SCENES: &'static str = "scenes";
pub const GENERATED_SHADERS: &'static str = "shaders/.generated";
pub const COMPILED_SHADERS: &'static str = "shaders/.compiled";

pub const SCENE_FILE: &'static str = "scene1.2d.json";
pub const SCENE_SHADER_FILE: &'static str = "scene_1";

pub const SCENE_PATH: &'static str = formatcp!("{ASSETS}/{SCENES}/{SCENE_FILE}");
pub const GENERATED_SHADER_PATH: &'static str = formatcp!("{ASSETS}/{GENERATED_SHADERS}/{SCENE_SHADER_FILE}.slang");
pub const COMPILED_VERTEX_PATH: &'static str = formatcp!("{ASSETS}/{COMPILED_SHADERS}/{SCENE_SHADER_FILE}.vert.spv");
pub const COMPILED_FRAGMENT_PATH: &'static str = formatcp!("{ASSETS}/{COMPILED_SHADERS}/{SCENE_SHADER_FILE}.frag.spv");
pub const COMPILED_VERTEX_GLSL_PATH: &'static str = formatcp!("{ASSETS}/{COMPILED_SHADERS}/{SCENE_SHADER_FILE}.vert");
pub const COMPILED_FRAGMENT_GLSL_PATH: &'static str = formatcp!("{ASSETS}/{COMPILED_SHADERS}/{SCENE_SHADER_FILE}.frag");
pub const COMPILED_VERTEX_BEVY_PATH: &'static str = formatcp!("{COMPILED_SHADERS}/{SCENE_SHADER_FILE}.vert.spv");
pub const COMPILED_FRAGMENT_BEVY_PATH: &'static str = formatcp!("{COMPILED_SHADERS}/{SCENE_SHADER_FILE}.frag.spv");
