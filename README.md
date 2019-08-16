## simplify.rs

port of [simplify-js](https://github.com/mourner/simplify-js)

### Installation

```bash
cargo add simplify
```

### Usage

```rust
use simplify::{Point as P, Simplify}; 

#[derive(Clone, Debug)]
pub struct Point {
    pub x: f64,
        pub y: f64
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }  
}

impl Eq for Point {}

impl P for Point {
    fn x(&self) -> f64 {
        self.x 
    }

    fn y(&self) ->f64 {
        self.y
    }
}

let mut data = vec![
    Point {x:224.55,y:250.15},Point {x:226.91,y:244.19},Point {x:233.31,y:241.45},Point {x:234.98,y:236.06},
    Point {x:244.21,y:232.76},Point {x:262.59,y:215.31},Point {x:267.76,y:213.81},Point {x:273.57,y:201.84},
    Point {x:273.12,y:192.16},Point {x:277.62,y:189.03},Point {x:280.36,y:181.41},Point {x:286.51,y:177.74},
    Point {x:292.41,y:159.37},Point {x:296.91,y:155.64},Point {x:314.95,y:151.37},Point {x:319.75,y:145.16},
    Point {x:330.33,y:137.57},Point {x:341.48,y:139.96},Point {x:369.98,y:137.89},Point {x:387.39,y:142.51},
    Point {x:391.28,y:139.39},Point {x:409.52,y:141.14},Point {x:414.82,y:139.75},Point {x:427.72,y:127.30},
    Point {x:439.60,y:119.74},Point {x:474.93,y:107.87},Point {x:486.51,y:106.75},Point {x:489.20,y:109.45},
];

let mut s = Simplify::new(&mut data);
s.set_tolerance(5f64);
s.run();

println!("{:?}", data);
```
