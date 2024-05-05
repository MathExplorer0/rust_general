use std::f64::consts::PI;

fn main() {
    let g: f64 = 9.81;
    let l: f64 = 1.0;
    let theta0 = PI / 6.0;
    let omega = (g / l).sqrt();

    let mut theta = theta0;
    let dt = 0.01;
    let mut t = 0.0;

    while t <= 10.0 {
        let x = l * theta.sin();
        let y = l * theta.cos();

        println!("Time: {:.2} s, x: {:.4} m, y: {:.4} m", t, x, y);

        let dtheta_dt = omega;
        theta += dtheta_dt * dt;

        t += dt;
    }
}
