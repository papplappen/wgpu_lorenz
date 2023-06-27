struct Instance {
    pos: vec3<f32>,
    color: vec3<f32>,
}

struct LorenzConfig {
    /* @location(0) */ rho: f32,
    /* @location(1) */ sigma: f32,
    /* @location(2) */ beta: f32,
}

@group(0) @binding(0)
var<storage, read_write> instances: array<Instance>;

fn lorenz_delta(config: LorenzConfig, state: vec3<f32>) -> vec3<f32> {
    let x = state.x;
    let y = state.y;
    let z = state.z;
    return vec3<f32>(
        config.sigma * (y - x),
        x * (config.rho - z) - y,
        x * y - config.beta * z,
    );
}

fn lorenz_step(config: LorenzConfig, dt: f32, state: vec3<f32>) -> vec3<f32> {
    return state + dt * lorenz_delta(config, state);
}


@compute
@workgroup_size(1)
fn cs_main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let i = global_id.x * NUM_WORKGROUPS_PER_DIM * NUM_WORKGROUPS_PER_DIM
          + global_id.y * NUM_WORKGROUPS_PER_DIM
          + global_id.z;

    let config = LorenzConfig(28., 10., 8./3.);

    instances[i].pos = lorenz_step(config, 0.01, instances[i].pos);

    instances[i].color = vec3<f32>(0.1,0.4,0.8);
}