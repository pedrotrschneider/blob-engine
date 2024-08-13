__include "shapes2d";

import math_utils;

public struct SDFScene2D {
    float aliasing;
    SDF2D sdf;

    public __init(float aliasing, float4 background_color = float4(0.0, 0.0, 0.0, 1.0)) {
        this.aliasing = aliasing;
        this.sdf.value = float.maxValue;
        this.sdf.material = SDFMaterial2D(background_color);
    }

    public float3 render() {
        return (this.sdf.material.color.rgb * smoothstep(this.aliasing, -this.aliasing, this.sdf.value));
    }

    // Trivial boolean operators

    [mutating]
    private void blend_materials(float old_sdf, SDF2D other, float smoothing = 0) {
        float color_smoothing = smoothing + this.aliasing;
        float4 other_color = float4(lerp(this.sdf.material.color.rgb * smoothstep(this.aliasing, -this.aliasing, old_sdf), other.material.color.rgb, other.material.color.a), 1.0);
        this.sdf.material.color = lerp(this.sdf.material.color, other_color, smoothstep(color_smoothing, -color_smoothing, min(other.value, other.value - old_sdf)));
    }

    [mutating]
    public void add(SDF2D other) {
        float old_sdf = this.sdf.value;
        this.sdf.value = min(this.sdf.value, other.value);
        this.blend_materials(old_sdf, other);
    }
    
    [mutating]
    public void intersect(SDF2D other) {
        float old_sdf = this.sdf.value;
        this.sdf.value = max(this.sdf.value, other.value);
        this.blend_materials(old_sdf, other);
    }
    
    [mutating]
    public void subtract(SDF2D other) {
        float old_sdf = this.sdf.value;
        this.sdf.value = max(this.sdf.value, -other.value);
        this.blend_materials(old_sdf, other);
    }

    // Smooth boolean operators

    [mutating]
    public void add_smooth(SDF2D other, float k) {
        float old_sdf = this.sdf.value;
        
        float h = saturate(0.5 + 0.5 * (other.value - this.sdf.value) / k);
        this.sdf.value = lerp(other.value, this.sdf.value, h) - k * h * (1.0 - h);

        this.blend_materials(old_sdf, other, k);
    }
    
    [mutating]
    public void intersect_smooth(SDF2D other, float k) {
        float old_sdf = this.sdf.value;
        
        float h = saturate(0.5 - 0.5 * (other.value - this.sdf.value) / k);
        this.sdf.value = lerp(other.value, this.sdf.value, h) + k * h * (1.0 - h);

        this.blend_materials(old_sdf, other, k);
    }
    
    [mutating]
    public void subtract_smooth(SDF2D other, float k) {
        float old_sdf = this.sdf.value;
        
        other.value *= -1;
        float h = saturate(0.5 - 0.5 * (other.value - this.sdf.value) / k);
        this.sdf.value = lerp(other.value, this.sdf.value, h) + k * h * (1.0 - h);
        other.value *= -1;
        
        this.blend_materials(old_sdf, other, k);
    }

    // Chamfer boolean operators

    [mutating]
    public void add_chamfer(SDF2D other, float r) {
        float old_sdf = this.sdf.value;

        this.sdf.value = min(min(this.sdf.value, other.value), (this.sdf.value - r + other.value) * sqrt(0.5));
        
        this.blend_materials(old_sdf, other, r);
    }

    [mutating]
    public void intersect_chamfer(SDF2D other, float r) {
        float old_sdf = this.sdf.value;

        this.sdf.value = max(max(this.sdf.value, other.value), (this.sdf.value + r + other.value) * sqrt(0.5));
        
        this.blend_materials(old_sdf, other, r);
    }

    [mutating]
    public void subtract_chamfer(SDF2D other, float r) {
        float old_sdf = this.sdf.value;
        
        other.value *= -1;
        this.sdf.value = max(max(this.sdf.value, other.value), (this.sdf.value + r + other.value) * sqrt(0.5));
        other.value *= -1;
        
        this.blend_materials(old_sdf, other, r);
    }

    // Round boolean operators

