use macroquad::prelude::*;
use ::rand::Rng;
use ::rand::rng; // No idea why I need both, but it didn't work without it

struct Particle {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    life: f32,
    color: Color,
}

fn spawn_confetti (particles: &mut Vec<Particle>, cx: f32, cy: f32) {
    let mut rng = rng();
    for _ in 0..100 {
        particles.push(Particle {
            x: cx,
            y: cy,
            dx: rng.random_range(-4.0..4.0),
            dy: rng.random_range(-4.0..4.0),
            life: rng.random_range(1.0..3.0),
            color: Color::new(
                    rng.random_range(0.0..1.0),
                    rng.random_range(0.0..1.0),
                    rng.random_range(0.0..1.0),
                    1.0,
                    ),
                });
            }
        }

#[macroquad::main("DVD Bouncer!")]
async fn main() {
    let logo = load_texture("dvd_logo.png").await.unwrap();

    let scale = 0.1;
    let logo_w = logo.width() * scale;
    let logo_h = logo.height() * scale;

    let screen_w = screen_width();
    let screen_h = screen_height();

    let mut x = 100.0;
    let mut y = 100.0;
    let mut dx = 3.5; // It's fast, but it takes too long otherwise
    let mut dy = 2.7;

    let mut particles: Vec<Particle> = Vec::new();

    // Test timer, comment out later
    //let mut test_timer = 0.0;
    //let test_interval = 5.0;


    loop {
        let dt = get_frame_time();
        //test_timer += dt;


        clear_background(BLACK);

        x += dx;
        y += dy;

        if x <= 0.0 || x + logo_w >= screen_w {
            dx = -dx;
        }

        if y <= 0.0 || y + logo_h >= screen_h {
            dy = -dy;
        }

        if (x <= 0.0 || x + logo_w >= screen_w)
        && (y <= 0.0 || y + logo_h >= screen_h) {
            spawn_confetti(&mut particles, x + logo_w, y + logo_h);   
        }

        // Comment out
        //if test_timer >= test_interval {
        //    spawn_confetti(&mut particles, x + logo_w, y + logo_h);
        //    test_timer = 0.0;
        //}

        for p in particles.iter_mut() {
            p.x += p.dx;
            p.y += p.dy;
            p.dy += 0.05;
            p.life -= dt;

            draw_rectangle(p.x, p.y, 4.0, 4.0, p.color);
        }

        particles.retain(|p| p.life > 0.0);

        draw_texture_ex(
            &logo,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(logo_w, logo_h)),
                ..Default::default()
            },
        );

        next_frame().await
    }
}
