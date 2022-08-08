extern crate core;

use std::env;

mod first_layer;
mod second_layer;
mod traffic_light;

fn main() {
    let args: Vec<String> = env::args().collect();
    let traffic_id = &args[1];
    //traffic_light::traffic_light(traffic_id.parse::<i32>().unwrap());
    first_layer::first_layer(traffic_id.parse::<i32>().unwrap());
    //second_layer::second_layer(traffic_id.parse::<i32>().unwrap());
}
