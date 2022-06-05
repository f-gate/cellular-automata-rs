use ndarray::Array3;

#[derive(Debug, Clone, Copy)]
pub struct Instance {
    pub position: cgmath::Vector3<f32>,
    pub color: [f32; 3],
}
impl Instance {
    pub fn to_raw(&self) -> InstanceRaw {
        InstanceRaw {
            model: (cgmath::Matrix4::from_translation(self.position)).into(),
            color: self.color
        }
    }
    pub fn get_instances(grid: &Array3<i8>, edge_max: i16) -> Vec<Option<Instance>> {
        const SPACE_BETWEEN: f32 = 2.5;
        let edge_max = edge_max;
        (0..edge_max).flat_map(|z| {
            (0..edge_max).flat_map(move |x| {
                (0..edge_max).map(move |y| {
                    if grid[[x as usize, y as usize, z as usize]] > 0 {
                        let x = SPACE_BETWEEN * (x as f32 - edge_max as f32 / 2.0);
                        let z = SPACE_BETWEEN * (z as f32 - edge_max as f32 / 2.0);
                        let y = SPACE_BETWEEN * (y as f32 - edge_max as f32 / 2.0);
                        
                        let y_color = 0.1;
                        let x_color = 0.1;
                        let z_color = 0.1;
                        
                        let position = cgmath::Vector3 { x, y, z };
                        
                        Some(Self {
                            position,
                            color : cgmath::Vector3 {x : x_color, y:  y_color, z : z_color}.into(),
                        })
                    }else {
                        None
                    }    
                })
            })
        }).collect::<Vec<_>>()
    }
}


//this is the data that will go into the wgpu::buffer as a matrix
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
    model: [[f32; 4]; 4],   
    pub color: [f32; 3],
}
impl InstanceRaw {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
            // We need to switch from using a step mode of Vertex to Instance
            // This means that our shaders will only change to use the next
            // instance when the shader starts processing a new instance
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    // While our vertex shader only uses locations 0, and 1 now, in later tutorials we'll
                    // be using 2, 3, and 4, for Vertex. We'll start at slot 5 not conflict with them later
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                // A mat4 takes up 4 vertex slots as it is technically 4 vec4s. We need to define a slot
                // for each vec4. We'll have to reassemble the mat4 in
                // the shader.
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 9,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}


