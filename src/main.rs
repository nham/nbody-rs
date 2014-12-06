use std::num::Float;

fn print_vals_pretty(x: f64, y: f64, z: f64, vx: f64, vy: f64, vz: f64) {
    println!("r: ({:10.6}, {:10.6}, {:10.6})  v: ({:10.6}, {:10.6}, {:10.6})",
             x, y, z, vx, vy, vz);
}

fn print_vals_bare(x: f64, y: f64, z: f64, vx: f64, vy: f64, vz: f64) {
    println!("{:10.6} {:10.6} {:10.6}  {:10.6} {:10.6} {:10.6}",
             x, y, z, vx, vy, vz);
}

fn main() {
    let (mut x, mut y, mut z): (f64, f64, f64) = (1., 0., 0.);
    let (mut vx, mut vy, mut vz): (f64, f64, f64) = (0., 0.5, 0.);
    let dt: f64 = 0.01;

    print_vals_bare(x,y,z,vx,vy,vz);

    for _ in range(0u, 1000) {
        // squared magnitude of the position vector |r|^2
        let r2: f64 = x*x + y*y + z*z;
        // |r|^2 * sqrt(|r|^2) = |r|^3
        let r3 = r2 * r2.sqrt();

        let (ax, ay, az) = (-x / r3, -y / r3, -z / r3);

        x += vx*dt;
        y += vx*dt;
        z += vx*dt;

        vx += ax*dt;
        vy += ay*dt;
        vz += az*dt;

        print_vals_bare(x,y,z,vx,vy,vz);
    }
}
