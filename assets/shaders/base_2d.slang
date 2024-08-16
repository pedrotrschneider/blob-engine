import sdf2d;
import math_utils;

struct VertexOutput {
    float4 position : SV_Position;
    float2 uv : LOC0;
};

struct WindowParams {
    float2 size;
    float aspect;
    float2 mouse_position;
};

struct SimulationParams {
    float time;
};

[[vk::binding(0, 2)]]
ConstantBuffer<SimulationParams> simulation;
[[vk::binding(1, 2)]]
ConstantBuffer<WindowParams> window;

[shader("vertex")]
VertexOutput vertex(uint vertex_index : SV_VertexID) {
    VertexOutput output;
    output.uv = float2(float(vertex_index >> 1u), float(vertex_index & 1u)) * 2.0;
    output.position = float4(output.uv * float2(2.0, -2.0) + float2(-1.0, 1.0), 0.0, 1.0);
    return output;
}

[shader("fragment")]
float4 fragment(VertexOutput input) : SV_Target0 {
    float2 uv = (input.uv - 0.5) * window.size;
    uv.y = 1 - uv.y;

    SDFScene2D scene = SDFScene2D(1.0);
    
// {!!}

    return pow(float4(scene.render(), 1.0), 2.2);
}