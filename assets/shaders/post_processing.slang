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
    
    SDFCircle circle = SDFCircle(250);
    circle.material = SDFMaterial2D.RED();
    circle.transform.position.x = sin(simulation.time * 0.3) * window.size.x * 0.2;
    scene.add(circle.sdf(uv));

    
    SDFRect rect = SDFRect(300, 600);
    rect.material.color = float4(0.0, 0.7, 0.4, 0.5);
    scene.add(rect.sdf(uv).onion(30));
    // scene.add_smooth(rect.sdf(uv).onion(30), 100.0);
    // scene.subtract_chamfer(rect.sdf(uv), 20.0);
    // scene.add_round(rect.sdf(uv), 50.0);

    SDFCircle circle2 = SDFCircle(250);
    circle2.material = SDFMaterial2D.BLUE();
    circle2.material.color.a = 0.5;
    circle2.transform.position.y = sin(simulation.time * 0.3) * window.size.y * 0.2;
    scene.add_smooth(circle2.sdf(uv).onion(30), 50);

    SDFCircle circle3 = SDFCircle(250);
    circle3.material = SDFMaterial2D.GREEN();
    circle3.transform.position = window.mouse_position;
    scene.add(circle3.sdf(uv));

    return pow(float4(scene.render(), 1.0), 2.2);
}