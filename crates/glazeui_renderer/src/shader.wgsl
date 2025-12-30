@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
    var position = array<vec2<f32>, 12>(
        vec2<f32>(-0.1, -0.5),   
        vec2<f32>(0.1, -0.5),    
        vec2<f32>(-0.1, 0.5),    
        
        vec2<f32>(0.1, -0.5),    
        vec2<f32>(0.1, 0.5),     
        vec2<f32>(-0.1, 0.5),

        vec2<f32>(-0.2, -0.5),   
        vec2<f32>(0.2, -0.6),    
        vec2<f32>(-0.2, -0.6),

        vec2<f32>(-0.2, -0.6),    
        vec2<f32>(0.2, -0.6),     
        vec2<f32>(0.2, -0.5),    
    );
    let pos = position[in_vertex_index];
    return vec4<f32>(pos, 0.0, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(46.0/255.0, 149.0/255.0, 152.0/255.0, 1.0);
}