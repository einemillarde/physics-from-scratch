struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) tangent: vec4<f32>
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) tangent: vec3<f32>,
    @location(4) bitangent: vec3<f32>
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
var base_color_texture: texture_2d<f32>;

@group(0) @binding(3)
var normal_texture: texture_2d<f32>;

@group(0) @binding(6)
var material_sampler: sampler;

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

    let N = normalize((object.model * vec4<f32>(in.normal, 0.0)).xyz);
    let T = normalize((object.model * vec4<f32>(in.tangent.xyz, 0.0)).xyz);
    let B = cross(N, T) * in.tangent.w;

    out.normal = N;
    out.tangent = T;
    out.bitangent = B;

    out.clip_position = camera.projection * camera.view * world_position;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let base_color = textureSample(base_color_texture, material_sampler, in.uv) * material.base_color_factor;

    let tangent_normal = textureSample(normal_texture, material_sampler, in.uv).xyz * 2.0 - 1.0;

    let TBN = mat3x3<f32>(
        normalize(in.tangent),
        normalize(in.bitangent),
        normalize(in.normal)
    );

    let world_normal = normalize(TBN * tangent_normal);

    let light_dir = normalize(vec3<f32>(1.0, 1.0, 1.0));

    let diffuse = max(dot(world_normal, light_dir), 0.0);

    let ambient = 0.1;

    return vec4<f32>(
        base_color.rgb * (ambient + diffuse),
        base_color.a
    );
}
