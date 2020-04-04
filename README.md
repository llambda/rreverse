# Rust Reverse Geocoder
A fast reverse geocoder in Rust. Inspired by Python [reverse-geocoder](https://github.com/thampiman/reverse-geocoder).

## Links

- [Crate](https://crates.io/crates/reverse_geocoder)
- [2.0.0-alpha.0 Docs](https://docs.rs/reverse_geocoder/2.0.0-alpha.0/reverse_geocoder/index.html)
- [1.0.1 Docs](https://docs.rs/reverse_geocoder/1.0.1/reverse_geocoder/)
- Build status: [![Build Status](https://travis-ci.org/llambda/rrgeo.svg?branch=master)](https://travis-ci.org/llambda/rrgeo)

## Description 
 
`rrgeo` takes a latitude and longitude as input and returns the closest city, country, latitude, and longitude, using a k-d tree to efficiently find the nearest neighbour based on a known list of locations. This can be useful if you need to reverse geocode a large number of coordinates quickly, or just need the rough location of coordinates but don't want the expense or complication of an online reverse geocoder.

This crate is implemented as a [library](https://crates.io/crates/reverse_geocoder), an [Actix](https://actix.rs/) REST API, an [Iron](https://github.com/iron/iron) REST API, and as a command-line utility, thanks to [Cargo workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html).

## Usage

### Command line search

Example usage:

```
> cargo run -p rrgeo-cmd --release 40 -73
71 ms to load cities.csv
3 ms to build the KdTree
(40.72788, -73.09761): West Sayville New York Suffolk County US
```

### Actix Web Server

Example usage:

```
cargo run -p rrgeo-actix --release
```

Navigate to [the local web server](http://localhost:3000/?lat=40&long=-73).

## Benchmarks 

The core library ([measured with criterion](https://github.com/japaric/criterion.rs)):

```
> cargo bench
Benchmarking search: Warming up for 3.0000 s5 ms to build the KdTree
search                  time:   [1.1530 ms 1.1639 ms 1.1748 ms]                    
                        change: [-2.6577% -1.4078% -0.0250%] (p = 0.04 < 0.05)
                        Change within noise threshold.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

```


Served over [Actix Web](https://actix.rs/):

```
> cargo run --release --bin rrgeo-actix
> wrk --latency http://localhost:3000/\?lat\=40\&long\=\-73
Running 10s test @ http://localhost:3000/?lat=40&long=-73
  2 threads and 10 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     2.47ms    0.87ms  13.01ms   74.17%
    Req/Sec     2.02k    39.42     2.07k    79.50%
  Latency Distribution
     50%    1.96ms
     75%    3.86ms
     90%    3.93ms
     99%    4.06ms
  40170 requests in 10.00s, 8.58MB read
Requests/sec:   4015.31
Transfer/sec:      0.86MB


```

Served over [Iron](http://ironframework.io/):

```
> cargo run --release --bin rrgeo-iron
> wrk --latency http://localhost:3000/\?lat\=40\&long\=\-73
Running 10s test @ http://localhost:3000/?lat=40&long=-73
  2 threads and 10 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     2.55ms    1.15ms  25.73ms   76.03%
    Req/Sec     1.99k   110.23     2.20k    82.50%
  Latency Distribution
     50%    1.96ms
     75%    3.35ms
     90%    3.92ms
     99%    5.95ms
  39563 requests in 10.01s, 8.23MB read
Requests/sec:   3954.05
Transfer/sec:    841.78KB

```


## Performance

Below we have comparisons between the Rust, Python and Node.js versions.

|              | Rust | Node   |
|--------------|------|--------|
| Load CSV     | 61ms | 1221ms |
| Build KdTree | 4ms  | 805ms  |
| Search       | 1.1ms  | 0.5ms |

Most of the performance differences appear to be in time taken to load the CSV file and create the k-d tree, but not searching the tree. Searching time resembles algorithmic complexity of [k-d tree](https://en.wikipedia.org/wiki/K-d_tree). Python version is partly implemented in C++ meaning it is not a purely Python implementation. (It might be interesting to see how a pure Python version performs.) The Node.js version is pure JavaScript, as in, not using C add-ons.

### Rust --release build

```
     Running `target/release/web`
PT0.062677465S seconds to load cities.csv
PT0.003835230S seconds to build the KdTree
PT0.068904911S seconds to search
PT0.002596743S seconds to search
PT0.002887542S seconds to search

```

### Rust --debug build

```
     Running `target/debug/web`
PT1.198010357S seconds to load cities.csv
PT0.124435778S seconds to build the KdTree
PT1.401588031S seconds to search
PT0.077837996S seconds to search
PT0.078178297S seconds to search

```

### Python mode 1 (single threaded K-D tree)

```
➜  reverse-geocoder git:(master) ✗ time python mode1.py
Loading formatted geocoded file...
[{'name': 'Saint Louis Park', 'cc': 'US', 'lon': '-93.34801', 'admin1': 'Minnesota', 'admin2': 'Hennepin County', 'lat': '44.9483'}]

python mode1.py  1.60s user 0.22s system 98% cpu 1.847 total
```

### Python mode 2 (multi threaded K-D tree)

```
➜  reverse-geocoder git:(master) ✗ time python mode2.py
Loading formatted geocoded file...
[{'name': 'Saint Louis Park', 'cc': 'US', 'lon': '-93.34801', 'admin1': 'Minnesota', 'admin2': 'Hennepin County', 'lat': '44.9483'}]

python mode2.py  2.82s user 0.34s system 142% cpu 2.221 total
```

### [nreverse](https://github.com/llambda/nreverse) (Node.js version)

```
load modules: 12.619ms
load cities.csv: 1221.833ms
create kdtree: 805.310ms
search tree: 0.758ms
search tree: 0.086ms
search tree: 0.198ms
search tree: 0.104ms
search tree: 0.031ms
total_heap_size 114mb
total_heap_size_executable 5mb
total_physical_size 112mb
total_available_size 1325mb
used_heap_size 83mb
heap_size_limit 1432mb
malloced_memory 0mb
peak_malloced_memory 4mb
does_zap_garbage 0mb

```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.


