#[allow(dead_code)]
mod scene_parsing;
#[allow(dead_code)]
mod slang_utils;

use std::fs;

use bevy::{
    math::vec2,
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef, ShaderType},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle, Mesh2dHandle},
};
use const_format::formatcp;
use scene_parsing::SceneParser;
use slang_utils::{ShaderStage, ShaderTarget, SlangCompile};

const ASSETS: &'static str = "assets";
const SCENES: &'static str = "scenes";
const GENERATED_SHADERS: &'static str = "shaders/.generated";
const COMPILED_SHADERS: &'static str = "shaders/.compiled";

const SCENE_FILE: &'static str = "scene1.2d.json";
const SCENE_SHADER_FILE: &'static str = "scene_1";

const SCENE_PATH: &'static str = formatcp!("{ASSETS}/{SCENES}/{SCENE_FILE}");
const GENERATED_SHADER_PATH: &'static str = formatcp!("{ASSETS}/{GENERATED_SHADERS}/{SCENE_SHADER_FILE}.slang");
const COMPILED_VERTEX_PATH: &'static str = formatcp!("{ASSETS}/{COMPILED_SHADERS}/{SCENE_SHADER_FILE}.vert.spv");
const COMPILED_FRAGMENT_PATH: &'static str = formatcp!("{ASSETS}/{GENERATED_SHADERS}/{SCENE_SHADER_FILE}.frag.spv");
const COMPILED_VERTEX_GLSL_PATH: &'static str = formatcp!("{ASSETS}/{COMPILED_SHADERS}/{SCENE_SHADER_FILE}.vert");
const COMPILED_FRAGMENT_GLSL_PATH: &'static str = formatcp!("{ASSETS}/{GENERATED_SHADERS}/{SCENE_SHADER_FILE}.frag");
const COMPILED_VERTEX_BEVY_PATH: &'static str = formatcp!("{COMPILED_SHADERS}/{SCENE_SHADER_FILE}.vert.spv");
const COMPILED_FRAGMENT_BEVY_PATH: &'static str = formatcp!("{GENERATED_SHADERS}/{SCENE_SHADER_FILE}.frag.spv");

#[derive(Default, Clone, Copy, ShaderType)]
struct SimulationParams {
    time: f32,
}

#[derive(Default, Clone, Copy, ShaderType)]
struct WindowParams {
    size: Vec2,
    aspect: f32,
    mouse_position: Vec2,
}

#[derive(Component)]
struct FullscreenTriangle;

#[derive(Default, Asset, TypePath, AsBindGroup, Clone)]
struct CustomMaterial {
    #[uniform(0)]
    simulation: SimulationParams,
    #[uniform(1)]
    window: WindowParams,
}

impl Material2d for CustomMaterial {
    fn vertex_shader() -> ShaderRef {
        return COMPILED_VERTEX_BEVY_PATH.into();
    }

    fn fragment_shader() -> ShaderRef {
        return COMPILED_FRAGMENT_BEVY_PATH.into();
    }
}

fn compile_shaders() {
    match SlangCompile::new()
        .with_source(GENERATED_SHADER_PATH)
        .with_stage(ShaderStage::Fragment)
        .to_destinatino(COMPILED_FRAGMENT_PATH)
        .to_target(ShaderTarget::SpirV)
        .compile()
    {
        Ok(_) => (),
        Err(_) => eprintln!("Failed to compile slang fragment shader"),
    }

    match SlangCompile::new()
        .with_source(GENERATED_SHADER_PATH)
        .with_stage(ShaderStage::Vertex)
        .to_destinatino(COMPILED_VERTEX_PATH)
        .to_target(ShaderTarget::SpirV)
        .compile()
    {
        Ok(_) => (),
        Err(_) => eprintln!("Failed to compile slang vertex shader"),
    }
}

fn compile_shaders_glsl() {
    match SlangCompile::new()
        .with_source(GENERATED_SHADER_PATH)
        .with_stage(ShaderStage::Fragment)
        .to_destinatino(COMPILED_FRAGMENT_GLSL_PATH)
        .to_target(ShaderTarget::Glsl)
        .compile()
    {
        Ok(_) => (),
        Err(_) => eprintln!("Failed to compile slang fragment shader"),
    }

    match SlangCompile::new()
        .with_source(GENERATED_SHADER_PATH)
        .with_stage(ShaderStage::Vertex)
        .to_destinatino(COMPILED_VERTEX_GLSL_PATH)
        .to_target(ShaderTarget::Glsl)
        .compile()
    {
        Ok(_) => (),
        Err(_) => eprintln!("Failed to compile slang vertex shader"),
    }
}

fn main() {
    fs::create_dir_all("assets/shaders/.compiled").expect("Unable to create .compiled shaders directory");
    fs::create_dir_all("assets/shaders/.generated").expect("Unable to create .generated shaders directory");

    SceneParser::parse_scene2d(SCENE_PATH).generate_shader();

    compile_shaders();
    compile_shaders_glsl();

    App::new()
        .add_plugins((DefaultPlugins, Material2dPlugin::<CustomMaterial>::default()))
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<CustomMaterial>>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Triangle2d::new(Vec2::ZERO, Vec2::ZERO, Vec2::ZERO))),
            material: materials.add(CustomMaterial { ..default() }),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        FullscreenTriangle,
    ));
}

fn update(
    time: Res<Time>,
    window: Query<&Window>,
    material_handles: Query<&Handle<CustomMaterial>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    let window = window.single();
    for material_handle in material_handles.iter() {
        if let Some(material) = materials.get_mut(material_handle) {
            let simulation_params = &mut material.simulation;
            simulation_params.time = time.elapsed_seconds();

            let window_params = &mut material.window;
            window_params.size = vec2(window.width(), window.height());
            window_params.aspect = window.width() / window.height();

            match window.cursor_position() {
                Some(mouse_position) => {
                    window_params.mouse_position =
                        (mouse_position - vec2(window.width() * 0.5, window.height() * 0.5)) * vec2(1.0, -1.0)
                }
                _ => (),
            }
        }
    }
}