    [mutating]
    public void add_round(SDF2D other, float r) {
        float old_sdf = this.sdf.value;

        float2 u = max(float2(r - this.sdf.value, r - other.value), float2(0.0));
        this.sdf.value = max(r, min (this.sdf.value, other.value)) - length(u);
        
        this.blend_materials(old_sdf, other, r);
    }

    [mutating]
    public void intersect_round(SDF2D other, float r) {
        float old_sdf = this.sdf.value;

        float2 u = max(float2(r + this.sdf.value, r + other.value), float2(0.0));
        this.sdf.value = min(-r, max (this.sdf.value, other.value)) + length(u);
        
        this.blend_materials(old_sdf, other, r);
    }

    [mutating]
    public void subtract_round(SDF2D other, float r) {
        float old_sdf = this.sdf.value;

        other.value *= -1;
        float2 u = max(float2(r + this.sdf.value, r + other.value), float2(0.0));
        this.sdf.value = min(-r, max (this.sdf.value, other.value)) + length(u);
        other.value *= -1;
        
        this.blend_materials(old_sdf, other, r);
    }

    // Unary operators

    [mutating]
    public void rounded(float r) {
        this.sdf.value -= r;
    }

    [mutating]
    public void onion(float r) {
        this.sdf.value = abs(this.sdf.value) - r;
    }
}

public struct SDF2D {
    float value;
    SDFMaterial2D material;

    __init(float distance) {
        this.value = distance;
        this.material = SDFMaterial2D.WHITE();
    }

    __init(float distance, SDFMaterial2D material) {
        this.value = distance;
        this.material = material;
    }

    public SDF2D rounded(float r) {
        SDF2D new_sdf = this;
        new_sdf.value -= r;
        return new_sdf;
    }

    public SDF2D onion(float r) {
        SDF2D new_sdf = this;
        new_sdf.value = abs(this.value) - r;
        return new_sdf;
    }
}

public interface ISDFShape2D {
    public SDF2D sdf(float2 p);
    internal float calculate_sdf(float2 p);
};

public struct SDFCircle : Shape2D, ISDFShape2D {
    public float radius;

    public __init(float radius) {
        this.radius = radius;
    }

    public SDF2D sdf(float2 p) {
        p -= this.transform.position;
        p /= this.transform.scale;
        return SDF2D(this.calculate_sdf(p) * this.transform.scale, this.material);
    }

    internal float calculate_sdf(float2 p) {
        return (length(p) - this.radius);
    }
}

public struct SDFRect : Shape2D, ISDFShape2D {
    float width, height;

    public __init(float width, float height) {
        this.width = width;
        this.height = height;
    }

    public SDF2D sdf(float2 p) {
        p -= this.transform.position;
        p /= this.transform.scale;
        p = rotated(p, transform.rotation);
        return SDF2D(this.calculate_sdf(p) * this.transform.scale, this.material);
    }

    internal float calculate_sdf(float2 p) {
        float2 d = abs(p) - float2(width, height) * 0.5;
        return (length(max(d, 0.0)) + min(max(d.x, d.y), 0.0));
    }
}

public struct SDFSegment: Shape2D, ISDFShape2D {
    public float2 a, b;

    public __init(float2 a, float2 b) {
        this.a = a;
        this.b = b;
    }

    public SDF2D sdf(float2 p) {
        p -= this.transform.position;
        p /= this.transform.scale;
        p = rotated(p, transform.rotation);
        return SDF2D(this.calculate_sdf(p) * this.transform.scale, this.material);
    }

    internal float calculate_sdf(float2 p) {
        float2 pa = p - this.a, ba = this.b - this.a;
        float h = saturate(dot(pa, ba) / dot(ba, ba));
        return length(pa - ba * h);
    }
}

public struct SDFTrapezoid: Shape2D, ISDFShape2D {
    public float base_top, base_bottom, height;

    public __init(float base_top, float base_bottom, float height) {
        this.base_top = base_top;
        this.base_bottom = base_bottom;
        this.height = height;
    }

    public SDF2D sdf(float2 p) {
        p -= this.transform.position;
        p /= this.transform.scale;
        p = rotated(p, transform.rotation);
        return SDF2D(this.calculate_sdf(p) * this.transform.scale, this.material);
    }

