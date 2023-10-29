use crate::{
    camera::{Camera, CameraStruct},
    prefabs::Prefab,
    prelude::{Vertex, Instance},
    loader::load_texture,
    shader,
    structs::{CameraController, Mesh},
    texture, window, resources::{UpdateInstance, DeltaTime, WindowEvents, MouseClickType}, primitives::rect, material::Material,
};
use bevy_ecs::prelude::*;
use glam::Vec2;
use instant::Duration;
use slab::Slab;
use wgpu::util::DeviceExt;
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, KeyboardInput, MouseButton, WindowEvent},
    event_loop::EventLoop,
    window::Window,
};
pub struct State {
    pub device: wgpu::Device,
    pub config: wgpu::SurfaceConfiguration,
    pub render_pipeline: wgpu::RenderPipeline,
    pub camera: CameraStruct,
    pub depth_texture: texture::Texture,
    pub window: window::Window,
    pub texture_bind_group_layout: wgpu::BindGroupLayout,
    pub mouse_locked: bool,
    build_path: String,
}

impl State {
    pub async fn new(
        mouse_lock: bool,
        build_path: &str,
        cam: Camera,
        speed: f32,
        sensitivity: f32,
    ) -> (Self, EventLoop<()>, World, Schedule) {
        let (window, event_loop) = window::Window::new(mouse_lock).await;
        let (device, queue) = window
            .adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                },
                None,
            )
            .await
            .unwrap();

        log::warn!("Surface");
        let surface_caps = window.surface.get_capabilities(&window.adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: window.size.width,
            height: window.size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        window.surface.configure(&device, &config);

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        //camera
        let camera = CameraStruct::new(
            &device,
            &config,
            cam,
            CameraController::new(speed, sensitivity),
        );

        let depth_texture =
            texture::Texture::create_depth_texture(&device, &config, "depth_texture");
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout, &camera.bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = shader::make_shader(
            include_str!("shader.wgsl"),
            &device,
            render_pipeline_layout,
            &config,
        );
        window.window.set_visible(true);
        let mut world = World::new();
        world.insert_resource(UpdateInstance {
            queue,
            prefab_slab: Slab::new(),
        });
        world.insert_resource(DeltaTime { dt: Duration::ZERO });
        world.insert_resource(WindowEvents { keys_pressed: vec![], screen_mouse_pos: PhysicalPosition { x: 0.0, y: 0.0 }, world_mouse_pos: PhysicalPosition { x: 0.0, y: 0.0 },left_mouse: MouseClickType::NotHeld, right_mouse: MouseClickType::NotHeld, middle_mouse: MouseClickType::NotHeld, aspect_ratio: (config.width as f32)/(config.height as f32) });
        let schedule = Schedule::default();
        (
            
            Self {
                device,
                config,
                render_pipeline,
                camera,
                depth_texture,
                window,
                texture_bind_group_layout,
                mouse_locked: mouse_lock,
                build_path: build_path.to_string(),
            },
            event_loop,
            world,
            schedule,
        )
    }
    pub fn window(&self) -> &Window {
        &self.window.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>, window_events: &mut WindowEvents) {
        if new_size.width > 0 && new_size.height > 0 {
            self.camera.camera_uniform.update_screen_size(new_size.width,new_size.height);
            window_events
                    .update_aspect_ratio(new_size.width, new_size.height);
            self.window.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.window.surface.configure(&self.device, &self.config);
            self.depth_texture =
                texture::Texture::create_depth_texture(&self.device, &self.config, "depth_texture");
        }
    }
    pub fn input(&mut self, event: &WindowEvent, window_events: &mut WindowEvents) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(key),
                        state,
                        ..
                    },
                ..
            } => {
                let key_pressed = (*key, *state);
                window_events
                    .keys_pressed
                    .push(key_pressed);
                self.camera.camera_controller.process_keyboard(*key, *state)
            }
            WindowEvent::MouseWheel { delta, .. } => {
                self.camera.camera_controller.process_scroll(delta);
                true
            }
            WindowEvent::MouseInput {
                button,
                state,
                ..
            } => {
                match button {
                    MouseButton::Left => window_events.left_mouse = if *state == ElementState::Pressed {MouseClickType::Clicked} else {MouseClickType::Released},
                    MouseButton::Right => window_events.right_mouse = if *state == ElementState::Pressed {MouseClickType::Clicked} else {MouseClickType::Released},
                    MouseButton::Middle => window_events.middle_mouse = if *state == ElementState::Pressed {MouseClickType::Clicked} else {MouseClickType::Released},
                    _ => {}
                }
                true
            }
            _ => false,
        }
    }
    pub fn update(&mut self, world: &mut World) {
        self.camera
            .camera_uniform
            .update_view_proj(&self.camera.camera_transform);
        let queue = world.get_resource_mut::<UpdateInstance>().unwrap();
        queue.queue.write_buffer(
            &self.camera.buffer,
            0,
            bytemuck::cast_slice(&[self.camera.camera_uniform]),
        );
    }
    pub async fn compile_material(&self, texture_name: &str, world: &mut World) -> Material {
        let queue = world.get_resource::<UpdateInstance>().unwrap();
        let diffuse_texture =
            load_texture(texture_name, &self.build_path, &self.device, &queue.queue)
                .await
                .unwrap();
        let texture_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                },
            ],
            label: None,
        });
        Material {
            bind_group: texture_bind_group,
        }
    }
    pub fn build_mesh(
        &mut self,
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
        instances: Vec<&mut Instance>,
        material: Material,
        is_updating: bool,
        world: &mut World,
    ) {
        let vertex_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });
        let index_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX,
            });
        let mesh = Mesh {
            vertex_buffer,
            index_buffer,
            num_elements: indices.len() as u32,
            material,
        };
        let mut instance_data = vec![];
        let mut length = 0;
        for instance in &instances {
            let instance_raw = instance.to_raw();
            if instance_raw.is_some() {
                instance_data.push(instance_raw.unwrap());
                length += 1;
            }
        }
        let instance_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: if is_updating {
                    wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
                } else {
                    wgpu::BufferUsages::VERTEX
                },
            });
        let container = Prefab::new(
            instance_buffer,
            mesh,
            length,
        );
        let mut update_instance = world.get_resource_mut::<UpdateInstance>().unwrap();
        let entry = update_instance.prefab_slab.vacant_entry();
        let key = entry.key();
        for instance in instances {
            instance.prefab_index = key;
        }
        entry.insert(container);
    }
    pub fn make_sprites(
        &mut self,
        instances: Vec<&mut Instance>,
        material: Material,
        is_updating: bool,
        world: &mut World,
    ) {
        //make sprite mesh
        let (vertices, indices) = rect(Vec2::new(0.5,0.5), Vec2::new(-0.5,-0.5));
        let vertex_buffer = self.device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });
        let index_buffer = self.device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX,
            });
            
        let mesh = Mesh {
            vertex_buffer,index_buffer, num_elements: indices.len() as u32,
            material,
        };
        let mut instance_data = vec![];
        let mut length = 0;
        for instance in &instances {
            let instance_raw = instance.to_raw();
            if instance_raw.is_some() {
                instance_data.push(instance_raw.unwrap());
                length += 1;
            }
        }
        let instance_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: if is_updating {
                    wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
                } else {
                    wgpu::BufferUsages::VERTEX
                },
            });
        let container = Prefab::new(
            instance_buffer,
            mesh,
            length,
        );
        let mut update_instance = world.get_resource_mut::<UpdateInstance>().unwrap();
        let entry = update_instance.prefab_slab.vacant_entry();
        let key = entry.key();
        for instance in instances {
            instance.prefab_index = key;
        }
        entry.insert(container);
    }
}
