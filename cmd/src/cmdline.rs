use reverse_geocoder::{Locations, ReverseGeocoder, SearchResult};
use std::{env, process::exit};
use std::error::Error;
use time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();

    if args.len() < 3 {
        eprint!("Usage: rreverse lat long\n e.g., rreverse 44.962786 -93.344722\n\n");
        exit(1);
    }

    let lat = args[1].parse::<f64>()?;
    let long = args[2].parse::<f64>()?;

    let loc = Locations::from_memory();
    let geocoder = ReverseGeocoder::new(&loc);

    let start = Instant::now();
    let search_result: SearchResult = geocoder.search((lat, long)).expect("Nothing found.");
    eprintln!("{} ms to search", start.elapsed().whole_milliseconds());

    println!("Location: {}", search_result.record);
    println!("Distance: {}", search_result.distance);

    Ok(())
}
