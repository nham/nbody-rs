#![feature(phase)]

extern crate serialize;

extern crate docopt;
#[phase(plugin)] extern crate docopt_macros;

use std::num::Float;
use docopt::Docopt;

docopt!(Args deriving Show, "
Usage: nbody [options] [pos <x> <y> <z>] [vel <vx> <vy> <vz>]

Options:
    --dt STEP     Set the time step
")


fn print_vals_pretty(x: f64, y: f64, z: f64, vx: f64, vy: f64, vz: f64) {
    println!("r: ({:10.6}, {:10.6}, {:10.6})  v: ({:10.6}, {:10.6}, {:10.6})",
             x, y, z, vx, vy, vz);
}

fn print_vals_bare(x: f64, y: f64, z: f64, vx: f64, vy: f64, vz: f64) {
    println!("{:10.6} {:10.6} {:10.6}  {:10.6} {:10.6} {:10.6}",
             x, y, z, vx, vy, vz);
}

fn main() {
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());

    // if all the vx, vy, vz arguments are there, set initial velocity to it
    // otherwise use default
    let (mut x, mut y, mut z) = if args.cmd_pos == true {
        let x: Option<f64> = from_str(args.arg_x.as_slice());
        let y: Option<f64> = from_str(args.arg_y.as_slice());
        let z: Option<f64> = from_str(args.arg_z.as_slice());
        (x.unwrap_or(0.), y.unwrap_or(0.), z.unwrap_or(0.))
    } else {
        (1., 0., 0.) // default
    };

    let (mut vx, mut vy, mut vz) = if args.cmd_vel == true {
        let vx: Option<f64> = from_str(args.arg_vx.as_slice());
        let vy: Option<f64> = from_str(args.arg_vy.as_slice());
        let vz: Option<f64> = from_str(args.arg_vz.as_slice());
        (vx.unwrap_or(0.), vy.unwrap_or(0.), vz.unwrap_or(0.))
    } else {
        (0., 0.6, 0.) // default
    };

    let dt: Option<f64> = from_str(args.flag_dt.as_slice());
    let dt: f64 = if dt.is_none() {
        0.01 //default
    } else {
        dt.unwrap()
    };


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
