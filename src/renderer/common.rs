#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Zeroable, bytemuck::Pod)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }

    pub fn new(x: f32, y: f32, z: f32, u: f32, v: f32) -> Self {
        Self { position: [x, y, z], tex_coords: [u, v] }
    }
}


pub struct Mesh {
    pub(crate) vertex_buffer: wgpu::Buffer,
    pub(crate) index_buffer: Option<wgpu::Buffer>,
    pub(crate) len_vertices: u32,
}