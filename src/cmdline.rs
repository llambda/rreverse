#![cfg_attr(feature="clippy", plugin(clippy))]
#![feature(plugin, proc_macro, custom_derive)]
#![plugin(rocket_codegen)]

extern crate kdtree;
extern crate time;
mod geocoder;
use geocoder::Locations;
use geocoder::ReverseGeocoder;
use geocoder::print_record;

use std::env;
use std::process::exit;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 3 {
        print!("Usage: rreverse lat long\n e.g., rreverse 44.962786 -93.344722\n\n");
        exit(1);
    }

    // println!("{:?}", args);

    let lat = args[1].parse::<f64>().unwrap();
    let long = args[2].parse::<f64>().unwrap();

    let loc = Locations::from_file();
    let geocoder = ReverseGeocoder::new(&loc);

    let y = geocoder.search(&[lat, long]).unwrap();

    print_record(y);
}
