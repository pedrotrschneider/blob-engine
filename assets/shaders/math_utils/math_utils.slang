public float2 rotated(float2 vec, float angle) {
    float sina = sin(angle);
    float cosa = cos(angle);
    return float2(vec.x * cosa - vec.y * sina, vec.x * sina + vec.y * cosa);
}

public float2 rotated45(float2 vec) {
    return (vec + float2(vec.y, -vec.x)) * sqrt(0.5);
}

public float mod(float x, float y) {
    return x - y * floor(x / y);
}

public float mod1(float p, float size) {
    float halfsize = size * 0.5;
	float c = floor((p + halfsize) / size);
	p = mod(p + halfsize, size) - halfsize;
	p *= mod(c, 2.0) * 2.0 - 1.0;
    return p;
}

public float ndot(float2 a, float2 b ) { return a.x * b.x - a.y * b.y; }

public float dot2(float2 a) { return dot(a, a); }

// Color conversion functions implemented by Sam Hocevar (https://stackoverflow.com/questions/15095909/from-rgb-to-hsv-in-opengl-glsl)

public float4 rgb2hsv(float4 c) {
    float4 K = float4(0.0, -1.0 / 3.0, 2.0 / 3.0, -1.0);
    float4 p = lerp(float4(c.bg, K.wz), float4(c.gb, K.xy), step(c.b, c.g));
    float4 q = lerp(float4(p.xyw, c.r), float4(c.r, p.yzx), step(p.x, c.r));

    float d = q.x - min(q.w, q.y);
    float e = 1.0e-10;
    return float4(abs(q.z + (q.w - q.y) / (6.0 * d + e)), d / (q.x + e), q.x, c.a);
}

public float4 hsv2rgb(float4 c)
{
    float4 K = float4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    float3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return float4(c.z * lerp(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y), c.a);
}