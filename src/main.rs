#[allow(dead_code)]
mod slang_utils;

use std::fs;

use bevy::{
    math::vec2,
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef, ShaderType},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle, Mesh2dHandle},
};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use slang_utils::{ShaderStage, ShaderTarget, SlangCompile};

const SCENE_PATH: &str = "assets/scenes/scene1.2d.json";
const GENERATED_SHADER_PATH: &str = "assets/shaders/.generated/scene_1.slang";
const COMPILED_VERTEX_PATH: &str = "assets/shaders/.compiled/scene_1.vert.spv";
const COMPILED_FRAGMENT_PATH: &str = "assets/shaders/.compiled/scene_1.frag.spv";
const BASE_2D_SHADER_PATH: &str = "assets/shaders/base_2d.slang";

const COMPILED_VERTEX_PATH_WIN: &str = "shaders/.compiled/scene_1.vert.spv";
const COMPILED_FRAGMENT_PATH_WIN: &str = "shaders/.compiled/scene_1.frag.spv";

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct ParsedVec2 {
    x: f32,
    y: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct ParsedVec3 {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct ParsedVec4 {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct ParsedColor {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct ParsedTransform2D {
    position: ParsedVec2,
    rotation: f32,
    scale: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct ParsedMaterial2D {
    color: ParsedColor,
    onion: Option<f32>,
    rounding: Option<f32>,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(i32)]
enum ParsedShape2DType {
    Circle,
    Rect,
    Segment,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct ParsedShape2D {
    shape_id: ParsedShape2DType,
    data: Vec<f32>,
    transform: ParsedTransform2D,
    material: ParsedMaterial2D,
}

#[derive(Serialize, Deserialize, Debug)]
struct ParsedScene2D {
    name: String,
    shapes: Vec<ParsedShape2D>,
}

impl ParsedScene2D {
    fn from_file(scene_path: &str) -> Self {
        let scene_str = fs::read_to_string(scene_path).expect(&format!("Unable to read scene at path {}", scene_path));
        let scene: ParsedScene2D = serde_json::from_str(&scene_str).unwrap();
        return scene;
    }

    fn generate_shader(&self) {
        let shader_str = fs::read_to_string(BASE_2D_SHADER_PATH)
            .expect(&format!("Unable to read base 2d shader in path {}", BASE_2D_SHADER_PATH));

        let mut scene_str = "".to_owned();
        for (i, shape) in self.shapes.iter().enumerate() {
            match shape.shape_id {
                ParsedShape2DType::Circle => {
                    scene_str += &format!("    SDFCircle shape{} = SDFCircle({});\n", i, shape.data[0]);

                    scene_str += &format!("    shape{}.transform.position.x = {};\n", i, shape.transform.position.x);
                    scene_str += &format!("    shape{}.transform.position.y = {};\n", i, shape.transform.position.y);
                    scene_str += &format!("    shape{}.transform.rotation = {};\n", i, shape.transform.rotation);
                    scene_str += &format!("    shape{}.transform.scale = {};\n", i, shape.transform.scale);

                    scene_str += &format!("    shape{}.material.color.r = {};\n", i, shape.material.color.r);
                    scene_str += &format!("    shape{}.material.color.g = {};\n", i, shape.material.color.g);
                    scene_str += &format!("    shape{}.material.color.b = {};\n", i, shape.material.color.b);
                    scene_str += &format!("    shape{}.material.color.a = {};\n", i, shape.material.color.a);

                    if let Some(onion) = shape.material.onion {
                        scene_str += &format!("    scene.add(shape{}.sdf(uv).onion({}));", i, onion);
                        continue;
                    }

                    if let Some(rounding) = shape.material.rounding {
                        scene_str += &format!("    scene.add(shape{}.sdf(uv).rounded({}));", i, rounding);
                        continue;
                    }

                    scene_str += &format!("    scene.add(shape{}.sdf(uv));", i);
                }
                _ => {}
            }
        }

        let shader_path = format!(
            "assets/shaders/.generated/{}.slang",
            self.name.to_lowercase().replace(" ", "_")
        );
        fs::write(shader_path, shader_str.replace("// {!!}", &scene_str)).expect("Unable to write shader for scene 1");
    }
}

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
        return COMPILED_VERTEX_PATH_WIN.into();
    }

    fn fragment_shader() -> ShaderRef {
        return COMPILED_FRAGMENT_PATH_WIN.into();
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
        .to_destinatino("assets/shaders/.compiled/slang_post_processing.frag")
        .to_target(ShaderTarget::Glsl)
        .compile()
    {
        Ok(_) => (),
        Err(_) => eprintln!("Failed to compile slang fragment shader"),
    }

    match SlangCompile::new()
        .with_source(GENERATED_SHADER_PATH)
        .with_stage(ShaderStage::Vertex)
        .to_destinatino("assets/shaders/.compiled/slang_post_processing.vert")
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

    ParsedScene2D::from_file(SCENE_PATH).generate_shader();

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
