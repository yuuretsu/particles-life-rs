use ::rand::thread_rng;
use macroquad::prelude::*;

use particles_life::*;

fn update_image(particles: &Particles, colors: &[Color; PARTICLES_TYPES]) {
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

    let mut colors = [BLACK; PARTICLES_TYPES];
    for i in 0..PARTICLES_TYPES {
        colors[i] = Color::random(&mut rng);
    }

    let mut particles = Particles::new(&mut rng);
    let mut rules = Rules::new(&mut rng);

    loop {
        particles.step(&mut rng, &rules);

        if is_mouse_button_pressed(MouseButton::Left) {
            particles = Particles::new(&mut rng);
            rules = Rules::new(&mut rng);
            for i in 0..PARTICLES_TYPES {
                colors[i] = Color::random(&mut rng);
            }
        }

        update_image(&particles, &colors);
        next_frame().await
    }
}
