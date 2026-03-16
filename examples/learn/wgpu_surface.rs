/// Example: WgpuSurface with Helio Sky Renderer
/// Demonstrates integration of helio-render-v2's volumetric sky system
use gpui::{
    App, Application, Context, Render, Window, WindowOptions, div, prelude::*, wgpu_surface, WgpuSurfaceHandle, rgb
};
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use gpui::Styled;
use gpui::AppContext;

use helio_render_v2::{
    Renderer, RendererConfig, Camera, GpuMesh, SceneLight, LightId,
    SkyAtmosphere, VolumetricClouds, Skylight,
};
use helio_render_v2::features::{
    FeatureRegistry, LightingFeature, BloomFeature, ShadowsFeature,
    BillboardsFeature, BillboardInstance,
};

struct HelioRenderState {
    renderer: Renderer,
    cube1: GpuMesh,
    cube2: GpuMesh,
    cube3: GpuMesh,
    ground: GpuMesh,
    roof: GpuMesh,
    sun_light_id: LightId,
    sun_angle: f32,
    cam_pos: glam::Vec3,
    cam_yaw: f32,
    cam_pitch: f32,
    frame_count: u32,
    width: u32,
    height: u32,
}

struct SurfaceExample {
    surface: WgpuSurfaceHandle,
    fps_rx: std::sync::mpsc::Receiver<f64>,
    display_fps: f64,
}

impl Render for SurfaceExample {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // pull any pending fps samples from channel
        while let Ok(f) = self.fps_rx.try_recv() {
            self.display_fps = f;
        }
        // ensure we keep repainting (needed since updates arrive off-thread)
        window.request_animation_frame();

        // The surface element will display the front buffer
        // Overlay a debug border and label for visibility
        div()
            .w(gpui::px(1720.0))
            .h(gpui::px(1080.0))
            .border_4()
            .border_color(rgb(0x00aaff))
            .rounded_lg()
            .shadow_xl()
            .bg(rgb(0x000000))
            .m(gpui::px(8.0))
            .child(
                wgpu_surface(self.surface.clone())
                    .absolute()
                    .inset_0() // Fill parent div
            )
            .child(
                div()
                    .absolute()
                    .top(gpui::px(4.0))
                    .left(gpui::px(8.0))
                    .text_color(rgb(0x00aaff))
                    .text_xl()
                    .child(format!("FPS: {:.1} | Helio Sky Renderer", self.display_fps))
            )
    }
}

fn load_sprite() -> (Vec<u8>, u32, u32) {
    // Create a simple white square sprite for billboards
    let size = 64u32;
    let mut pixels = Vec::with_capacity((size * size * 4) as usize);
    for _y in 0..size {
        for _x in 0..size {
            pixels.extend_from_slice(&[255u8, 255u8, 255u8, 255u8]); // white RGBA
        }
    }
    (pixels, size, size)
}

