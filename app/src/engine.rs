use std::{fmt, iter::once, sync::Arc};

use anyhow::{Context, Result};
use bytemuck::{cast_slice, Pod, Zeroable};

use wgpu::TextureSampleType;
use wgpu::{
    include_wgsl,
    util::{BufferInitDescriptor, DeviceExt},
    vertex_attr_array, Backends, BindGroup, BindGroupDescriptor, BindGroupEntry,
    BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType, BlendComponent,
    BlendState, Buffer, BufferAddress, BufferUsages, Color, ColorTargetState, ColorWrites,
    CommandEncoderDescriptor, Device, DeviceDescriptor, Face, Features, FragmentState, FrontFace,
    Instance, InstanceDescriptor, Limits, LoadOp, MultisampleState, Operations,
    PipelineLayoutDescriptor, PolygonMode, PowerPreference, PrimitiveState, PrimitiveTopology,
    Queue, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline,
    RenderPipelineDescriptor, RequestAdapterOptions, SamplerBindingType, ShaderStages, StoreOp,
    Surface, SurfaceConfiguration, TextureUsages, TextureViewDescriptor, TextureViewDimension,
    VertexAttribute, VertexBufferLayout, VertexState, VertexStepMode,
};
use winit::{dpi::PhysicalSize, window::Window};

use crate::resources::load_texture;

const MODEL_VERTEX_ATTRIBUTES: [VertexAttribute; 2] =
    vertex_attr_array![0 => Float32x3, 1 => Float32x2];

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct ModelVertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl ModelVertex {
    fn desc() -> VertexBufferLayout<'static> {
        use std::mem;

        VertexBufferLayout {
            array_stride: mem::size_of::<ModelVertex>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &MODEL_VERTEX_ATTRIBUTES,
        }
    }
}

const SQUARE_VERTICES: &[ModelVertex; 6] = &[
    ModelVertex {
        position: [-1.0, 1.0, 1.0],
        tex_coords: [0.0, 0.0],
    },
    ModelVertex {
        position: [-1.0, -1.0, 1.0],
        tex_coords: [0.0, 1.0],
    },
    ModelVertex {
        position: [1.0, 1.0, 1.0],
        tex_coords: [1.0, 0.0],
    },
    ModelVertex {
        position: [1.0, 1.0, 1.0],
        tex_coords: [1.0, 0.0],
    },
    ModelVertex {
        position: [-1.0, -1.0, 1.0],
        tex_coords: [0.0, 1.0],
    },
    ModelVertex {
        position: [1.0, -1.0, 1.0],
        tex_coords: [1.0, 1.0],
    },
];

pub struct Engine<'a> {
    bind_group: BindGroup,
    device: Device,
    queue: Queue,
    render_pipeline: RenderPipeline,
    surface: Surface<'a>,
    vertex_buffer: Buffer,
    window: Arc<Window>,
}

impl Engine<'_> {
    pub async fn new(window: Arc<Window>, width: u32, height: u32) -> Result<Self> {
        log::debug!("Testing logging");

        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone())?;

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .context("Requst for adapter failed")?;

        let supported_features = adapter.features();
        let webgpu_features = Features::all_webgpu_mask();
        let requested_webgpu_features = supported_features & webgpu_features;

        let device_fut = adapter.request_device(
            &DeviceDescriptor {
                label: Some("Engine.device"),
                required_features: requested_webgpu_features,
                required_limits: if cfg!(target_arch = "wasm32") {
                    Limits::downlevel_webgl2_defaults()
                } else {
                    Limits::default()
                },
            },
            None,
        );

        let surface_caps = surface.get_capabilities(&adapter);

        let format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .context("Surface does not support any sRGB texture formats")
            .unwrap_or(surface_caps.formats[0]);

        //TODO: ensure we have a usable present_mode and alpha_mode
        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format,
            width,
            height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        //TODO: why does unwrap work here, while using a ? causes a compile error using wasm-pack?
        let (device, queue) = device_fut.await.unwrap();
        log::debug!("About to configure surface {:?} using config {:?}", surface, config);
        surface.configure(&device, &config);

        let texture = load_texture("blue_square_arrows_up_right.png", false, &device, &queue)
            .await
            .unwrap();

        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Engine.bind_group_layout"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: true },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Engine.bind_group"),
            layout: &bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&texture.view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&texture.sampler),
                },
            ],
        });

        let module = device.create_shader_module(include_wgsl!("shader.wgsl"));

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Engine::new pipeline_layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Engine.render_pipeline"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &module,
                entry_point: "vs_main",
                buffers: &[ModelVertex::desc()],
            },
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(FragmentState {
                module: &module,
                entry_point: "fs_main",
                targets: &[Some(ColorTargetState {
                    format: config.format,
                    blend: Some(BlendState {
                        alpha: BlendComponent::REPLACE,
                        color: BlendComponent::REPLACE,
                    }),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            multiview: None,
        });

        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Engine.vertex_buffer"),
            contents: cast_slice(SQUARE_VERTICES),
            usage: BufferUsages::VERTEX,
        });

        let r = Self {
            bind_group,
            device,
            queue,
            render_pipeline,
            surface,
            vertex_buffer,
            window: window.clone(),
        };

        println!("Initialized {}", r);

        Ok(r)
    }

    pub fn render(&self) -> Result<()> {
        let target = self.surface.get_current_texture()?;
        let view = target.texture.create_view(&TextureViewDescriptor {
            label: Some("Engine::render target texture"),
            ..Default::default()
        });
        // println!("TextureView: {:#?}", view);
        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Engine::render CommandEncoder"),
            });
        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Engine::render RenderPass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_bind_group(0, &self.bind_group, &[]);
            render_pass.draw(0..6, 0..1);
        }
        self.queue.submit(once(encoder.finish()));
        target.present();
        Ok(())
    }
}

impl fmt::Display for Engine<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let PhysicalSize { width, height } = self.window.inner_size();
        write!(f, "Engine {{ window: {}x{} }}", width, height)
    }
}
