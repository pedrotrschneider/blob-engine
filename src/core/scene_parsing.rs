use std::fs;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ParsedVec2 {
    x: f32,
    y: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ParsedVec3 {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ParsedVec4 {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ParsedColor {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ParsedTransform2D {
    position: ParsedVec2,
    rotation: f32,
    scale: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ParsedMaterial2D {
    color: ParsedColor,
    onion: Option<f32>,
    rounding: Option<f32>,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(i32)]
pub enum ParsedShape2DType {
    Circle,
    Rect,
    Segment,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ParsedShape2D {
    shape_id: ParsedShape2DType,
    data: Vec<f32>,
    transform: ParsedTransform2D,
    material: ParsedMaterial2D,
}

impl ParsedShape2D {
    pub fn generate_shader_string(&self, index: usize) -> String {
        let mut shape_code = "".to_owned();

        match self.shape_id {
            ParsedShape2DType::Circle => shape_code += &format!("    SDFCircle shape{} = SDFCircle({});\n", index, self.data[0]),
            ParsedShape2DType::Rect => {
                shape_code += &format!("    SDFRect shape{} = SDFRect({}, {});\n", index, self.data[0], self.data[1])
            }
            ParsedShape2DType::Segment => {
                shape_code += &format!(
                    "    SDFSegment shape{} = SDFSegment(float2({}, {}), float2({}, {}));\n",
                    index, self.data[0], self.data[1], self.data[2], self.data[3]
                )
            }
        }

        shape_code += &format!("    shape{}.transform.position.x = {};\n", index, self.transform.position.x);
        shape_code += &format!("    shape{}.transform.position.y = {};\n", index, self.transform.position.y);
        shape_code += &format!("    shape{}.transform.rotation = {};\n", index, self.transform.rotation);
        shape_code += &format!("    shape{}.transform.scale = {};\n", index, self.transform.scale);

        shape_code += &format!("    shape{}.material.color.r = {};\n", index, self.material.color.r);
        shape_code += &format!("    shape{}.material.color.g = {};\n", index, self.material.color.g);
        shape_code += &format!("    shape{}.material.color.b = {};\n", index, self.material.color.b);
        shape_code += &format!("    shape{}.material.color.a = {};\n", index, self.material.color.a);

        if let Some(onion) = self.material.onion {
            shape_code += &format!("    scene.add(shape{}.sdf(uv).onion({}));", index, onion);
        } else if let Some(rounding) = self.material.rounding {
            shape_code += &format!("    scene.add(shape{}.sdf(uv).rounded({}));", index, rounding);
        } else {
            shape_code += &format!("    scene.add(shape{}.sdf(uv));", index);
        }

        return shape_code;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParsedScene2D {
    name: String,
    aliasing: f32,
    gamma: f32,
    shapes: Vec<ParsedShape2D>,
}

impl ParsedScene2D {
    const BASE_2D_SHADER_PATH: &'static str = "assets/shaders/base_2d.slang";

    pub fn generate_shader(&self) {
        let shader_str = fs::read_to_string(Self::BASE_2D_SHADER_PATH).expect(&format!(
            "Unable to read base 2d shader in path {}",
            Self::BASE_2D_SHADER_PATH
        ));

        let mut scene_str = format!("    SDFScene2D scene = SDFScene2D({});\n", self.aliasing);
        for (i, shape) in self.shapes.iter().enumerate() {
            scene_str += &shape.generate_shader_string(i);
        }
        scene_str += &format!("    return pow(float4(scene.render(), 1.0), {});", self.gamma);

        let shader_path = format!(
            "assets/shaders/.generated/{}.slang",
            self.name.to_lowercase().replace(" ", "_")
        );
        fs::write(shader_path, shader_str.replace("// {!!}", &scene_str)).expect("Unable to write shader for scene 1");
    }
}

pub struct SceneParser;

impl SceneParser {
    pub fn parse_scene2d(scene_path: &str) -> ParsedScene2D {
        let scene_str = fs::read_to_string(scene_path).expect(&format!("Unable to read scene at path {}", scene_path));
        let scene: ParsedScene2D = serde_json::from_str(&scene_str).unwrap();
        return scene;
    }
}
