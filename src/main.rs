extern crate core;

use std::env;

mod first_layer;
mod second_layer;
mod traffic_light;

fn main() {
    let args: Vec<String> = env::args().collect();
    let traffic_id = &args[1];

    crossbeam::scope(|scope| {
        scope.spawn(move |_| {
            first_layer::first_layer(13);
        });

        scope.spawn(move |_| {
            traffic_light::traffic_light(traffic_id.parse::<i32>().unwrap());
        });
    })
    .unwrap();
}
