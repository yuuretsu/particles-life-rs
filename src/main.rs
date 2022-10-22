use ::rand::{rngs::ThreadRng, thread_rng, Rng};
use macroquad::{color::hsl_to_rgb, prelude::*};

use particles_life::*;

fn update_image(particles: &Particles, colors: &[Color; PARTICLES_TYPES_AMOUNT], dx: f32, dy: f32) {
    let dx = screen_width() / 2. + dx;
    let dy = screen_height() / 2. + dy;
    clear_background(BLACK);
    for p in particles.into_iter() {
        let mut color = colors[p.rule as usize];
        color.a = 0.75;
        draw_poly(
            p.visual_x as f32 + dx,
            p.visual_y as f32 + dy,
            8,
            3.,
            0.,
            color,
        );
    }
}

fn generate_colors(rng: &mut ThreadRng) -> [Color; PARTICLES_TYPES_AMOUNT] {
    (0..PARTICLES_TYPES_AMOUNT)
        .map(|_| hsl_to_rgb(rng.gen(), 1., 0.5))
        .collect::<Vec<Color>>()
        .try_into()
        .unwrap()
}

#[macroquad::main("Particles life")]
async fn main() {
    let mut rng = thread_rng();

    let mut colors = generate_colors(&mut rng);

    let mut particles = Particles::new(&mut rng);
    let mut rules = Rules::new();
    rules.fill_random(&mut rng);

    let mut paused = false;

    let mut is_dragging = false;
    let mut dx = 0.0;
    let mut dy = 0.0;
    let mut offset_x = 0.0;
    let mut offset_y = 0.0;

    loop {
        let (mouse_x, mouse_y) = mouse_position();
        if !paused {
            particles.step(&mut rng, &rules);
        }

        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("a")
                .fixed_size((f32::MAX, f32::MAX))
                .fixed_pos((20.0, 20.0))
                .title_bar(false)
                .collapsible(false)
                .show(egui_ctx, |ui| {
                    let pause_btn = ui.button(if paused { "Continue" } else { "Pause" });
                    if pause_btn.clicked() {
                        paused = !paused;
                    }
                    if ui.button("Start new").clicked() {
                        particles = Particles::new(&mut rng);
                        rules.fill_random(&mut rng);
                        colors = generate_colors(&mut rng);
                    }
                });
        });

        if is_dragging {
            dx = mouse_x + offset_x;
            dy = mouse_y + offset_y;
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            is_dragging = true;
            offset_x = dx - mouse_x;
            offset_y = dy - mouse_y;
        }

        if is_mouse_button_released(MouseButton::Left) {
            is_dragging = false;
        }

        update_image(&particles, &colors, dx, dy);
        egui_macroquad::draw();

        next_frame().await
    }
}
