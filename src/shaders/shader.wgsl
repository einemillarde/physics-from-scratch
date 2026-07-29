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
    position: vec3<f32>,
    _pad0: u32,
    view: mat4x4<f32>,
    projection: mat4x4<f32>,
};

struct Object {
    model: mat4x4<f32>
}

struct Material {
    base_color_factor: vec4<f32>,

    emissive_factor: vec3<f32>,
    _pad0: u32,

    metallic_factor: f32,
    roughness_factor: f32,
    _pad1: vec2<u32>
};

struct Light {
    position_or_direction: vec3<f32>,
    light_type: u32,

    color: vec3<f32>,
    intensity: f32,

    _pad0: vec3<u32>,
    range: f32,
};

struct LightCount {
    _pad0: vec3<u32>,
    count: u32,
};

@group(0) @binding(0)
var<uniform> material: Material;

@group(0) @binding(1)
var base_color_texture: texture_2d<f32>;

@group(0) @binding(2)
var metallic_roughness_texture: texture_2d<f32>;

@group(0) @binding(3)
var normal_texture: texture_2d<f32>;

@group(0) @binding(6)
var material_sampler: sampler;

@group(1) @binding(0)
var<uniform> camera: Camera;

@group(2) @binding(0)
var<uniform> object: Object;

@group(3) @binding(0)
var<storage, read> lights: array<Light>;

@group(3) @binding(1)
var<uniform> light_count: LightCount;

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

    let view_direction = normalize(camera.position - in.position);

    let roughness = textureSample(metallic_roughness_texture, material_sampler, in.uv).g * material.roughness_factor;

    let shininess = max(2.0, pow(1.0 - roughness, 4.0) * 128.0);

    let metallic = textureSample(metallic_roughness_texture, material_sampler, in.uv).b * material.metallic_factor;

    var diffuse_lighting = vec3<f32>(0.0);
    var specular_lighting = vec3<f32>(0.0);

    for (var i = 0u; i < light_count.count; i++) {
        let light = lights[i];

        switch light.light_type {
            case 0u: { // Point Light
                let to_light = light.position_or_direction - in.position;
                let distance = length(to_light);

                if (distance > light.range) { continue; }

                let light_direction = normalize(to_light);

                let attenuation = light.intensity / (distance * distance + 0.01);
                let diffuse = max(dot(world_normal, light_direction), 0.0);

                let halfway_direction = normalize(light_direction + view_direction);
                let specular = pow(max(dot(world_normal, halfway_direction), 0.0), shininess);
                let specular_color = mix(vec3<f32>(1.0), base_color.rgb, metallic);

                diffuse_lighting += light.color * attenuation * diffuse * (1.0 - metallic);
                specular_lighting += light.color * attenuation * specular_color * specular;
            }
            case 1u { // Directional Light
                let light_direction = normalize(-light.position_or_direction);

                let diffuse = max(dot(world_normal, light_direction), 0.0);

                let halfway_direction = normalize(light_direction + view_direction);
                let specular = pow(max(dot(world_normal, halfway_direction), 0.0), shininess);
                let specular_color = mix(vec3<f32>(1.0), base_color.rgb, metallic);

                diffuse_lighting += light.color * light.intensity * diffuse * (1.0 - metallic);
                specular_lighting += light.color * light.intensity * specular_color * specular;
            }
            default: {
                continue;
            }
        }
    }

    let final_color = base_color.rgb * diffuse_lighting + specular_lighting;

    return vec4<f32>(final_color, base_color.a);
}