fn main() {
    env_logger::init();
    Application::new().run(|cx: &mut App| {
        // Open a window
        _ = cx.open_window(WindowOptions::default(), |window: &mut Window, cx: &mut App| {
            // Create a WgpuSurfaceHandle (1720x1080 RGBA8)
            let surface = window.create_wgpu_surface(1720, 1080, wgpu::TextureFormat::Rgba8UnormSrgb)
                .expect("WgpuSurface not supported on this platform");
            let surface_thread = surface.clone();
            let fps_data: Arc<Mutex<f64>> = Arc::new(Mutex::new(0.0));
            let (fps_tx, fps_rx) = std::sync::mpsc::channel::<f64>();

            // secondary render thread with Helio renderer
            let fps_shared = fps_data.clone();
            thread::spawn(move || {
                // Wait for surface to be ready
                loop {
                    if surface_thread.back_buffer_view().is_some() {
                        break;
                    }
                    thread::sleep(Duration::from_millis(10));
                }

                let device = surface_thread.device();
                let queue = surface_thread.queue();
                let (width, height) = surface_thread.size();
                let format = surface_thread.format();

                log::info!("Initializing Helio renderer {}x{} {:?}", width, height, format);

                // Load sprite for billboards
                let (sprite_rgba, sprite_w, sprite_h) = load_sprite();

                // Build feature registry
                let feature_registry = FeatureRegistry::builder()
                    .with_feature(LightingFeature::new())
                    .with_feature(BloomFeature::new().with_intensity(0.3).with_threshold(1.2))
                    .with_feature(ShadowsFeature::new().with_atlas_size(1024).with_max_lights(4))
                    .with_feature(BillboardsFeature::new().with_sprite(sprite_rgba, sprite_w, sprite_h).with_max_instances(100))
                    .build();

                // Create Helio renderer
                let mut renderer = Renderer::new(
                    Arc::new(device.clone()),
                    Arc::new(queue.clone()),
                    RendererConfig::new(width, height, format, feature_registry),
                )
                .expect("Failed to create Helio renderer");

                renderer.set_editor_mode(true);

                // Create scene objects
                let cube1 = renderer.create_mesh_cube([0.0, 0.5, 0.0], 0.5);
                let cube2 = renderer.create_mesh_cube([-2.0, 0.4, -1.0], 0.4);
                let cube3 = renderer.create_mesh_cube([2.0, 0.3, 0.5], 0.3);
                let ground = renderer.create_mesh_plane([0.0, 0.0, 0.0], 20.0);
                let roof = renderer.create_mesh_rect3d([0.0, 2.85, 0.0], [4.5, 0.15, 4.5]);

                renderer.add_object(&cube1, None, glam::Mat4::IDENTITY);
                renderer.add_object(&cube2, None, glam::Mat4::IDENTITY);
                renderer.add_object(&cube3, None, glam::Mat4::IDENTITY);
                renderer.add_object(&ground, None, glam::Mat4::IDENTITY);
                renderer.add_object(&roof, None, glam::Mat4::IDENTITY);

                // Set up initial sun direction (start at a nice afternoon angle)
                let init_sun_angle = 1.0f32;
                let init_sun_dir = glam::Vec3::new(init_sun_angle.cos() * 0.3, init_sun_angle.sin(), 0.5).normalize();
                let init_light_dir = [-init_sun_dir.x, -init_sun_dir.y, -init_sun_dir.z];
                let init_elev = init_sun_dir.y.clamp(-1.0, 1.0);
                let init_lux = (init_elev * 3.0).clamp(0.0, 1.0);
                let sun_light_id = renderer.add_light(SceneLight::directional(
                    init_light_dir,
                    [1.0, 0.85, 0.7],
                    (init_lux * 0.35).max(0.01)
                ));

                // Add colored point lights
                renderer.add_light(SceneLight::point([0.0, 2.5, 0.0], [1.0, 0.85, 0.6], 4.0, 8.0));
                renderer.add_light(SceneLight::point([-2.5, 2.0, -1.5], [0.4, 0.6, 1.0], 3.5, 7.0));
                renderer.add_light(SceneLight::point([2.5, 1.8, 1.5], [1.0, 0.3, 0.3], 3.0, 6.0));

                // Set up volumetric sky with clouds
                renderer.set_sky_atmosphere(Some(
                    SkyAtmosphere::new()
                        .with_sun_intensity(22.0)
                        .with_exposure(4.0)
                        .with_mie_g(0.76)
                        .with_clouds(
                            VolumetricClouds::new()
                                .with_coverage(0.30)
                                .with_density(0.7)
                                .with_layer(800.0, 1800.0)
                                .with_wind([1.0, 0.0], 0.08),
                        ),
                ));

                renderer.set_skylight(Some(Skylight::new().with_intensity(0.08).with_tint([1.0, 1.0, 1.0])));

                // Add billboards for the lights
                renderer.add_billboard(BillboardInstance::new([0.0, 2.5, 0.0], [0.35, 0.35]).with_color([1.0, 0.85, 0.6, 1.0]));
                renderer.add_billboard(BillboardInstance::new([-2.5, 2.0, -1.5], [0.35, 0.35]).with_color([0.4, 0.6, 1.0, 1.0]));
                renderer.add_billboard(BillboardInstance::new([2.5, 1.8, 1.5], [0.35, 0.35]).with_color([1.0, 0.3, 0.3, 1.0]));

                let mut state = HelioRenderState {
                    renderer,
                    cube1,
                    cube2,
                    cube3,
                    ground,
                    roof,
                    sun_light_id,
                    sun_angle: init_sun_angle,
                    cam_pos: glam::Vec3::new(0.0, 2.5, 7.0),
                    cam_yaw: 0.0,
                    cam_pitch: -0.2,
                    frame_count: 0,
                    width,
                    height,
                };

                log::info!("Helio renderer initialized, starting render loop");

                // Render loop
                let mut last_report = std::time::Instant::now();
                let mut frame_count: u32 = 0;
                let mut last_frame_time = std::time::Instant::now();

                loop {
                    // Throttle: wait until compositor consumes last frame
                    surface_thread.wait_for_present();

                    // Get back buffer view and size
                    let (view, (dw, dh)) = match surface_thread.back_view_with_size() {
                        Some(tuple) => tuple,
                        None => {
                            thread::sleep(Duration::from_millis(1));
                            continue;
                        }
                    };

                    // Calculate delta time
                    let now = std::time::Instant::now();
                    let dt = (now - last_frame_time).as_secs_f32();
                    last_frame_time = now;

                    // Resize renderer if needed
                    if state.width != dw || state.height != dh {
                        log::info!("Resizing renderer to {}x{}", dw, dh);
                        state.renderer.resize(dw, dh);
                        state.width = dw;
                        state.height = dh;
                    }

                    // Update sun angle (slow rotation for demo)
                    state.sun_angle += 0.1 * dt;

                    // Calculate camera
                    let (sy, cy) = state.cam_yaw.sin_cos();
                    let (sp, cp) = state.cam_pitch.sin_cos();
                    let forward = glam::Vec3::new(sy * cp, sp, -cy * cp);
                    let aspect = dw as f32 / dh.max(1) as f32;
                    let time = state.frame_count as f32 * 0.016;

                    let camera = Camera::perspective(
                        state.cam_pos,
                        state.cam_pos + forward,
                        glam::Vec3::Y,
                        std::f32::consts::FRAC_PI_4,
                        aspect,
                        0.1,
                        1000.0,
                        time,
                    );

                    // Update sun direction
                    let sun_dir = glam::Vec3::new(
                        state.sun_angle.cos() * 0.3,
                        state.sun_angle.sin(),
                        0.5,
                    ).normalize();
                    let light_dir = [-sun_dir.x, -sun_dir.y, -sun_dir.z];

                    // Sun intensity dims at horizon/night
                    let sun_elev = sun_dir.y.clamp(-1.0, 1.0);
                    let sun_lux = (sun_elev * 3.0).clamp(0.0, 1.0);
                    let sun_color = [
                        1.0_f32.min(1.0 + (1.0 - sun_elev) * 0.3),
                        (0.85 + sun_elev * 0.15).clamp(0.0, 1.0),
                        (0.7 + sun_elev * 0.3).clamp(0.0, 1.0),
                    ];

                    // Update sun light
                    state.renderer.update_light(
                        state.sun_light_id,
                        SceneLight::directional(light_dir, sun_color, (sun_lux * 0.35).max(0.01))
                    );

                    // Render to back buffer
                    if let Err(e) = state.renderer.render(&camera, &view, dt) {
                        log::error!("Helio render error: {:?}", e);
                    }

                    // Present
                    surface_thread.present();
                    state.frame_count = state.frame_count.wrapping_add(1);

                    // Update FPS counter
                    frame_count = frame_count.wrapping_add(1);
                    if now.duration_since(last_report) >= Duration::from_secs(1) {
                        let fps = frame_count as f64;
                        *fps_shared.lock().unwrap() = fps;
                        frame_count = 0;
                        last_report = now;
                    }
                }
            });

            // Construct entity and keep handle in outer scope
            let handle = cx.new(|_cx| SurfaceExample { surface, fps_rx, display_fps: 0.0 });

            // Timer thread: wake once per second and push fps into channel
            let fps_shared = fps_data.clone();
            let tx_clone = fps_tx.clone();
            thread::spawn(move || {
                loop {
                    std::thread::sleep(Duration::from_secs(1));
                    let val = *fps_shared.lock().unwrap();
                    let _ = tx_clone.send(val);
                }
            });

            handle
        });
    });
}