    internal float calculate_sdf(float2 p) {
        float2 k1 = float2(this.base_bottom, this.height);
        float2 k2 = float2(this.base_bottom - this.base_top,2.0 * this.height);
        p.x = abs(p.x);
        float2 ca = float2(p.x-min(p.x,(p.y<0.0) ? this.base_top : this.base_bottom), abs(p.y) - this.height);
        float2 cb = p - k1 + k2*clamp( dot(k1-p,k2) / dot2(k2), 0.0, 1.0 );
        float s = (cb.x<0.0 && ca.y<0.0) ? -1.0 : 1.0;
        return s*sqrt(min(dot2(ca), dot2(cb)));
    }
}

public struct SDFParalelogram: Shape2D, ISDFShape2D {
    public float width, height, skew;

    public __init(float width, float height, float skew) {
        this.width = width;
        this.height = height;
        this.skew = skew;
    }

    public SDF2D sdf(float2 p) {
        p -= this.transform.position;
        p /= this.transform.scale;
        p = rotated(p, transform.rotation);
        return SDF2D(this.calculate_sdf(p) * this.transform.scale, this.material);
    }

    internal float calculate_sdf(float2 p) {
        float2 e = float2(this.skew, this.height);
        p = (p.y < 0.0) ? -p : p;
        float2 w = p - e;
        w.x -= clamp(w.x, -this.width, this.width);
        float2 d = float2(dot2(w), -w.y);
        float s = p.x * e.y - p.y * e.x;
        p = (s < 0.0) ? -p:p;
        float2 v = p - float2(this.width, 0); v -= e * clamp(dot(v, e) / dot(e, e), -1.0, 1.0);
        d = min(d, float2(dot(v, v), this.width * this.height - abs(s)));
        return sqrt(d.x) * sign(-d.y);
    }
}

public struct SDFTriangle: Shape2D, ISDFShape2D {
    public float2 p0, p1, p2;

    public __init(float2 p0, float2 p1, float2 p2) {
        this.p0 = p0;
        this.p1 = p1;
        this.p2 = p2;
    }

    public SDF2D sdf(float2 p) {
        p -= this.transform.position;
        p /= this.transform.scale;
        p = rotated(p, transform.rotation);
        return SDF2D(this.calculate_sdf(p) * this.transform.scale, this.material);
    }

    internal float calculate_sdf(float2 p) {
        float2 e0 = this.p1 - this.p0;
        float2 e1 = this.p2 - this.p1;
        float2 e2 = this.p0 - this.p2;

        float2 v0 = p - this.p0;
        float2 v1 = p - this.p1;
        float2 v2 = p - this.p2;
        
        float2 pq0 = v0 - e0 * clamp(dot(v0, e0) / dot(e0, e0), 0.0, 1.0);
        float2 pq1 = v1 - e1 * clamp(dot(v1, e1) / dot(e1, e1), 0.0, 1.0);
        float2 pq2 = v2 - e2 * clamp(dot(v2, e2) / dot(e2, e2), 0.0, 1.0);
        
        float s = sign(e0.x * e2.y - e0.y * e2.x);
        float2 d = min(min(float2(dot(pq0, pq0), s * (v0.x * e0.y - v0.y * e0.x)),
                           float2(dot(pq1, pq1), s * (v1.x * e1.y - v1.y * e1.x))),
                           float2(dot(pq2, pq2), s * (v2.x * e2.y - v2.y * e2.x)));
        return -sqrt(d.x) * sign(d.y);
    }
}

public struct SDFEquiTriangle: Shape2D, ISDFShape2D {
    public float side_length;

    public __init(float side_length) {
        this.side_length = side_length;
    }

    public SDF2D sdf(float2 p) {
        p -= this.transform.position;
        p /= this.transform.scale;
        p = rotated(p, transform.rotation);
        return SDF2D(this.calculate_sdf(p) * this.transform.scale, this.material);
    }

