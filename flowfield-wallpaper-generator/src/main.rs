// A demonstration of drawing to a very large texture, capturing the texture in its original size
// as a PNG and displaying a down-scaled version of the image within the window each frame.

use nannou::{
    noise::{NoiseFn, Perlin, Seedable},
    prelude::*,
};

fn main() {
    nannou::app(model).update(update).exit(exit).run();
}

const TEXTURE_SIZE: [u32; 2] = [3_840, 2_160];
const PARTICLE_AMOUNT: usize = 10000;
const NOISE_SCALE: f64 = 0.01;
const PARTICLE_COLOR: [f32; 4] = [0.0, 0.0, 1.0, 0.1];

struct Particle {
    pos: Vec2,
    vel: Vec2,
    acc: Vec2,
}

impl Particle {
    fn _default() -> Self {
        let pos = vec2(0.0, 0.0);
        let vel = vec2(0.0, 0.0);
        let acc = vec2(0.0, 0.0);
        Particle { pos, vel, acc }
    }

    fn new(pos: Vec2) -> Self {
        let vel = vec2(0.0, 0.0);
        let acc = vec2(0.0, 0.0);
        Particle { pos, vel, acc }
    }

    fn update(&mut self) {
        self.vel += self.acc;
        self.vel = self.vel.clamp_length_max(3.0);
        self.pos += self.vel;
        self.acc *= 0.0;

        if self.pos.x < 0.0 {
            self.pos.x = TEXTURE_SIZE[0] as f32 - 1.0;
        } else if self.pos.x > TEXTURE_SIZE[0] as f32 {
            self.pos.x = 1.0;
        }

        if self.pos.y < 0.0 {
            self.pos.y = TEXTURE_SIZE[1] as f32 - 1.0;
        } else if self.pos.y > TEXTURE_SIZE[1] as f32 {
            self.pos.y = 1.0;
        }
    }

    fn apply_force(&mut self, force: Vec2) {
        self.acc += force;
        self.acc = self.acc.clamp_length_max(2.0);
    }
}

struct Model {
    // The texture that we will draw to.
    texture: wgpu::Texture,
    // Create a `Draw` instance for drawing to our texture.
    draw: nannou::Draw,
    // The type used to render the `Draw` vertices to our texture.
    renderer: nannou::draw::Renderer,
    // The type used to capture the texture.
    texture_capturer: wgpu::TextureCapturer,
    // The type used to resize our texture to the window texture.
    texture_reshaper: wgpu::TextureReshaper,

    ps: Vec<Particle>,
    perlin: Perlin,
    z: f64,
}

impl Model {
    fn noise(&self, x: f64, y: f64) -> f64 {
        self.perlin.get([x, y, self.z])
    }
}

