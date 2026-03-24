//! VoxelCraft - Minecraft Clone
//!
//! Single binary that can run as:
//! - Single-player (default, windowed)
//! - Dedicated server (--server)

mod engine;
mod game;
mod storage;
mod ui;
mod network;

use std::env;
use tracing::{info, error, Level};
use tracing_subscriber::FmtSubscriber;

fn main() {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");

    // Parse command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "--server" => {
                info!("Starting VoxelCraft dedicated server");
                run_server();
            }
            "--help" | "-h" => {
                println!("VoxelCraft - Minecraft Clone");
                println!();
                println!("Usage:");
                println!("  voxel              # Single-player mode (windowed)");
                println!("  voxel --server     # Start dedicated server");
                println!("  voxel --help       # Show this help");
                std::process::exit(0);
            }
            _ => {
                error!("Unknown argument: {}", args[1]);
                println!("Unknown argument. Use --help for usage.");
                std::process::exit(1);
            }
        }
    } else {
        info!("Starting VoxelCraft single-player");
        run_singleplayer_windowed();
    }
}

// =====================================================================
// WINDOWED SINGLE-PLAYER MODE
// =====================================================================

use winit::{
    event::{WindowEvent, MouseButton, ElementState, KeyEvent, StartCause},
    event_loop::{EventLoop, ActiveEventLoop},
    dpi::PhysicalSize,
};

/// Application state for the game
struct VoxelApp {
    game: game::Game,
    camera: engine::camera::Camera,
    last_cx: i32,
    last_cz: i32,
    load_radius: i32,
    keys: std::collections::HashMap<winit::keyboard::KeyCode, bool>,
    mouse_delta: (f32, f32),
    renderer: Option<engine::renderer::Renderer>,
    window: Option<std::sync::Arc<winit::window::Window>>,
    need_update_chunks: bool,
    window_size: PhysicalSize<u32>,
}

impl VoxelApp {
    fn new() -> Self {
        let game = game::Game::new();
        let camera = engine::camera::Camera::new(game.player.position);

        let pos = game.player.position();
        let cx = pos.x as i32 / 16;
        let cz = pos.z as i32 / 16;

        Self {
            game,
            camera,
            last_cx: cx,
            last_cz: cz,
            load_radius: 4,
            keys: std::collections::HashMap::new(),
            mouse_delta: (0.0, 0.0),
            renderer: None,
            window: None,
            need_update_chunks: true,
            window_size: PhysicalSize::new(1280, 720),
        }
    }

    fn load_chunks(&mut self) {
        for dx in -self.load_radius..=self.load_radius {
            for dz in -self.load_radius..=self.load_radius {
                let chunk_pos = voxel_shared::ChunkPos::new(self.last_cx + dx, self.last_cz + dz);
                let _ = self.game.world.get_chunk(chunk_pos);
            }
        }
        self.need_update_chunks = true;
    }
}

impl winit::application::ApplicationHandler<()> for VoxelApp {
    fn new_events(&mut self, _event_loop: &ActiveEventLoop, _cause: StartCause) {
        // Nothing needed here
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Create renderer when resumed (first time or after suspend)
        if self.renderer.is_none() {
            let window = event_loop.create_window(
                winit::window::WindowAttributes::default()
                    .with_title("VoxelCraft")
                    .with_inner_size(self.window_size)
            ).expect("Failed to create window");

            println!("Game initialized!");
            println!("World seed: {}", self.game.world.seed());
            println!("Player position: {:?}", self.game.player.position());
            println!();
            println!("Controls:");
            println!("  WASD - Move");
            println!("  Mouse - Look");
            println!("  Space - Jump");
            println!("  ESC - Quit");
            println!();

            // Load initial chunks
            self.load_chunks();
            println!("Loaded {} chunks", self.game.world.loaded_chunks().len());
            println!();

            // Store window as Arc so it stays alive
            let window = std::sync::Arc::new(window);
            self.window = Some(window.clone());

            // Create renderer using tokio runtime
            let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
            let renderer = rt.block_on(engine::renderer::Renderer::new(window)).expect("Failed to create renderer");
            self.renderer = Some(renderer);
        }
    }

