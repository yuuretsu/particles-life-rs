mod draggable;
mod particle_system;

use ::rand::{rngs::ThreadRng, thread_rng, Rng};
use draggable::Draggable;
use egui::Vec2;
use lerp::Lerp;
use macroquad::{color::hsl_to_rgb, prelude::*};
use particle_system::{ParticleSystem, Rules, PARTICLES_TYPES_AMOUNT};

fn lerp_color(a: &Color, b: &Color, t: f32) -> Color {
    Color {
        r: a.r.lerp(b.r, t),
        g: a.g.lerp(b.g, t),
        b: a.b.lerp(b.b, t),
        a: a.a.lerp(b.a, t),
    }
}

fn update_image(
    particles: &ParticleSystem,
    offset: Vec2,
    colors: &[[f32; 3]; PARTICLES_TYPES_AMOUNT],
) {
    let center = Vec2::new(screen_width(), screen_height()) / 2.;
    let offset = center + offset;
    clear_background(BLACK);
    for particle in particles {
        let (x, y) = (particle.visual_pos + offset).into();
        let speed = (particle.real_pos - particle.visual_pos).length();
        let raduis = 3. + speed * 0.2;
        let [r, g, b] = colors[particle.rule as usize];
        let color = lerp_color(
            &Color::new(r, g, b, 0.75),
            &Color::new(1., 1., 1., 0.),
            speed * 0.02,
        );
        if x > 0. - raduis
            && x < screen_width() + raduis
            && y > 0. - raduis
            && y < screen_height() + raduis
        {
            draw_poly(x, y, 8, raduis, 0., color);
        }
    }
}

fn generate_colors(rng: &mut ThreadRng) -> [[f32; 3]; PARTICLES_TYPES_AMOUNT] {
    [[0., 0., 0.]; PARTICLES_TYPES_AMOUNT].map(|_| {
        let mut color = hsl_to_rgb(rng.gen(), 1., 0.5);
        color.a = 0.75;
        [color.r, color.g, color.b]
    })
}

#[macroquad::main("Particles life")]
async fn main() {
    let mut rng = thread_rng();

    let mut colors = generate_colors(&mut rng);

    let mut particles = ParticleSystem::new(&mut rng);

    let mut rules = Rules::new();
    rules.fill_random(&mut rng);

    let mut paused = false;
    let mut show_rules = true;
    let mut offset = Draggable::new(Vec2::ZERO);

    loop {
        let mouse_pos: Vec2 = mouse_position().into();
        if !paused {
            particles.step(&mut rng, &rules);
        }

        egui_macroquad::ui(|ctx| {
            if show_rules {
                egui::Window::new("Rules editor")
                    .collapsible(false)
                    .fixed_size((0., 0.))
                    .default_pos((10., 50.))
                    .show(ctx, |ui| {
                        egui::Grid::new("Rules").show(ui, |ui| {
                            ui.label("".to_string());
                            for y in 0..PARTICLES_TYPES_AMOUNT {
                                egui::widgets::color_picker::color_edit_button_rgb(
                                    ui,
                                    &mut colors[y],
                                );
                            }
                            ui.end_row();
                            for y in 0..PARTICLES_TYPES_AMOUNT {
                                egui::widgets::color_picker::color_edit_button_rgb(
                                    ui,
                                    &mut colors[y],
                                );
                                for x in 0..PARTICLES_TYPES_AMOUNT {
                                    ui.add(egui::DragValue::new(rules.get_mut(y, x)));
                                }
                                ui.end_row();
                            }
                        });
                    });
            }
            egui::Window::new("Options")
                .fixed_size((100., f32::MAX))
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
                            particles = ParticleSystem::new(&mut rng);
                            rules.fill_random(&mut rng);
                            colors = generate_colors(&mut rng);
                            offset = Draggable::new(Vec2::ZERO);
                        }
                        ui.checkbox(&mut show_rules, "Show rules");
                    });
                });
        });

        offset.update(mouse_pos);

        if is_mouse_button_pressed(MouseButton::Right) {
            offset.start_dragging(mouse_pos);
        }

        if is_mouse_button_released(MouseButton::Right) {
            offset.end_dragging();
        }

        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }

        if is_key_down(KeyCode::Escape) {
            break;
        }

        update_image(&particles, offset.position, &colors);
        egui_macroquad::draw();

        next_frame().await;
    }
}