fn model(app: &App) -> Model {
    // Lets write to a 4K UHD texture.

    // Create the window.
    let [win_w, win_h] = [TEXTURE_SIZE[0] / 4, TEXTURE_SIZE[1] / 4];
    let w_id = app
        .new_window()
        .size(win_w, win_h)
        .title("flowfield wallpaper")
        .view(view)
        .build()
        .unwrap();
    let window = app.window(w_id).unwrap();

    // Retrieve the wgpu device.
    let device = window.device();

    // Create our custom texture.
    let sample_count = window.msaa_samples();
    let texture = wgpu::TextureBuilder::new()
        .size(TEXTURE_SIZE)
        // Our texture will be used as the RENDER_ATTACHMENT for our `Draw` render pass.
        // It will also be SAMPLED by the `TextureCapturer` and `TextureResizer`.
        .usage(wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING)
        // Use nannou's default multisampling sample count.
        .sample_count(sample_count)
        // Use a spacious 16-bit linear sRGBA format suitable for high quality drawing.
        .format(wgpu::TextureFormat::Rgba16Float)
        // Build it!
        .build(device);

    // Create our `Draw` instance and a renderer for it.
    let draw = nannou::Draw::new();
    let descriptor = texture.descriptor();
    let renderer =
        nannou::draw::RendererBuilder::new().build_from_texture_descriptor(device, descriptor);

    // Create the texture capturer.
    let texture_capturer = wgpu::TextureCapturer::default();

    // Create the texture reshaper.
    let texture_view = texture.view().build();
    let texture_sample_type = texture.sample_type();
    let dst_format = Frame::TEXTURE_FORMAT;
    let texture_reshaper = wgpu::TextureReshaper::new(
        device,
        &texture_view,
        sample_count,
        texture_sample_type,
        sample_count,
        dst_format,
    );

    // Make sure the directory where we will save images to exists.
    std::fs::create_dir_all(&capture_directory(app)).unwrap();

    let ps = (0..PARTICLE_AMOUNT)
        .map(|_| {
            Particle::new(vec2(
                random_f32() * TEXTURE_SIZE[0] as f32,
                random_f32() * TEXTURE_SIZE[1] as f32,
            ))
        })
        .collect();

    let perlin = Perlin::new().set_seed((app.time * 1000.0) as u32);

    Model {
        texture,
        draw,
        renderer,
        texture_capturer,
        texture_reshaper,
        ps,
        z: 0.0,
        perlin,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    // First, reset the `draw` state.
    let draw = &model
        .draw
        .x_y(
            -(model.texture.size()[0] as f32) / 2.0,
            (model.texture.size()[1] as f32) / 2.0,
        )
        .scale_y(-1.0);
    draw.reset();

    // Create a `Rect` for our texture to help with drawing.
    // let [w, h] = model.texture.size();
    // let r = geom::Rect::from_w_h(w as f32, h as f32);

    // Use the frame number to animate, ensuring we get a constant update time.
    let elapsed_frames = app.main_window().elapsed_frames();
    // let t = elapsed_frames as f32 / 60.0;

    // Draw like we normally would in the `view`.
    // draw.rect()
    //     .x_y(TEXTURE_SIZE[0] as f32 * 0.5, TEXTURE_SIZE[1] as f32 * 0.5)
    //     .w_h(TEXTURE_SIZE[0] as f32, TEXTURE_SIZE[1] as f32)
    //     .color(rgba(89.0 / 255.0, 87.0 / 255.0, 88.0 / 255.0, 0.1));

    let noises: Vec<f64> = model
        .ps
        .iter()
        .map(|p| {
            let x = p.pos.x as f64 * NOISE_SCALE;
            let y = p.pos.y as f64 * NOISE_SCALE;
            model.noise(x, y)
        })
        .collect();

    model.ps.iter_mut().zip(noises).for_each(|(p, n)| {
        let angle = map_range(n, -1.0, 1.0, 0.0, TAU);
        let mut vec_angle = vec2(angle.cos(), angle.sin());
        vec_angle *= 1.0;

        p.apply_force(vec_angle);
        p.update();
        draw.ellipse()
            .x_y(p.pos.x, p.pos.y)
            .radius(2.0)
            // cyan
            .color(rgba(
                PARTICLE_COLOR[0],
                PARTICLE_COLOR[1],
                PARTICLE_COLOR[2],
                PARTICLE_COLOR[3],
            ));
    });
    model.z += 0.01;

    // Draw frame number and size in bottom left.
    // let string = format!("Frame {} - {:?}", elapsed_frames, [w, h]);
    // let text = text(&string)
    //     .font_size(48)
    //     .left_justify()
    //     .align_bottom()
    //     .build(r.pad(r.h() * 0.05));
    // draw.path().fill().color(WHITE).events(text.path_events());
    // Render our drawing to the texture.
    let window = app.main_window();
    let device = window.device();
    let ce_desc = wgpu::CommandEncoderDescriptor {
        label: Some("texture renderer"),
    };
    let mut encoder = device.create_command_encoder(&ce_desc);
    model
        .renderer
        .render_to_texture(device, &mut encoder, draw, &model.texture);
    // Take a snapshot of the texture. The capturer will do the following:
    //
    // 1. Resolve the texture to a non-multisampled texture if necessary.
    // 2. Convert the format to non-linear 8-bit sRGBA ready for image storage.
    // 3. Copy the result to a buffer ready to be mapped for reading.
    let snapshot = model
        .texture_capturer
        .capture(device, &mut encoder, &model.texture);

    // Submit the commands for our drawing and texture capture to the GPU.
    window.queue().submit(Some(encoder.finish()));

    if elapsed_frames % 50 == 0 {
        // Submit a function for writing our snapshot to a PNG.
        //
        // NOTE: It is essential that the commands for capturing the snapshot are `submit`ted before we
        // attempt to read the snapshot - otherwise we will read a blank texture!
        let path = capture_directory(app)
            .join(
                elapsed_frames.to_string()
                    + &PARTICLE_COLOR
                        .iter()
                        .map(|c| format!("{:.2}", c))
                        .collect::<String>(),
            )
            .with_extension("png");

        snapshot
            .read(move |result| {
                let image = result.expect("failed to map texture memory").to_owned();
                image
                    .save(&path)
                    .expect("failed to save texture to png image");
            })
            .unwrap();
    }
}

// Draw the state of your `Model` into the given `Frame` here.
fn view(_app: &App, model: &Model, frame: Frame) {
    // Sample the texture and write it to the frame.
    let mut encoder = frame.command_encoder();
    model
        .texture_reshaper
        .encode_render_pass(frame.texture_view(), &mut *encoder);
}

// Wait for capture to finish.
fn exit(app: &App, model: Model) {
    println!("Waiting for PNG writing to complete...");
    let window = app.main_window();
    let device = window.device();
    model
        .texture_capturer
        .await_active_snapshots(&device)
        .unwrap();
    println!("Done!");
}

// The directory where we'll save the frames.
fn capture_directory(app: &App) -> std::path::PathBuf {
    app.project_path()
        .expect("could not locate project_path")
        .join(app.exe_name().unwrap())
}
