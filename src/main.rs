use ::rand::thread_rng;
use macroquad::prelude::*;

use particles_life::*;

fn update_image(particles: &Particles, colors: &[Color; PARTICLES_TYPES_AMOUNT]) {
    let dx = screen_width() / 2.;
    let dy = screen_height() / 2.;
    clear_background(BLACK);
    for p in particles.list() {
        let mut color = colors[p.t as usize];
        color.a = 0.75;
        draw_poly(p.x as f32 + dx, p.y as f32 + dy, 8, 3., 0., color);
    }
}

#[macroquad::main("Particles life")]
async fn main() {
    let mut rng = thread_rng();

    let mut colors = [BLACK; PARTICLES_TYPES_AMOUNT];
    for i in 0..PARTICLES_TYPES_AMOUNT {
        colors[i] = Color::random(&mut rng);
    }

    let mut particles = Particles::new(&mut rng);
    let mut rules = Rules::new(&mut rng);

    let mut paused = false;

    loop {
        if !paused {
            particles.step(&mut rng, &rules);
        }

        egui_macroquad::ui(|egui_ctx| {
            egui::TopBottomPanel::top("top_panel").show(egui_ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui
                        .button(if paused { "Continue" } else { "Pause" })
                        .clicked()
                    {
                        paused = !paused;
                    }
                    if ui.button("New rules").clicked() {
                        particles = Particles::new(&mut rng);
                        rules = Rules::new(&mut rng);
                        for i in 0..PARTICLES_TYPES_AMOUNT {
                            colors[i] = Color::random(&mut rng);
                        }
                    }
                });
            });
        });

        update_image(&particles, &colors);
        egui_macroquad::draw();

        next_frame().await
    }
}
