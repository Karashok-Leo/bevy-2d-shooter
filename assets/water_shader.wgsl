#import bevy_render::globals::Globals
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(0) @binding(1) var<uniform> globals: Globals;

@group(2) @binding(0) var<uniform> radial_scale: f32;
@group(2) @binding(1) var<uniform> axial_scale: f32;
@group(2) @binding(2) var<uniform> contrast: f32;
@group(2) @binding(3) var<uniform> speed: f32;
@group(2) @binding(4) var<uniform> intensity: f32;
@group(2) @binding(5) var<uniform> color_offset: vec3<f32>;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var uv = in.uv;
    uv -= vec2<f32>(0.5, 0.5);
    var uv_len = length(uv);
    var angle = atan2(uv.y, uv.x);
    uv_len *= radial_scale;
    angle *= axial_scale;
    for (var i: f32 = 1.0; i < contrast; i += 1.0) {
        let delta = globals.time * speed;
        uv_len += intensity * sin(angle * i * i + delta) * sin(uv_len * i * i + delta);
    }
    return vec4(color_offset - uv_len, 1.0);
}