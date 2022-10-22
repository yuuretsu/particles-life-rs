mod draggable;
mod particles_life;

use ::rand::{rngs::ThreadRng, thread_rng, Rng};
use draggable::Draggable;
use egui::Vec2;
use macroquad::{color::hsl_to_rgb, prelude::*};
use particles_life::{ParticlesSystem, Rules, PARTICLES_TYPES_AMOUNT};

fn update_image(
    particles: &ParticlesSystem,
    offset: Vec2,
    colors: &[Color; PARTICLES_TYPES_AMOUNT],
) {
    let center = Vec2::new(screen_width(), screen_height()) / 2.;
    let offset = center + offset;
    clear_background(BLACK);
    for particle in particles.into_iter() {
        let (x, y) = (particle.visual_pos + offset).into();
        draw_poly(x, y, 8, 3., 0., colors[particle.rule as usize]);
    }
}

fn generate_colors(rng: &mut ThreadRng) -> [Color; PARTICLES_TYPES_AMOUNT] {
    (0..PARTICLES_TYPES_AMOUNT)
        .map(|_| {
            let mut color = hsl_to_rgb(rng.gen(), 1., 0.5);
            color.a = 0.75;
            color
        })
        .collect::<Vec<Color>>()
        .try_into()
        .unwrap()
}

#[macroquad::main("Particles life")]
async fn main() {
    let mut rng = thread_rng();

    let mut colors = generate_colors(&mut rng);

    let mut particles = ParticlesSystem::new(&mut rng);

    let mut rules = Rules::new();
    rules.fill_random(&mut rng);

    let mut paused = false;
    let mut offset = Draggable::new(Vec2::ZERO);

    loop {
        let mouse_pos: Vec2 = mouse_position().into();
        if !paused {
            particles.step(&mut rng, &rules);
        }

        egui_macroquad::ui(|ctx| {
            egui::Window::new("a")
                .fixed_size((f32::MAX, f32::MAX))
                .fixed_pos((10.0, 10.0))
                .title_bar(false)
                .collapsible(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        let pause_btn = ui.button(if paused { "Continue" } else { "Pause" });
                        if pause_btn.clicked() {
                            paused = !paused;
                        }
                        if ui.button("Start new").clicked() {
                            particles = ParticlesSystem::new(&mut rng);
                            rules.fill_random(&mut rng);
                            colors = generate_colors(&mut rng);
                            offset = Draggable::new(Vec2::ZERO);
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

        if is_key_down(KeyCode::Escape) {
            break;
        }

        update_image(&particles, offset.position, &colors);
        egui_macroquad::draw();

        next_frame().await;
    }
}
