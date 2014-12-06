extern crate serialize;
extern crate docopt;

use std::num::Float;
use docopt::Docopt;

// Write the Docopt usage string.
static USAGE: &'static str = "
Usage: nbody [--dt STEP]
       nbody [--dt STEP] <vx> <vy> <vz>

Options:
    --dt STEP  Set the time step
";

#[deriving(Decodable, Show)]
struct Args {
    arg_vx: String,
    arg_vy: String,
    arg_vz: String,
    flag_dt: String,
}


fn print_vals_pretty(x: f64, y: f64, z: f64, vx: f64, vy: f64, vz: f64) {
    println!("r: ({:10.6}, {:10.6}, {:10.6})  v: ({:10.6}, {:10.6}, {:10.6})",
             x, y, z, vx, vy, vz);
}

fn print_vals_bare(x: f64, y: f64, z: f64, vx: f64, vy: f64, vz: f64) {
    println!("{:10.6} {:10.6} {:10.6}  {:10.6} {:10.6} {:10.6}",
             x, y, z, vx, vy, vz);
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    // if all the vx, vy, vz arguments are there, set initial velocity to it
    // otherwise use default
    let vx: Option<f64> = from_str(args.arg_vx.as_slice());
    let vy: Option<f64> = from_str(args.arg_vy.as_slice());
    let vz: Option<f64> = from_str(args.arg_vz.as_slice());
    let dt: Option<f64> = from_str(args.flag_dt.as_slice());

    let (mut vx, mut vy, mut vz) = if vx.is_none() || vy.is_none() || vz.is_none() {
        (0., 0.6, 0.) // default
    } else {
        (vx.unwrap(), vy.unwrap(), vz.unwrap())
    };

    let dt: f64 = if dt.is_none() {
        0.01
    } else {
        dt.unwrap()
    };

    let (mut x, mut y, mut z): (f64, f64, f64) = (1., 0., 0.);

    println!("dt = {}", dt);
    print_vals_bare(x,y,z,vx,vy,vz);

    for _ in range(0u, 1000) {
        // squared magnitude of the position vector |r|^2
        let r2: f64 = x*x + y*y + z*z;
        // |r|^2 * sqrt(|r|^2) = |r|^3
        let r3 = r2 * r2.sqrt();

        let (ax, ay, az) = (-x / r3, -y / r3, -z / r3);

        x += vx*dt;
        y += vy*dt;
        z += vz*dt;

        vx += ax*dt;
        vy += ay*dt;
        vz += az*dt;

        print_vals_bare(x,y,z,vx,vy,vz);
    }
}
