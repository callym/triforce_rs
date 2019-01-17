#[macro_use] extern crate cucumber_rust;

pub mod setup;
pub mod steps;

cucumber! {
    features: "./features",
    world: setup::Tests,
    steps: &[
        steps::init::steps
    ]
}
