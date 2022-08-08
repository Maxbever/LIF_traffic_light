use std::thread;
use std::time::Duration;
use rustupolis_server::client::Client;
use rustupolis_server::repository::Repository;
use rustupolis_server::server::{Protocol, Server};
use rustupolis_server::server_launcher::ServerLauncher;
use rustupolis_server::{E, tuple};
use crate::first_layer::send_message_to_light;

pub fn second_layer(id: i32) {
    let ip_address = String::from("127.0.0.1");
    let port_tcp = String::from("90") + &*id.to_string();
    let repository = Repository::new("admin");
    let key = "an_example_very_";
    let mut client = Client::new();
    let road_linked: [i32; 13] = [0, 0, 0, 6, 9, 0, 3, 0, 11, 4, 0, 8, 0];

    repository.add_tuple_space(String::from("district_manager"), vec![String::from("admin")]);
    let server_tcp = Server::new(Protocol::TCP, &ip_address, &port_tcp, &repository,key);

    let server_launcher = ServerLauncher::new(vec![server_tcp]);

    crossbeam::scope(|scope| {
        scope.spawn(move |_| {
            server_launcher.launch_server();
        });

        client.connect(
            ip_address.clone(),
            port_tcp.clone(),
            String::from("tcp"),
            &String::from("Second_Layer"),
            key,
        );

        client.attach(
            &String::from("Second_Layer"),
            vec![String::from("admin")],
            &String::from("district_manager"),
        );

        loop {
            let data_car_passing =
                client.in_instr(vec![tuple![E::Any, E::str("nbr_cars_passing"), E::Any]]);

            if !data_car_passing.is_empty() {
                let id_car_passing = match data_car_passing.first() {
                    E::I(id) => id.clone(),
                    _ => panic!("Not a valid id"),
                };

                if road_linked.contains(&id_car_passing) {
                    let nb_car_passing = match data_car_passing.rest().rest().first() {
                        E::I(id) => id.clone(),
                        _ => panic!("Not a valid id"),
                    };

                    send_message_to_light(road_linked[id_car_passing as usize], tuple![
                        E::str("nbr_cars_coming"),
                        E::I(nb_car_passing)
                    ] )
                }
            }
            thread::sleep(Duration::new(5, 0))
        }

    }).unwrap();
}