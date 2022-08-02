use std::{env, thread};
use std::time::Duration;
use rand::Rng;
use rustupolis_server::client::Client;
use rustupolis_server::repository::Repository;
use rustupolis_server::server::{Protocol, Server};
use rustupolis_server::server_launcher::ServerLauncher;
use rustupolis_server::tuple;
use crate::E::D;
use crate::tuple::E;

fn main() {
    let args: Vec<String> = env::args().collect();
    let traffic_id = &args[1];
    let ip_address = String::from("127.0.0.1");
    let port_tcp = String::from("900") + traffic_id;
    let repository = Repository::new("admin");
    let key = "an_example_very_";
    let mut client = Client::new();

    repository.add_tuple_space(String::from("Traffic_light_") + traffic_id, vec![String::from("admin")]);
    repository.add_tuple_to_tuple_space(String::from("Traffic_light_") + traffic_id, tuple![E::str("state"), E::str("green")]);

    let server_tcp = Server::new(Protocol::TCP, &ip_address, &port_tcp, &repository,key);

    let server_launcher = ServerLauncher::new(vec![server_tcp]);

    crossbeam::scope(|scope| {
        scope.spawn(move |_| {
            server_launcher.launch_server();
        });

        scope.spawn(move |_| {
            let ip_address = String::from("127.0.0.1");
            let mut client = Client::new();
            let attribute = String::from("attribute");
            let tuple_space_name = String::from("intersection_manager");
            let first_layer = String::from("first_layer");
            let key = "an_example_very_";

            thread::sleep(Duration::new(30, 0));

            client.connect(
                ip_address,
                String::from("9013"),
                String::from("tcp"),
                &first_layer,
                key,
            );

            client.attach(&first_layer, vec![attribute.clone()], &tuple_space_name);

            loop {
                thread::sleep(Duration::new(30, 0));
                let random_number = rand::thread_rng().gen_range(5..15);

                client.out(vec![tuple![E::str("nbr_cars_passing"), E::I(random_number)]])
            }
        });

        loop {
            client.connect(
                ip_address.clone(),
                port_tcp.clone(),
                String::from("tcp"),
                &String::from("Traffic_light"),
                key,
            );

            client.attach(&String::from("Traffic_light"), vec![String::from("admin")], &(String::from("Traffic_light_") + traffic_id));

            let state = client.in_instr(vec![tuple![E::str("state"),E::Any]]);

            dbg!(state);
            
            thread::sleep(Duration::new(5, 0))
        }

    }).unwrap();
}