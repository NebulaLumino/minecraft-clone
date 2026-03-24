//! Renderer module
//!
//! WGPU-based rendering pipeline for chunks and blocks.

use std::collections::HashMap;
use std::sync::Arc;
use winit::window::Window;
use winit::dpi::PhysicalSize;
use crate::game::Terrain;
use crate::engine::camera::Camera;
use crate::engine::chunk_mesh::{ChunkMesh, ChunkVertex};
use voxel_shared::{ChunkPos, BlockPos, BlockState};
use voxel_shared::world::Chunk;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{Buffer, BufferUsages, RenderPipeline, Surface, SurfaceConfiguration, TextureFormat};

/// Render pipeline state
pub struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    config: SurfaceConfiguration,
    render_pipeline: RenderPipeline,
    chunk_buffers: HashMap<ChunkPos, ChunkBuffers>,
    uniform_buffer: Buffer,
    bind_group: wgpu::BindGroup,
    size: PhysicalSize<u32>,
}

/// Camera uniforms for shader
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniforms {
    view_proj: [[f32; 4]; 4],
}

/// GPU buffers for a chunk mesh
struct ChunkBuffers {
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    index_count: u32,
    chunk_x: i32,
    chunk_z: i32,
}

impl Renderer {
    /// Create new renderer
    pub async fn new(window: Arc<Window>) -> Result<Self, String> {
        let size = window.inner_size();

        // Initialize WGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).map_err(|e| e.to_string())?;
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }).await.ok_or("Failed to request adapter")?;

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor::default(),
            None
        ).await.map_err(|e| e.to_string())?;

        let format = surface.get_capabilities(&adapter).formats.first()
            .copied()
            .unwrap_or(TextureFormat::Bgra8Unorm);

        let config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            desired_maximum_frame_latency: 2,
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        // Create camera uniform buffer
        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Camera Uniforms"),
            size: std::mem::size_of::<CameraUniforms>() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create bind group layout for camera uniforms
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Camera Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        // Create bind group for camera
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera Bind Group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        // Create shader module
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Chunk Shader"),
            source: wgpu::ShaderSource::Wgsl(CHUNK_SHADER.into()),
        });

        // Create pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        // Create render pipeline
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[ChunkVertex::desc()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                cull_mode: Some(wgpu::Face::Back),
                ..Default::default()
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multiview: None,
            multisample: wgpu::MultisampleState::default(),
        });

        Ok(Self {
            device,
            queue,
            surface,
            config,
            render_pipeline,
            chunk_buffers: HashMap::new(),
            uniform_buffer,
            bind_group,
            size,
        })
    }

    /// Resize handler
    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width == 0 || new_size.height == 0 {
            return;
        }
        self.size = new_size;
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);
    }

    /// Update camera uniforms
    pub fn update_camera(&mut self, camera: &Camera) {
        let view_proj = camera.view_proj();
        let uniforms = CameraUniforms {
            view_proj: view_proj.to_cols_array_2d(),
        };
        self.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));
    }

    /// Update chunk meshes for rendering
    pub fn update_chunks(&mut self, terrain: &Terrain) {
        let needed_chunks: Vec<ChunkPos> = terrain.chunks().keys().copied().collect();

        // Remove chunks that are no longer needed
        self.chunk_buffers.retain(|pos, _| needed_chunks.contains(pos));

        // Add/update chunks that are needed
        for chunk_pos in needed_chunks {
            if !self.chunk_buffers.contains_key(&chunk_pos) {
                if let Some(chunk) = terrain.chunks().get(&chunk_pos) {
                    if let Some(buffers) = Self::create_chunk_buffers(&self.device, chunk, chunk_pos.x, chunk_pos.z, |x, y, z| {
                        terrain.get_block(BlockPos::new(x, y, z))
                    }) {
                        self.chunk_buffers.insert(chunk_pos, buffers);
                    }
                }
            }
        }
    }

    /// Create GPU buffers for a chunk mesh
    fn create_chunk_buffers<F>(device: &wgpu::Device, chunk: &Chunk, chunk_x: i32, chunk_z: i32, get_block: F) -> Option<ChunkBuffers>
    where F: Fn(i32, i32, i32) -> BlockState {
        let mesh = ChunkMesh::generate(chunk, &get_block);

        if mesh.vertices.is_empty() {
            return None;
        }

        // Offset vertices by chunk world position
        let world_offset_x = (chunk_x * 16) as f32;
        let world_offset_z = (chunk_z * 16) as f32;

        let mut vertices_with_offset: Vec<ChunkVertex> = Vec::with_capacity(mesh.vertices.len());
        for vertex in &mesh.vertices {
            let mut v = *vertex;
            v.position[0] += world_offset_x;
            v.position[2] += world_offset_z;
            vertices_with_offset.push(v);
        }

        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices_with_offset),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });

        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&mesh.indices),
            usage: BufferUsages::INDEX | BufferUsages::COPY_DST,
        });

        Some(ChunkBuffers {
            vertex_buffer,
            index_buffer,
            index_count: mesh.indices.len() as u32,
            chunk_x,
            chunk_z,
        })
    }

    /// Render a frame
    pub fn render(&mut self) -> Result<(), String> {
        let frame = self.surface.get_current_texture().map_err(|e| e.to_string())?;
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        // Create depth texture
        let depth_texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size: wgpu::Extent3d {
                width: self.size.width,
                height: self.size.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Main Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.4,
                            g: 0.7,
                            b: 1.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);

            // Set camera bind group
            render_pass.set_bind_group(0, &self.bind_group, &[]);

            // Draw all chunks
            for (_chunk_pos, buffers) in self.chunk_buffers.iter() {
                render_pass.set_vertex_buffer(0, buffers.vertex_buffer.slice(..));
                render_pass.set_index_buffer(buffers.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                render_pass.draw_indexed(0..buffers.index_count, 0, 0..1);
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        frame.present();

        Ok(())
    }
}

impl ChunkVertex {
    /// Get vertex buffer descriptor
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<ChunkVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: 12,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: 24,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: 32,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

/// WGSL shader code for chunk rendering
const CHUNK_SHADER: &str = r#"
struct CameraUniforms {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniforms;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) tex_coord: vec2<f32>,
    @location(3) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) color: vec4<f32>,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.clip_position = camera.view_proj * vec4<f32>(input.position, 1.0);
    output.world_position = input.position;
    output.normal = input.normal;
    output.color = input.color;
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Simple directional lighting
    let light_dir = normalize(vec3<f32>(0.5, 1.0, 0.3));
    let ambient = 0.5;
    let diffuse = max(dot(input.normal, light_dir), 0.0) * 0.5;

    // Apply lighting to color
    let lighting = ambient + diffuse;
    return vec4<f32>(input.color.rgb * lighting, input.color.a);
}
"#;
