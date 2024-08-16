use crate::constants::*;

use std::process::Command;

#[derive(Clone, Copy)]
pub enum ShaderStage {
    Fragment,
    Vertex,
}

#[derive(Clone, Copy)]
pub enum ShaderTarget {
    SpirV,
    Glsl,
}

#[derive(Clone, Copy)]
pub enum ShaderProfile {
    Glsl450,
}

#[derive(Clone, Copy)]
pub enum CompileError {
    MissingSourcePath,
    MissingDestPath,
    MissingShaderStage,
}

pub struct SlangCompile {
    source_path: Option<String>,
    dest_path: Option<String>,
    stage: Option<ShaderStage>,
    compile_to: Option<ShaderTarget>,
    profile: Option<ShaderProfile>,
}

impl SlangCompile {
    pub fn new() -> Self {
        return SlangCompile {
            source_path: None,
            dest_path: None,
            stage: None,
            compile_to: None,
            profile: None,
        };
    }

    pub fn with_source(&mut self, source_path: &str) -> &mut Self {
        self.source_path = Some(source_path.to_owned());
        return self;
    }

    pub fn to_destinatino(&mut self, dest_path: &str) -> &mut Self {
        self.dest_path = Some(dest_path.to_owned());
        return self;
    }

    pub fn with_stage(&mut self, stage: ShaderStage) -> &mut Self {
        self.stage = Some(stage);
        return self;
    }

    pub fn to_target(&mut self, compile_to: ShaderTarget) -> &mut Self {
        self.compile_to = Some(compile_to);
        return self;
    }

    pub fn with_profile(&mut self, profile: ShaderProfile) -> &mut Self {
        self.profile = Some(profile);
        return self;
    }

    pub fn compile(&self) -> Result<(), CompileError> {
        let mut slangc_command = Command::new("slangc");
        let source_path_cache: &str;
        let dest_path_cache: &str;

        if let Some(source_path) = &self.source_path {
            slangc_command.arg(source_path);
            source_path_cache = source_path;
        } else {
            return Err(CompileError::MissingSourcePath);
        }

        if let Some(profile) = self.profile {
            slangc_command.arg("-profile").arg(match profile {
                ShaderProfile::Glsl450 => "glsl_450",
            });
        } else {
            slangc_command.arg("-profile").arg("glsl_450");
        }

        if let Some(compile_to) = self.compile_to {
            slangc_command.arg("-target").arg(match compile_to {
                ShaderTarget::SpirV => "spirv",
                ShaderTarget::Glsl => "glsl",
            });
        } else {
            slangc_command.arg("-target").arg("spirv");
        }

        if let Some(dest_path) = &self.dest_path {
            slangc_command.arg("-o").arg(dest_path);
            dest_path_cache = dest_path;
        } else {
            return Err(CompileError::MissingDestPath);
        }

        if let Some(stage) = self.stage {
            slangc_command.arg("-entry").arg(match stage {
                ShaderStage::Fragment => "fragment",
                ShaderStage::Vertex => "vertex",
            });
        } else {
            return Err(CompileError::MissingShaderStage);
        }

        slangc_command.arg("-I").arg("assets/shaders/sdf2d");
        slangc_command.arg("-I").arg("assets/shaders/math_utils");

        slangc_command.arg("-fvk-use-entrypoint-name").status().expect(&format!(
            "Failed to compile shader {} to {}",
            source_path_cache, dest_path_cache
        ));

        println!("Compiling shader {} to {}", source_path_cache, dest_path_cache);

        return Ok(());
    }
}

pub fn compile_shaders(path: &str) {
    let file_name = path.split("/").last().unwrap().split(".").collect::<Vec<&str>>()[0];

    match SlangCompile::new()
        .with_source(path)
        .with_stage(ShaderStage::Fragment)
        .to_destinatino(&format!("{ASSETS}/{COMPILED_SHADERS}/{file_name}.frag.spv"))
        .to_target(ShaderTarget::SpirV)
        .compile()
    {
        Ok(_) => (),
        Err(_) => eprintln!("Failed to compile slang fragment shader at {path} to SPIRV"),
    }

    match SlangCompile::new()
        .with_source(path)
        .with_stage(ShaderStage::Vertex)
        .to_destinatino(&format!("{ASSETS}/{COMPILED_SHADERS}/{file_name}.vert.spv",))
        .to_target(ShaderTarget::SpirV)
        .compile()
    {
        Ok(_) => (),
        Err(_) => eprintln!("Failed to compile slang vertex shader at {path} to SPIRV"),
    }

    match SlangCompile::new()
        .with_source(path)
        .with_stage(ShaderStage::Fragment)
        .to_destinatino(&format!("{ASSETS}/{COMPILED_SHADERS}/{file_name}.frag",))
        .to_target(ShaderTarget::Glsl)
        .compile()
    {
        Ok(_) => (),
        Err(_) => eprintln!("Failed to compile slang fragment shader at {path} to GLSL"),
    }

    match SlangCompile::new()
        .with_source(path)
        .with_stage(ShaderStage::Vertex)
        .to_destinatino(&format!("{ASSETS}/{COMPILED_SHADERS}/{file_name}.vert"))
        .to_target(ShaderTarget::Glsl)
        .compile()
    {
        Ok(_) => (),
        Err(_) => eprintln!("Failed to compile slang vertex shader at {path} to GLSL"),
    }
}
