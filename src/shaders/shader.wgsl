struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) normal: vec3<f32>
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) normal: vec3<f32>
};

struct Camera {
    view: mat4x4<f32>,
    projection: mat4x4<f32>,
};

struct Object {
    model: mat4x4<f32>
}

struct Material {
    base_color_factor: vec4<f32>,

    emissive_factor: vec3<f32>,
    _pad0: f32,

    metallic_factor: f32,
    roughness_factor: f32,

    _pad1: vec2<f32>,
};

@group(0) @binding(0)
var<uniform> material: Material;

@group(0) @binding(1)
var t_base_color: texture_2d<f32>;

@group(0) @binding(6)
var s_base_color: sampler;

@group(1) @binding(0)
var<uniform> camera: Camera;

@group(2) @binding(0)
var<uniform> object: Object;


@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let world_position = object.model * vec4<f32>(in.position, 1.0);
    out.position = world_position.xyz;
    out.uv = in.uv;
    out.normal = normalize((object.model * vec4<f32>(in.normal, 0.0)).xyz);
    out.clip_position = camera.projection * camera.view * world_position;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let tex_color = textureSample(t_base_color, s_base_color, in.uv);
    return tex_color * material.base_color_factor;
}