    internal float calculate_sdf(float2 p) {
        const float k = sqrt(3.0);
        p.x = abs(p.x) - this.side_length;
        p.y += this.side_length / k;
        if (p.x + k * p.y > 0.0) p = float2(p.x - k * p.y, -k * p.x - p.y) / 2.0;
        p.x -= clamp(p.x, -2.0 * this.side_length, 0.0);
        return -length(p) * sign(p.y);
    }
}

public struct SDFUnevenCapsule: Shape2D, ISDFShape2D {
    public float radius1, radius2, height;

    public __init(float radius1, float radius2, float height) {
        this.radius1 = radius1;
        this.radius2 = radius2;
        this.height = height;
    }

    public SDF2D sdf(float2 p) {
        p -= this.transform.position;
        p /= this.transform.scale;
        p = rotated(p, transform.rotation);
        return SDF2D(this.calculate_sdf(p) * this.transform.scale, this.material);
    }

    internal float calculate_sdf(float2 p) {
        p.x = abs(p.x);
        float b = (this.radius1 - this.radius2) / this.height;
        float a = sqrt(1.0 - b * b);
        float k = dot(p, float2(-b, a));
        if (k < 0.0) return length(p) - this.radius1;
        if (k > a * this.height) return length(p - float2(0.0, this.height)) - this.radius2;
        return dot(p, float2(a, b)) - this.radius1;
    }
}

public struct SDFRegularPentagon: Shape2D, ISDFShape2D {
    public float side_length;

    public __init(float side_length) {
        this.side_length = side_length;
    }

    public SDF2D sdf(float2 p) {
        p -= this.transform.position;
        p /= this.transform.scale;
        p = rotated(p, transform.rotation);
        return SDF2D(this.calculate_sdf(p) * this.transform.scale, this.material);
    }

    internal float calculate_sdf(float2 p) {
        p.y = 1 - p.y;
        const float3 k = float3(0.809016994, 0.587785252, 0.726542528);
        p.x = abs(p.x);
        p -= 2.0 * min(dot(float2(-k.x, k.y), p), 0.0) * float2(-k.x, k.y);
        p -= 2.0 * min(dot(float2( k.x, k.y), p), 0.0) * float2( k.x, k.y);
        p -= float2(clamp(p.x, -this.side_length * k.z, this.side_length * k.z), this.side_length);    
        return length(p) * sign(p.y);
    }
}

public struct SDFRegularHexagon: Shape2D, ISDFShape2D {
    public float side_length;

    public __init(float side_length) {
        this.side_length = side_length;
    }

    public SDF2D sdf(float2 p) {
        p -= this.transform.position;
        p /= this.transform.scale;
        p = rotated(p, transform.rotation);
        return SDF2D(this.calculate_sdf(p) * this.transform.scale, this.material);
    }

    internal float calculate_sdf(float2 p) {
        const float3 k = float3(-0.866025404, 0.5, 0.577350269);
        p = abs(p);
        p -= 2.0 * min(dot(k.xy, p), 0.0) * k.xy;
        p -= float2(clamp(p.x, -k.z * this.side_length, k.z * this.side_length), this.side_length);
        return length(p) * sign(p.y);
    }
}

public struct SDFRegularOctagon: Shape2D, ISDFShape2D {
    public float side_length;

    public __init(float side_length) {
        this.side_length = side_length;
    }

    public SDF2D sdf(float2 p) {
        p -= this.transform.position;
        p /= this.transform.scale;
        p = rotated(p, transform.rotation);
        return SDF2D(this.calculate_sdf(p) * this.transform.scale, this.material);
    }

    internal float calculate_sdf(float2 p) {
        const float3 k = float3(-0.9238795325, 0.3826834323, 0.4142135623);
        p = abs(p);
        p -= 2.0 * min(dot(float2( k.x, k.y), p), 0.0) * float2( k.x, k.y);
        p -= 2.0 * min(dot(float2(-k.x, k.y), p), 0.0) * float2(-k.x, k.y);
        p -= float2(clamp(p.x, -k.z * this.side_length, k.z * this.side_length), this.side_length);
        return length(p) * sign(p.y);
    }
}

public struct SDFStar: Shape2D, ISDFShape2D {
    public float radius, proportion;

    public __init(float radius, float proportion) {
        this.radius = radius;
        this.proportion = proportion;
    }

