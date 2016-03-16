#![feature(test)]
#![feature(phase)]
#![feature(box_syntax)]
extern crate kdtree;
extern crate csv;
extern crate rustc_serialize;
extern crate time;
extern crate params;

use kdtree::KdTree;
use time::PreciseTime;

extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use rustc_serialize::json;
use router::Router;

#[derive(Clone, RustcDecodable, RustcEncodable)]
struct Record {
    lat: f64,
    lon: f64,
    name: String,
    admin1: String,
    admin2: String,
    admin3: String,
}

pub struct Locations {
    records: Vec<([f64; 2], Record)>,
}

impl Locations {
    fn from_file() -> Locations {
        let start = PreciseTime::now();
        let mut records = Vec::new();

        let mut rdr = csv::Reader::from_file("cities.csv").unwrap();

        for record in rdr.decode() {
            let r: Record = record.unwrap();
            records.push(([r.lat, r.lon], r));
        }

        let end = PreciseTime::now();

        println!("{} seconds to load cities.csv", start.to(end));

        Locations {
            records: records,
        }
    }
}

pub struct ReverseGeocoder<'a> {
    tree: KdTree<'a, &'a Record>,
}

impl<'a> ReverseGeocoder<'a> {
    fn new(loc: &'a Locations) -> ReverseGeocoder<'a> {
        let mut r = ReverseGeocoder::<'a> {
            tree: KdTree::new(2),
        };
        r.initialize(loc);
        r
    }

    fn initialize(&mut self, loc: &'a Locations) {
        let start = PreciseTime::now();
        for record in &loc.records {
            self.tree.add(&record.0, &record.1).unwrap();
        }
        let end = PreciseTime::now();
        println!("{} seconds to build the KdTree", start.to(end));
    }

    fn search(&self, loc: &[f64; 2]) -> Option<Record> {
        use kdtree::distance::squared_euclidean;

        let y = self.tree.nearest(loc, 1, &squared_euclidean).unwrap();

        if y.len() > 0 {
            return Some((*y[0].1).clone());
        } else {
            return None;
        }
    }

}

fn print_record(r: &Record) {
    println!("({}, {}): {} / {} / {} / {}", r.lat, r.lon, r.name, r.admin1, r.admin2, r.admin3);
}

fn main() {
    let loc = Locations::from_file();
    let geocoder = ReverseGeocoder::new(&loc);

    let test = geocoder.search(&[44.962786, -93.344722]).unwrap();

    print_record(&test);
    let mut router = Router::new();

    router.get("/reverse", hello_world);

    fn hello_world(_: &mut Request) -> IronResult<Response> {
        use params::{Map, Params};

        let map: Map = try!(req.get_ref::<Params>());

        let loc = Locations::from_file();
        let geocoder = ReverseGeocoder::new(&loc);
        let y = geocoder.search(&[44.962786, -93.344722]).unwrap();
        let payload = json::encode(&y).unwrap();
        Ok(Response::with((status::Ok, payload)))
    }

    Iron::new(router).http("localhost:3000").unwrap();
    println!("On 3000");
}

extern crate test;

mod tests {

    #[test]
    fn it_works() {
        let loc = super::Locations::from_file();
        let geocoder = super::ReverseGeocoder::new(&loc);
        let y = geocoder.search(&[44.962786, -93.344722]);
        assert_eq!(y.is_some(), true);
        let slp = y.unwrap();

        assert_eq!(slp.name, "Saint Louis Park");

        // [44.894519, -93.308702] is 60 St W @ Penn Ave S, Minneapolis, Minnesota; however, this is physically closer to Richfield
        let mpls = geocoder.search(&[44.894519, -93.308702]).unwrap();
        assert_eq!(mpls.name, "Richfield");

        // [44.887055, -93.334204] is HWY 62 and Valley View Road, whish is in Edina
        let edina = geocoder.search(&[44.887055, -93.334204]).unwrap();
        assert_eq!(edina.name, "Edina");
    }

    // #[bench]
    // fn bench_lookup(b: &mut Bencher) {
    //
    // }
}


/*
fn geodetic_in_ecef(geo_coords: (f32, f32)) -> (f32, f32, f32) {
    let a = 6378.137; // major axis in kms
    let e2 = 0.00669437999014;

    let lat = geo_coords.0;
    let lon = geo_coords.1;

    let lat_r = lat.to_radians();
    let lon_r = lon.to_radians();
    let normal = a / (1f32 - e2 * lat_r.sin().powi(2));

    let x = normal * lat_r.cos() * lon_r.cos();
    let y = normal * lat_r.cos() * lon_r.sin();
    let z = normal * (1f32 - e2) * lat.sin();

    (x, y, z)
}
*/
