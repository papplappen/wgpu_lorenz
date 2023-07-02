struct Instance {
    pos: vec3<f32>,
    color: vec3<f32>,
}

struct LorenzConfig {
    rho: f32,
    sigma: f32,
    beta: f32,
    step_size_factor: f32,
}
struct Config {
    lorenz: LorenzConfig,
    num_workgroups: vec3<u32>,
}

@group(0) @binding(0)
var<storage, read_write> instances: array<Instance>;

@group(0) @binding(1)
var<uniform> config: Config;

@group(0) @binding(2)
var<uniform> delta_time: f32;

fn lorenz_vel(lorenz_config: LorenzConfig, state: vec3<f32>) -> vec3<f32> {
    let x = state.x;
    let y = state.y;
    let z = state.z;
    return vec3<f32>(
        lorenz_config.sigma * (y - x),
        x * (lorenz_config.rho - z) - y,
        x * y - lorenz_config.beta * z,
    );
}

// fn lorenz_step(lorenz_config: LorenzConfig, dt: f32, state: vec3<f32>) -> vec3<f32> {
//     return state + lorenz_config.step_size_factor * dt * lorenz_delta(lorenz_config, state);
// }


@compute
@workgroup_size(1)
fn cs_main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let i = global_id.x * config.num_workgroups.x * config.num_workgroups.x
          + global_id.y * config.num_workgroups.y
          + global_id.z;

    let vel = lorenz_vel(config.lorenz, instances[i].pos);
    let step = config.lorenz.step_size_factor * delta_time * vel;
    
    instances[i].pos += step;
    instances[i].color = vel_to_color(vel);
}

const VEL_SCALE = 0.0075;
const SLOW_COLOR = vec3<f32>(.66, .74, .89);
const FAST_COLOR = vec3<f32>(.89, .44, .11);

fn vel_to_color(vel: vec3<f32>) -> vec3<f32> {
    
    let mag = length(vel);
    let value = VEL_SCALE * mag;
    return mix(SLOW_COLOR, FAST_COLOR, value);
    // return gradient(value);
}

// fn gradient(value : f32) -> vec3<f32> {
//     let i = u32(saturate(value) * 33.);
//     return vec3<f32>(TEST[i]);
// }