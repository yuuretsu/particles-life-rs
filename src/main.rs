use ::rand::thread_rng;
use macroquad::prelude::*;

use particles_life::*;

fn update_image(particles: &Particles, colors: &[Color; PARTICLES_TYPES_AMOUNT], dx: f32, dy: f32) {
    let dx = screen_width() / 2. + dx;
    let dy = screen_height() / 2. + dy;
    clear_background(BLACK);
    for p in particles.list() {
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
