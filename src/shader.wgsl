// Vertex shader

struct Camera {
    pos: vec4<f32>,
}
@group(1) @binding(0)
var<uniform> camera: Camera;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}
struct InstanceInput {
    @location(5) model_transform: vec3<f32>,
    @location(6) is_world_space: u32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@vertex
fn vs_main(
    vertex: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var model = vertex;
    model.position.y = model.position.y * camera.pos.w; //hid the aspect ratio in the w component
    let world_position = vec4<f32>(instance.model_transform + model.position, 1.0);
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    if (instance.is_world_space == u32(0)) {
        out.clip_position = vec4<f32>(camera.pos.xyz,0.0) + world_position;
    }
    else {
        out.clip_position = world_position;
    }
    return out;
}

// Fragment shader

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0)@binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}