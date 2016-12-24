#![cfg_attr(feature="clippy", plugin(clippy))]
#![feature(plugin, proc_macro, custom_derive)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate lazy_static;

extern crate rocket;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

extern crate rustc_serialize;

extern crate kdtree;
extern crate time;

use time::PreciseTime;
use rocket::response::content::JSON;

mod geocoder;
use geocoder::Locations;
use geocoder::ReverseGeocoder;

#[derive(FromForm)]
struct LatLongQuery {
    lat: f64,
    lng: f64
}

lazy_static! {
    static ref LOCATIONS: Locations = Locations::from_file();
    static ref GEOCODER: ReverseGeocoder<'static> = ReverseGeocoder::new(&LOCATIONS);
}

#[get("/api/lookup?<latLong>")]
fn lookup(latLong: LatLongQuery) -> JSON<String> {
    let start = PreciseTime::now();
    let y = GEOCODER.search(&[latLong.lat, latLong.lng]).unwrap();
    let end = PreciseTime::now();
    println!("{} ms to search", start.to(end).num_milliseconds());

    // Ok(Response::with((status::Ok, json::encode(y).unwrap())))
    JSON(serde_json::to_string(&y).unwrap())
}


// fn geocoder_middleware(request: &mut Request) -> IronResult<Response> {
//     match request.url.query().clone() {
//         Some(query) => {
//             // println!("{:?}", query);
//             let data = parse(&query).unwrap();
//             // println!("{:?}", data);
//             // println!("{:?}", data.is_object());

//             let obj = data.as_object().unwrap();
//             let lat = obj.get("lat").unwrap().as_str().unwrap().parse::<f64>().unwrap();
//             let long = obj.get("long").unwrap().as_str().unwrap().parse::<f64>().unwrap();

//             let start = PreciseTime::now();
//             let y = GEOCODER.search(&[lat, long]).unwrap();
//             let end = PreciseTime::now();
//             println!("{} ms to search", start.to(end).num_milliseconds());

//             Ok(Response::with((status::Ok, json::encode(y).unwrap())))
//         }
//         None => Ok(Response::with((status::BadRequest, "Need a lat/long"))),
//     }
// }

fn main() {
    // Iron::new(geocoder_middleware).http("localhost:3000").unwrap();
    rocket::ignite().mount("/", routes![lookup]).launch()
    // println!("On 3000");
}
