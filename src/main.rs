use ::rand::{rngs::ThreadRng, thread_rng, Rng};
use draggable::Draggable;
use egui::Vec2;
use macroquad::{color::hsl_to_rgb, prelude::*};
mod draggable;

use particles_life::*;

fn update_image(particles: &Particles, colors: &[Color; PARTICLES_TYPES_AMOUNT], offset: Vec2) {
    let offset = Vec2::new(screen_width() / 2., screen_height() / 2.) + offset;
    clear_background(BLACK);
    for p in particles.into_iter() {
        let mut color = colors[p.rule as usize];
        color.a = 0.75;
        let (x, y) = (p.visual_pos + offset).into();
        draw_poly(x, y, 8, 3., 0., color);
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
    let mut offset = Draggable::new(Vec2::new(0., 0.));

    loop {
        let mouse_pos: Vec2 = mouse_position().into();
        if !paused {
            particles.step(&mut rng, &rules);
        }

        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("a")
                .fixed_size((f32::MAX, f32::MAX))
                .fixed_pos((10.0, 10.0))
                .title_bar(false)
                .collapsible(false)
                .show(egui_ctx, |ui| {
                    ui.horizontal(|ui| {
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
        });

        offset.update(mouse_pos);

        if is_mouse_button_pressed(MouseButton::Left) {
            offset.start_dragging(mouse_pos);
        }

        if is_mouse_button_released(MouseButton::Left) {
            offset.end_dragging();
        }

        update_image(&particles, &colors, offset.position);
        egui_macroquad::draw();

        next_frame().await
    }
}
