use raylib::prelude::*;

const WIDTH: i32 = 900;
const HEIGHT: i32 = 600;

const MASS_RADIUS: f32 = 20.0;
const LINE_THICKENESS: f32 = 4.0;

const L1: f32 = 250.0;
const L2: f32 = 200.0;

const G: f32 = 1000.0;
const M1: f32 = 1.0;
const M2: f32 = 1.0;


fn get_end_pos(l: f32, start: Vector2, phi: f32) -> Vector2 {
    Vector2::new(start.x + l * phi.sin(), start.y + l * phi.cos())
}

fn draw_pendulum(l: f32, start: Vector2, phi: f32, d: &mut RaylibDrawHandle){
    let end = get_end_pos(l, start, phi);
    d.draw_line_ex(start, end, LINE_THICKENESS, Color::WHITE);
    d.draw_circle_v(end, MASS_RADIUS, Color::RED);
}

fn draw_double_pendulum(start: Vector2, phi1: f32, phi2: f32, l1: f32, l2: f32, d: &mut RaylibDrawHandle){
    let end = get_end_pos(l1, start, phi1);
    draw_pendulum(l2, end, phi2, d);

    draw_pendulum(l1, start, phi1, d);
}

fn step(phi1: &mut f32, phi2: &mut f32, phi1_d: &mut f32, phi2_d: &mut f32, dt: f32){
    let num1 =
        -G * (2.0 * M1 + M2) * (*phi1).sin()
        - M2 * G * (*phi1 - 2.0 * *phi2).sin()
        - 2.0 * (*phi1 - *phi2).sin() * M2
            * (
                (*phi2_d).powi(2) * L2 +
                (*phi1_d).powi(2) * L1 * (*phi1 - *phi2).cos()
            );

    let den1 =
        L1 * (
            2.0 * M1 + M2 -
            M2 * (2.0 * *phi1 - 2.0 * *phi2).cos()
        );

        
    let num2 =
        2.0 * (*phi1 - *phi2).sin() * (
            (*phi1_d).powi(2) * L1 * (M1 + M2) +
            G * (M1 + M2) * (*phi1).cos() +
            (*phi2_d).powi(2) * L2 * M2 * (*phi1 - *phi2).cos()
        );
        
    let den2 =
        L2 * (
            2.0 * M1 + M2 -
            M2 * (2.0 * *phi1 - 2.0 * *phi2).cos()
        );
        
    let phi1_dd = num1 / den1;
    let phi2_dd = num2 / den2;

    *phi1_d += phi1_dd * dt;
    *phi2_d += phi2_dd * dt;

    *phi1 += *phi1_d * dt;
    *phi2 += *phi2_d * dt;
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("double pendulum simulation")
        .build();

    let start = Vector2::new((WIDTH/2) as f32, 0.0);

    // rl.set_target_fps(60);

    let mut phi1 = 80.0;
    let mut phi2 = -80.0;
    let mut phi1_d = 0.0;
    let mut phi2_d = 0.0;
    
    while !rl.window_should_close() {
        let dt = rl.get_frame_time(); 
        let fps = rl.get_fps();
        let mut d = rl.begin_drawing(&thread);

        step(&mut phi1, &mut phi2, &mut phi1_d, &mut phi2_d, dt);

        d.clear_background(Color::BLACK);


        d.draw_text("DOUBLE PENDULUM SIMULATION", 10, 10, 10, Color::WHITE);
        d.draw_text(&format!("FPS: {}", fps), 20, 20, 20, Color::WHITE);
        
        draw_double_pendulum(start, phi1, phi2, L1, L2, &mut d);
    }
}