    fn window_event(&mut self, _event_loop: &ActiveEventLoop, _window_id: winit::window::WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                std::process::exit(0);
            }
            WindowEvent::KeyboardInput { device_id: _, event: KeyEvent { state, physical_key, .. }, is_synthetic: _ } => {
                let key_code = match physical_key {
                    winit::keyboard::PhysicalKey::Code(code) => code,
                    winit::keyboard::PhysicalKey::Unidentified(_) => return,
                };
                let pressed = state == ElementState::Pressed;
                self.keys.insert(key_code, pressed);

                if key_code == winit::keyboard::KeyCode::Escape && pressed {
                    std::process::exit(0);
                }
            }
            WindowEvent::MouseInput { device_id: _, state, button } => {
                if button == MouseButton::Left {
                    if state == ElementState::Pressed {
                        // Break block
                        if let Some(target) = self.game.player.get_targeted_block(&self.game.world, 5.0) {
                            tracing::info!("Breaking block at {:?}", target.position);
                            self.game.world.set_block(target.position, voxel_shared::BlockState::AIR);
                            self.need_update_chunks = true;
                        }
                    }
                } else if button == MouseButton::Right {
                    if state == ElementState::Pressed {
                        // Place block (dirt)
                        let block_state = voxel_shared::BlockState { id: 3, properties: 0 };
                        if let Some(target) = self.game.player.get_targeted_block(&self.game.world, 5.0) {
                            let place_pos = match target.face {
                                0 => voxel_shared::BlockPos::new(target.position.x + 1, target.position.y, target.position.z),
                                1 => voxel_shared::BlockPos::new(target.position.x - 1, target.position.y, target.position.z),
                                2 => voxel_shared::BlockPos::new(target.position.x, target.position.y + 1, target.position.z),
                                3 => voxel_shared::BlockPos::new(target.position.x, target.position.y - 1, target.position.z),
                                4 => voxel_shared::BlockPos::new(target.position.x, target.position.y, target.position.z + 1),
                                5 => voxel_shared::BlockPos::new(target.position.x, target.position.y, target.position.z - 1),
                                _ => voxel_shared::BlockPos::new(0, 0, 0),
                            };
                            if place_pos.x != 0 || place_pos.y != 0 || place_pos.z != 0 {
                                if self.game.world.get_block(place_pos).id == 0 {
                                    tracing::info!("Placing block at {:?}", place_pos);
                                    self.game.world.set_block(place_pos, block_state);
                                    self.need_update_chunks = true;
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn device_event(&mut self, _event_loop: &ActiveEventLoop, _device_id: winit::event::DeviceId, event: winit::event::DeviceEvent) {
        if let winit::event::DeviceEvent::MouseMotion { delta } = event {
            self.mouse_delta.0 += delta.0 as f32;
            self.mouse_delta.1 += delta.1 as f32;
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        // Apply mouse look
        if self.mouse_delta.0 != 0.0 || self.mouse_delta.1 != 0.0 {
            self.camera.add_yaw(self.mouse_delta.0 * 0.002);
            self.camera.add_pitch(-self.mouse_delta.1 * 0.002);
            self.mouse_delta = (0.0, 0.0);
        }

        // Handle movement input
        let mut move_forward = 0.0f32;
        let mut move_strafe = 0.0f32;

        if self.keys.get(&winit::keyboard::KeyCode::KeyW).copied().unwrap_or(false) {
            move_forward += 1.0;
        }
        if self.keys.get(&winit::keyboard::KeyCode::KeyS).copied().unwrap_or(false) {
            move_forward -= 1.0;
        }
        if self.keys.get(&winit::keyboard::KeyCode::KeyA).copied().unwrap_or(false) {
            move_strafe -= 1.0;
        }
        if self.keys.get(&winit::keyboard::KeyCode::KeyD).copied().unwrap_or(false) {
            move_strafe += 1.0;
        }

        // Apply player movement relative to camera direction
        if move_forward != 0.0 || move_strafe != 0.0 {
            let forward = self.camera.forward();
            let right = self.camera.right();
            let move_dir = forward * move_forward + right * move_strafe;
            let move_dir = move_dir.normalize();
            self.game.player.move_relative(move_dir.z, move_dir.x);
        }

        // Jump
        if self.keys.get(&winit::keyboard::KeyCode::Space).copied().unwrap_or(false) {
            self.game.player.jump();
        }

        // Update game
        self.game.update(1.0 / 60.0);

        // Update camera position to follow player
        let pos = self.game.player.position();
        self.camera.set_position(pos + glam::Vec3::new(0.0, 1.6, 0.0));

        // Load new chunks if player moved
        let new_cx = pos.x as i32 / 16;
        let new_cz = pos.z as i32 / 16;
        if new_cx != self.last_cx || new_cz != self.last_cz {
            self.last_cx = new_cx;
            self.last_cz = new_cz;
            self.load_chunks();
        }

        // Show debug info every 2 seconds
        static mut LAST_PRINT: u64 = 0;
        unsafe {
            LAST_PRINT = LAST_PRINT.wrapping_add(1);
            if LAST_PRINT % 120 == 0 {
                let height = self.game.world.get_height_at(pos.x as i32, pos.z as i32);
                let target = if let Some(t) = self.game.player.get_targeted_block(&self.game.world, 5.0) {
                    format!("({},{},{})", t.position.x, t.position.y, t.position.z)
                } else {
                    "none".to_string()
                };
                println!("Pos: ({:.1}, {:.1}, {:.1}) Terrain: {} Chunks: {} Target: {}",
                    pos.x, pos.y, pos.z, height, self.game.world.loaded_chunks().len(), target);
            }
        }

        // Render if we have a renderer
        if let Some(ref mut renderer) = self.renderer {
            // Update camera uniforms
            renderer.update_camera(&self.camera);

            // Update chunk meshes if needed
            if self.need_update_chunks {
                renderer.update_chunks(&self.game.world);
                self.need_update_chunks = false;
            }

            // Render frame
            if let Err(e) = renderer.render() {
                tracing::error!("Render error: {}", e);
            }
        }
    }
}

fn run_singleplayer_windowed() {
    println!("Initializing VoxelCraft with window...");

    // Create event loop
    let event_loop = EventLoop::new().expect("Failed to create event loop");

    // Create app state
    let mut app = VoxelApp::new();

    // Run the event loop
    event_loop.run_app(&mut app).expect("Failed to run event loop");
}

// =====================================================================
// SERVER MODE
// =====================================================================

fn run_server() {
    println!();
    println!("========================================");
    println!("  VoxelCraft - Dedicated Server");
    println!("========================================");
    println!();

    let port = 25565;
    let mut server = network::GameServer::new(port);

    match server.start() {
        Ok(()) => {
            println!("  Server started on port {}", port);
            println!("  Waiting for connections...");
            println!();
            println!("========================================");
            println!();
        }
        Err(e) => {
            error!("Failed to start server: {}", e);
            eprintln!("Failed to start server: {}", e);
            std::process::exit(1);
        }
    }

    // Simple server loop
    let mut client_count = 0u32;
    loop {
        match server.accept() {
            Ok((client_id, _stream)) => {
                client_count += 1;
                println!("  Client {} connected (total: {})", client_id, client_count);
            }
            Err(_e) => {
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    }
}