    public SDF2D sdf(float2 p) {
        p -= this.transform.position;
        p /= this.transform.scale;
        p = rotated(p, transform.rotation);
        return SDF2D(this.calculate_sdf(p) * this.transform.scale, this.material);
    }

    internal float calculate_sdf(float2 p) {
        p.y = 1 - p.y;
        const float2 k1 = float2(0.809016994375, -0.587785252292);
        const float2 k2 = float2(-k1.x,k1.y);
        p.x = abs(p.x);
        p -= 2.0*max(dot(k1,p),0.0)*k1;
        p -= 2.0*max(dot(k2,p),0.0)*k2;
        p.x = abs(p.x);
        p.y -= this.radius;
        float2 ba = this.proportion*float2(-k1.y,k1.x) - float2(0,1);
        float h = clamp( dot(p,ba)/dot(ba,ba), 0.0, this.radius );
        return length(p-ba*h) * sign(p.y*ba.x-p.x*ba.y);
    }
}

public struct SDFHexagram: Shape2D, ISDFShape2D {
    public float radius;

    public __init(float radius) {
        this.radius = radius;
    }

    public SDF2D sdf(float2 p) {
        p -= this.transform.position;
        p /= this.transform.scale;
        p = rotated(p, transform.rotation);
        return SDF2D(this.calculate_sdf(p) * this.transform.scale, this.material);
    }

    internal float calculate_sdf(float2 p) {
        const float4 k = float4(-0.5,0.8660254038, 0.5773502692, 1.7320508076);
        p = abs(p);
        p -= 2.0 * min(dot(k.xy, p), 0.0) * k.xy;
        p -= 2.0 * min(dot(k.yx, p), 0.0) * k.yx;
        p -= float2(clamp(p.x, this.radius * k.z, this.radius * k.w), this.radius);
        return length(p) * sign(p.y);
    }
}

public struct SDFNStar: Shape2D, ISDFShape2D {
    public float radius, sharpness;
    public int points;

    public __init(float radius, int points, float sharpness) {
        this.radius = radius;
        this.points = points;
        this.sharpness = sharpness;
    }

    public SDF2D sdf(float2 p) {
        p -= this.transform.position;
        p /= this.transform.scale;
        p = rotated(p, transform.rotation);
        return SDF2D(this.calculate_sdf(p) * this.transform.scale, this.material);
    }

    internal float calculate_sdf(float2 p) {
        float m = (1 - this.sharpness) * (this.points - 2) + 2;
        // next 4 lines can be precomputed for a given shape
        float an = 3.141593 / float(this.points);
        float en = 3.141593 / m;  // m is between 2 and n
        float2 acs = float2(cos(an), sin(an));
        float2 ecs = float2(cos(en), sin(en)); // ecs=float2(0,1) for regular polygon

        float bn = mod(atan2(p.x, p.y), 2.0 * an) - an;
        p = length(p) * float2(cos(bn), abs(sin(bn)));
        p -= this.radius * acs;
        p += ecs * clamp(-dot(p, ecs), 0.0, this.radius * acs.y / ecs.y);
        return length(p) * sign(p.x);
    }
}

public struct SDFNgon: Shape2D, ISDFShape2D {
    public float radius;
    public int sides;

    public __init(float radius, int sides) {
        this.radius = radius;
        this.sides = sides;
    }

    public SDF2D sdf(float2 p) {
        p -= this.transform.position;
        p /= this.transform.scale;
        p = rotated(p, transform.rotation);
        return SDF2D(this.calculate_sdf(p) * this.transform.scale, this.material);
    }

    internal float calculate_sdf(float2 p) {
        float an = 3.141593 / float(this.sides);
        float2 acs = float2(cos(an), sin(an));
        float2 ecs = float2(0, 1);

        float bn = mod(atan2(p.x, p.y), 2.0 * an) - an;
        p = length(p) * float2(cos(bn), abs(sin(bn)));
        p -= this.radius * acs;
        p += ecs * clamp(-dot(p, ecs), 0.0, this.radius * acs.y / ecs.y);
        return length(p) * sign(p.x);
    }
}