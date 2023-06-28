struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) model_position: vec3<f32>,
    @location(1) color: vec4<f32>,
};

struct VertexInput {
    @location(0) position: vec3<f32>,
}

struct CameraUniform {
    view_proj: mat4x4<f32>,
}

struct InstanceInput {
    @location(1) @size(16) pos: vec3<f32>,
    @location(2) @size(16) color: vec3<f32>,
}

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

const POINT_RADIUS = .5;
const ASPECT_RATIO = 0.5625;
@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput
) -> VertexOutput {
    let ppos = camera.view_proj * vec4<f32>(instance.pos, 1.0);

    let pos = ppos + POINT_RADIUS * vec4<f32>(ASPECT_RATIO * model.position.x, model.position.y,0.,0.);

    return VertexOutput(pos, model.position, vec4<f32>(instance.color, 1.0));
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let radius_sq = dot(in.model_position, in.model_position);
    if radius_sq < 0.25 {
        // return in.color;
        let c = smoothstep(-1., 1., in.model_position.x + in.model_position.y);
        // return vec4<f32>(vec3<f32>(c), 1.);
        return c*in.color;

    } else {
        // return vec4<f32>(0.,0.,0.,1.);
        discard;
    }
}
