extern crate simplify;
#[macro_use]
extern crate criterion;

mod benchmarks;

criterion_main! {
   benchmarks::simplify::benches,
}
