//TODO


struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

struct VertexInput {
    @location(0) position: vec3<f32>,
}

struct CameraUniform {
    view_proj: mat4x4<f32>
}
struct InstanceInput {
    @location(1) pos: vec3<f32>,
    @location(2) color: vec3<f32>,
}

@group(0) @binding(0)
var<uniform> camera: CameraUniform;



@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput
) -> VertexOutput {
    let pos = camera.view_proj * ((vec4<f32>(instance.pos, 1.0) + vec4<f32>(model.position, 1.0)));

    return VertexOutput(pos, vec4<f32>(instance.color, 1.0));
}



@fragment
fn fs_main(in: VertexOutput) -> @ location(0) vec4<f32> {
        return in.color;
}
