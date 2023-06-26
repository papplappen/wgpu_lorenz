const SIZE = 100u;

struct Instance {
    @location(0) pos: vec3<f32>,
    @location(1) color: vec3<f32>,
}

struct LorenzConfig {
    @location(0) rho: f32,
    @location(1) sigma: f32,
    @location(2) beta: f32,
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
@workgroup_size(1,1,1)
fn cs_main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let i = flatten_global_id(global_id);

    let config = LorenzConfig(28., 10., 8./3.);

    instances[i].pos = lorenz_step(config, 0.001, instances[i].pos);
}

fn flatten_global_id(global_id: vec3<u32>) -> u32 {
    return global_id.x * SIZE * SIZE + global_id.y * SIZE + global_id.z;
}