mod scene_parsing;

use std::fs;

use bevy::{
    app::{Startup, Update},
    asset::{Asset, Assets},
    math::vec2,
    math::Vec2,
    prelude::*,
    prelude::{Camera2dBundle, Commands, Component, Mesh, ResMut},
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef, ShaderType},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle, Mesh2dHandle},
    DefaultPlugins,
};
use scene_parsing::SceneParser;

use crate::constants::*;
use crate::slang_utils;

#[derive(Default, Clone, Copy, ShaderType)]
pub struct SimulationParams {
    pub time: f32,
}

#[derive(Default, Clone, Copy, ShaderType)]
pub struct WindowParams {
    pub size: Vec2,
    pub aspect: f32,
    pub mouse_position: Vec2,
}

#[derive(Component)]
pub struct FullscreenTriangle;

#[derive(Default, Asset, TypePath, AsBindGroup, Clone)]
pub struct CustomMaterial {
    #[uniform(0)]
    pub simulation: SimulationParams,
    #[uniform(1)]
    pub window: WindowParams,
}

impl Material2d for CustomMaterial {
    fn vertex_shader() -> ShaderRef {
        return COMPILED_VERTEX_BEVY_PATH.into();
    }

    fn fragment_shader() -> ShaderRef {
        return COMPILED_FRAGMENT_BEVY_PATH.into();
    }
}

pub struct App;

impl App {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn run(&self) {
        fs::create_dir_all("assets/shaders/.compiled").expect("Unable to create .compiled shaders directory");
        fs::create_dir_all("assets/shaders/.generated").expect("Unable to create .generated shaders directory");

        SceneParser::parse_scene2d(SCENE_PATH).generate_shader();

        slang_utils::compile_shaders(GENERATED_SHADER_PATH);

        bevy::app::App::new()
            .add_plugins((DefaultPlugins, Material2dPlugin::<CustomMaterial>::default()))
            .add_systems(Startup, setup)
            .add_systems(Update, update)
            .run();
    }
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
