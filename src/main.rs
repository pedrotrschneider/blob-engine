#[allow(dead_code)]
mod slang_utils;

use bevy::{
    math::vec2,
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef, ShaderType},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle, Mesh2dHandle},
};
use slang_utils::{ShaderStage, ShaderTarget, SlangCompile};

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
        return "shaders\\.compiled\\slang_post_processing.vert.spv".into();
    }

    fn fragment_shader() -> ShaderRef {
        return "shaders\\.compiled\\slang_post_processing.frag.spv".into();
    }
}

fn main() {
    compile_shaders();
    App::new()
        .add_plugins((
            DefaultPlugins,
            Material2dPlugin::<CustomMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

fn compile_shaders() {
    match SlangCompile::new()
        .with_source("assets/shaders/post_processing.slang")
        .with_stage(ShaderStage::Fragment)
        .to_destinatino("assets/shaders/.compiled/slang_post_processing.frag.spv")
        .to_target(ShaderTarget::SpirV)
        .compile()
    {
        Ok(_) => (),
        Err(_) => eprintln!("Failed to compile slang fragment shader"),
    }

    match SlangCompile::new()
        .with_source("assets/shaders/post_processing.slang")
        .with_stage(ShaderStage::Vertex)
        .to_destinatino("assets/shaders/.compiled/slang_post_processing.vert.spv")
        .to_target(ShaderTarget::SpirV)
        .compile()
    {
        Ok(_) => (),
        Err(_) => eprintln!("Failed to compile slang vertex shader"),
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
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
                    window_params.mouse_position = (mouse_position
                        - vec2(window.width() * 0.5, window.height() * 0.5))
                        * vec2(1.0, -1.0)
                }
                _ => (),
            }
        }
    }
}
