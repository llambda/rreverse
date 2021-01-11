# Rust Reverse Geocoder
A fast reverse geocoder in Rust. Inspired by Python [reverse-geocoder](https://github.com/thampiman/reverse-geocoder).

## Links

- [Crate](https://crates.io/crates/reverse_geocoder)
- [2.0.0 Docs](https://docs.rs/reverse_geocoder/2.0.0/reverse_geocoder/index.html)
- [1.0.1 Docs](https://docs.rs/reverse_geocoder/1.0.1/reverse_geocoder/)

## Description 
 
`rrgeo` takes a latitude and longitude as input and returns the closest city, country, latitude, and longitude, using a k-d tree to efficiently find the nearest neighbour based on a known list of locations. This can be useful if you need to reverse geocode a large number of coordinates quickly, or just need the rough location of coordinates but don't want the expense or complication of an online reverse geocoder.

This crate is implemented as a [library](https://crates.io/crates/reverse_geocoder), an [Actix](https://actix.rs/) REST API, a [Warp](https://seanmonstar.com/post/176530511587/warp) REST API, and as a command-line utility, thanks to [Cargo workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html).

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

### Warp Web Server

Example usage:

```
cargo run -p rrgeo-warp --release
```

Navigate to [the local web server](http://localhost:3000/?lat=40&long=-73).

## Benchmarks 

Benchmarked on Intel Core i7 4790K at 4.00GHz.

The core library ([measured with criterion](https://github.com/japaric/criterion.rs)):

```
> cargo bench
Benchmarking search: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 11.7s or reduce sample count to 40.
search                  time:   [2.3144 ms 2.3440 ms 2.3760 ms]
                        change: [-3.7824% -1.2338% +1.6347%] (p = 0.36 > 0.05)
                        No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) high mild
  4 (4.00%) high severe

```

Served via [Actix Web](https://actix.rs/):

```
> cargo run --release --bin rrgeo-actix
> oha http://localhost:3000/\?lat\=40\&long\=\-73 -z 5sec
Summary:
  Success rate: 1.0000
  Total:        5.0166 secs
  Slowest:      0.0531 secs
  Fastest:      0.0023 secs
  Average:      0.0166 secs
  Requests/sec: 2985.8853

  Total data:   1.47 MiB
  Size/request: 103.00 B
  Size/sec:     300.34 KiB

Response time histogram:
  0.004 [606]  |■■■■
  0.009 [2031] |■■■■■■■■■■■■■■■
  0.013 [4248] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.017 [3815] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.022 [2415] |■■■■■■■■■■■■■■■■■■
  0.026 [1097] |■■■■■■■■
  0.030 [542]  |■■■■
  0.035 [130]  |
  0.039 [55]   |
  0.043 [34]   |
  0.048 [6]    |

Latency distribution:
  10% in 0.0092 secs
  25% in 0.0124 secs
  50% in 0.0158 secs
  75% in 0.0206 secs
  90% in 0.0252 secs
  95% in 0.0284 secs
  99% in 0.0347 secs

Details (average, fastest, slowest):
  DNS+dialup:   0.0043 secs, 0.0008 secs, 0.0099 secs
  DNS-lookup:   0.0000 secs, 0.0000 secs, 0.0001 secs

Status code distribution:
  [200] 14979 responses
```

Served via Warp:

```
> cargo run --release --bin rrgeo-warp
Summary:
Summary:
  Success rate: 1.0000
  Total:        5.0006 secs
  Slowest:      0.2553 secs
  Fastest:      0.0113 secs
  Average:      0.1263 secs
  Requests/sec: 388.9503

  Total data:   275.42 KiB
  Size/request: 145.00 B
  Size/sec:     55.08 KiB

Response time histogram:
  0.022 [9]    |
  0.044 [10]   |
  0.067 [8]    |
  0.089 [9]    |
  0.111 [78]   |■
  0.133 [1793] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.155 [0]    |
  0.177 [0]    |
  0.200 [0]    |
  0.222 [0]    |
  0.244 [38]   |

Latency distribution:
  10% in 0.1229 secs
  25% in 0.1239 secs
  50% in 0.1250 secs
  75% in 0.1266 secs
  90% in 0.1277 secs
  95% in 0.1285 secs
  99% in 0.2504 secs

Details (average, fastest, slowest):
  DNS+dialup:   0.0045 secs, 0.0009 secs, 0.0097 secs
  DNS-lookup:   0.0000 secs, 0.0000 secs, 0.0001 secs

Status code distribution:
  [200] 1945 responses
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

### [nreverse](https://github.com/ggcode1/nreverse) (Node.js version)

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


