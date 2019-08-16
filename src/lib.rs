use std::collections::HashSet;

pub trait Point: Clone + Eq {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
}

pub trait SqDistance<Rhs=Self>
    where Self: Point,
          Rhs: Point
{
    fn get_sq_distance(&self, rhs: &Rhs) -> f64 {
        let dx = self.x() - rhs.x();
        let dy = self.y() - rhs.y();

        dx * dx + dy * dy
    }
}

pub trait SqSegDistance<L=Self, R=Self>
    where Self: Point,
          L: Point,
          R: Point
{
    fn get_sq_seg_distance(&self, l: &L, r: &R) -> f64 {
        let mut x = l.x();
        let mut y = l.y();

        let mut dx = r.x() - x;
        let mut dy = r.y() - y;

        if dx != 0f64 && dy != 0f64 {
            let t = ((self.x() - x) * dx + (self.y() - y) * dy) / (dx * dx + dy * dy);

            if t > 1f64 {
                x = r.x();
                y = r.y();
            } else if t > 0f64 {
                x += dx * t; 
                y += dy * t;
            }
        }

        dx = self.x() - x;
        dy = self.y() - y;

        dx * dx + dy * dy
    }
}

impl<T> SqDistance<T> for T
    where T: Point
{
}

impl<T> SqSegDistance<T> for T
    where T: Point
{
}


pub struct Simplify<'a, T>
    where T: Point
{
    highest_quality: bool,
    tolerance: f64,
    points: &'a mut Vec<T>
}

impl<'a, T> Simplify<'a, T>
    where T: Point
{
    pub fn new(points: &'a mut Vec<T>) -> Self {
        Simplify {
            highest_quality: false,
            tolerance: 0.1,
            points
        }
    }

    pub fn set_highest_quality(&mut self, b: bool) -> &mut Self {
        self.highest_quality = b;
        self
    }

    pub fn set_tolerance(&mut self, t: f64) -> &mut Self {
        self.tolerance = t * t;
        self
    }

    fn simplify_radial_dist(&mut self) -> &mut Self {
        let mut prev = 0;
        let mut index = 1;

        while index < self.points.len() - 1 {
            if self.points[index].get_sq_distance(&self.points[prev]) <= self.tolerance {
                self.points.remove(index);
                index -= 1;
            } else {
                prev = index;
            }

            index += 1;
        }

        self
    }

    fn simplify_dp_step(&mut self, first: usize, last: usize, simplified_hash_set: &mut HashSet<usize>) {
        let mut max_sq_dist = self.tolerance; 
        let mut index = 0;

        for i in first+1..last {
            let sq_dist = self.points[i].get_sq_seg_distance(&self.points[first], &self.points[last]);

            if sq_dist > max_sq_dist {
                index = i; 
                max_sq_dist = sq_dist;
            }
        }

        if max_sq_dist > self.tolerance {
            if index - first > 1 {
                self.simplify_dp_step(first, index, simplified_hash_set);
            }

            simplified_hash_set.insert(index);

            if last - index > 1 {
                self.simplify_dp_step(index, last, simplified_hash_set);
            }
        }
    }

    fn simplify_douglas_peucker(&mut self, simplified_hash_set: &mut HashSet<usize>) {
        let last = self.points.len() - 1; 

        self.simplify_dp_step(0, last, simplified_hash_set);
        simplified_hash_set.insert(last);
    }

    pub fn run(&mut self) {
        if self.points.len() <= 2 {
            return;
        }

        if !self.highest_quality {
            self.simplify_radial_dist();
        }

        let mut simplified_hash_set = HashSet::new();
        simplified_hash_set.insert(0);

        self.simplify_douglas_peucker(&mut simplified_hash_set);

        let mut index = 0;
        self.points.retain(|_| {
            simplified_hash_set.contains(&(index, index += 1).0)
        });
    }
}

#[cfg(test)]
mod tests {
    use super::{Point as P, Simplify}; 

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
    
