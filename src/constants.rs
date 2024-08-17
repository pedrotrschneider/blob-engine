pub mod paths {
    use const_format::formatcp;

    pub const ASSETS: &'static str = "assets";
    pub const SCENES: &'static str = "scenes";
    pub const GENERATED_SHADERS: &'static str = "shaders/.generated";
    pub const COMPILED_SHADERS: &'static str = "shaders/.compiled";
    pub const CORE_SHADERS: &'static str = "shaders/.core";

    pub const ASSETS_SCENES: &'static str = formatcp!("{ASSETS}/{SCENES}");
    pub const ASSETS_SHADERS: &'static str = formatcp!("{ASSETS}/shaders");
    pub const ASSETS_COMPILED_SHADERS: &'static str = formatcp!("{ASSETS}/{COMPILED_SHADERS}");
    pub const ASSETS_GENERATED_SHADERS: &'static str = formatcp!("{ASSETS}/{GENERATED_SHADERS}");
    pub const ASSETS_CORE_SHADERS: &'static str = formatcp!("{ASSETS}/{CORE_SHADERS}");

    pub const MATH_UTILS: &'static str = formatcp!("{ASSETS_SHADERS}/math_utils");
    pub const SDF2D: &'static str = formatcp!("{ASSETS_SHADERS}/sdf2d");

    pub const MATH_UTILS_CORE: &'static str = formatcp!("{ASSETS_CORE_SHADERS}/math_utils");
    pub const SDF2D_CORE: &'static str = formatcp!("{ASSETS_CORE_SHADERS}/sdf2d");
}

pub mod files {
    use super::paths;
    use const_format::formatcp;

    pub const GENERATED_SHADER: &'static str = formatcp!("{}/main.slang", paths::ASSETS_GENERATED_SHADERS);
    pub const COMPILED_VERTEX: &'static str = formatcp!("{}/main.vert.spv", paths::ASSETS_COMPILED_SHADERS);
    pub const COMPILED_FRAGMENT: &'static str = formatcp!("{}/main.frag.spv", paths::ASSETS_COMPILED_SHADERS);
    pub const COMPILED_VERTEX_GLSL: &'static str = formatcp!("{}/main.vert", paths::ASSETS_COMPILED_SHADERS);
    pub const COMPILED_FRAGMENT_GLSL: &'static str = formatcp!("{}/main.frag", paths::ASSETS_COMPILED_SHADERS);
    pub const COMPILED_VERTEX_BEVY: &'static str = formatcp!("{}/main.vert.spv", paths::COMPILED_SHADERS);
    pub const COMPILED_FRAGMENT_BEVY: &'static str = formatcp!("{}/main.frag.spv", paths::COMPILED_SHADERS);

    pub const MATH_UTILS_SHADER_CORE: &'static str = formatcp!("{}/math_utils.slang", paths::MATH_UTILS_CORE);
    pub const SDF2D_SHADER_CORE: &'static str = formatcp!("{}/sdf2d.slang", paths::SDF2D_CORE);
    pub const SHAPES2D_SHADER_CORE: &'static str = formatcp!("{}/shapes2d.slang", paths::SDF2D_CORE);
}

pub mod shaders {
    pub const MATH_UTILS: &'static str = include_str!("../assets/shaders/math_utils/math_utils.slang");
    pub const BASE2D: &'static str = include_str!("../assets/shaders/base2d.slang");
    pub const SDF2D: &'static str = include_str!("../assets/shaders/sdf2d/sdf2d.slang");
    pub const SHAPES2D: &'static str = include_str!("../assets/shaders/sdf2d/shapes2d.slang");
}
