implementing sdf2d;

import math_utils;

public struct Transform2D {
    public float2 position;
    public float scale;
    public float rotation;
    
    __init() {
        this.position = float2(0.0);
        this.scale = 1.0;
        this.rotation = 0.0;
    }
    
    __init(float2 position, float scale, float rotation) {
        this.position = position;
        this.scale = scale;
        this.rotation = rotation;
    }
}

public struct SDFMaterial2D {
    public static SDFMaterial2D WHITE() { return SDFMaterial2D(); }
    public static SDFMaterial2D RED() { return SDFMaterial2D(float4(1.0, 0.0, 0.0, 1.0)); }
    public static SDFMaterial2D GREEN() { return SDFMaterial2D(float4(0.0, 1.0, 0.0, 1.0)); }
    public static SDFMaterial2D BLUE() { return SDFMaterial2D(float4(0.0, 0.0, 1.0, 1.0)); }
    
    public float4 color;
    public float rouding;
    public float onion;
    
    public __init() {
        this.color = float4(1.0, 1.0, 1.0, 1.0);
    }
    
    public __init(float4 color) {
        this.color = color;
    }
}

[open]
public struct Shape2D {
    public Transform2D transform;
    public SDFMaterial2D material;
    
    __init() {
        this.transform = Transform2D();
    }

    [mutating]
    public void translate(float x, float y) {
        this.translate(float2(x, y));
    }
    
    [mutating]
    public void translate(float2 translation) {
        this.transform.position += translation;
    }
    
    [mutating]
    public void scale(float scale) {
        this.transform.scale *= scale;
    }
    
    [mutating]
    public void rotate(float rotation) {
        this.transform.rotation += rotation;
    }
}