    #[test]
    fn test_simplify_with_given_tolerance() {
        let mut data = vec![
            Point {x:224.55,y:250.15},Point {x:226.91,y:244.19},Point {x:233.31,y:241.45},Point {x:234.98,y:236.06},
            Point {x:244.21,y:232.76},Point {x:262.59,y:215.31},Point {x:267.76,y:213.81},Point {x:273.57,y:201.84},
            Point {x:273.12,y:192.16},Point {x:277.62,y:189.03},Point {x:280.36,y:181.41},Point {x:286.51,y:177.74},
            Point {x:292.41,y:159.37},Point {x:296.91,y:155.64},Point {x:314.95,y:151.37},Point {x:319.75,y:145.16},
            Point {x:330.33,y:137.57},Point {x:341.48,y:139.96},Point {x:369.98,y:137.89},Point {x:387.39,y:142.51},
            Point {x:391.28,y:139.39},Point {x:409.52,y:141.14},Point {x:414.82,y:139.75},Point {x:427.72,y:127.30},
            Point {x:439.60,y:119.74},Point {x:474.93,y:107.87},Point {x:486.51,y:106.75},Point {x:489.20,y:109.45},
            Point {x:493.79,y:108.63},Point {x:504.74,y:119.66},Point {x:512.96,y:122.35},Point {x:518.63,y:120.89},
            Point {x:524.09,y:126.88},Point {x:529.57,y:127.86},Point {x:534.21,y:140.93},Point {x:539.27,y:147.24},
            Point {x:567.69,y:148.91},Point {x:575.25,y:157.26},Point {x:580.62,y:158.15},Point {x:601.53,y:156.85},
            Point {x:617.74,y:159.86},Point {x:622.00,y:167.04},Point {x:629.55,y:194.60},Point {x:638.90,y:195.61},
            Point {x:641.26,y:200.81},Point {x:651.77,y:204.56},Point {x:671.55,y:222.55},Point {x:683.68,y:217.45},
            Point {x:695.25,y:219.15},Point {x:700.64,y:217.98},Point {x:703.12,y:214.36},Point {x:712.26,y:215.87},
            Point {x:721.49,y:212.81},Point {x:727.81,y:213.36},Point {x:729.98,y:208.73},Point {x:735.32,y:208.20},
            Point {x:739.94,y:204.77},Point {x:769.98,y:208.42},Point {x:779.60,y:216.87},Point {x:784.20,y:218.16},
            Point {x:800.24,y:214.62},Point {x:810.53,y:219.73},Point {x:817.19,y:226.82},Point {x:820.77,y:236.17},
            Point {x:827.23,y:236.16},Point {x:829.89,y:239.89},Point {x:851.00,y:248.94},Point {x:859.88,y:255.49},
            Point {x:865.21,y:268.53},Point {x:857.95,y:280.30},Point {x:865.48,y:291.45},Point {x:866.81,y:298.66},
            Point {x:864.68,y:302.71},Point {x:867.79,y:306.17},Point {x:859.87,y:311.37},Point {x:860.08,y:314.35},
            Point {x:858.29,y:314.94},Point {x:858.10,y:327.60},Point {x:854.54,y:335.40},Point {x:860.92,y:343.00},
            Point {x:856.43,y:350.15},Point {x:851.42,y:352.96},Point {x:849.84,y:359.59},Point {x:854.56,y:365.53},
            Point {x:849.74,y:370.38},Point {x:844.09,y:371.89},Point {x:844.75,y:380.44},Point {x:841.52,y:383.67},
            Point {x:839.57,y:390.40},Point {x:845.59,y:399.05},Point {x:848.40,y:407.55},Point {x:843.71,y:411.30},
            Point {x:844.09,y:419.88},Point {x:839.51,y:432.76},Point {x:841.33,y:441.04},Point {x:847.62,y:449.22},
            Point {x:847.16,y:458.44},Point {x:851.38,y:462.79},Point {x:853.97,y:471.15},Point {x:866.36,y:480.77}
        ];

       let simplified = vec![
            Point {x:224.55,y:250.15},Point {x:267.76,y:213.81},Point {x:296.91,y:155.64},Point {x:330.33,y:137.57},
            Point {x:409.52,y:141.14},Point {x:439.60,y:119.74},Point {x:486.51,y:106.75},Point {x:529.57,y:127.86},
            Point {x:539.27,y:147.24},Point {x:617.74,y:159.86},Point {x:629.55,y:194.60},Point {x:671.55,y:222.55},
            Point {x:727.81,y:213.36},Point {x:739.94,y:204.77},Point {x:769.98,y:208.42},Point {x:779.60,y:216.87},
            Point {x:800.24,y:214.62},Point {x:820.77,y:236.17},Point {x:859.88,y:255.49},Point {x:865.21,y:268.53},
            Point {x:857.95,y:280.30},Point {x:867.79,y:306.17},Point {x:859.87,y:311.37},Point {x:854.54,y:335.40},
            Point {x:860.92,y:343.00},Point {x:849.84,y:359.59},Point {x:854.56,y:365.53},Point {x:844.09,y:371.89},
            Point {x:839.57,y:390.40},Point {x:848.40,y:407.55},Point {x:839.51,y:432.76},Point {x:853.97,y:471.15},
            Point {x:866.36,y:480.77}
       ]; 

       let mut s = Simplify::new(&mut data);
       s.set_tolerance(5f64);
       s.run();

       assert_eq!(data, simplified);
    }

    #[test]
    fn test_simplify_with_one_element() {
        let mut data = vec![Point {x: 1f64, y: 2f64}];

        let mut s = Simplify::new(&mut data);
        s.run();

       assert_eq!(data, vec![Point {x:1f64,y:2f64}]);
    }

    #[test]
    fn test_simplify_with_no_element() {
        let mut data: Vec<Point> = vec![];

        let mut s = Simplify::new(&mut data);
        s.run();

       assert_eq!(data, vec![]);
    }
}
