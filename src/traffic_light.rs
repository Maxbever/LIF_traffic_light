use rand::Rng;
use rustupolis_server::client::Client;
use rustupolis_server::repository::Repository;
use rustupolis_server::server::{Protocol, Server};
use rustupolis_server::server_launcher::ServerLauncher;
use rustupolis_server::tuple;
use rustupolis_server::tuple::E;
use std::thread;
use std::time::Duration;
use std::sync::mpsc::channel;

pub fn traffic_light(traffic_id: i32) {
    let ip_address = String::from("127.0.0.1");
    let mut port_tcp = String::from("900") + &*traffic_id.to_string();
    if traffic_id >= 10 {
        port_tcp = String::from("90") + &*traffic_id.to_string();
    }
    let repository = Repository::new("admin");
    let key = "an_example_very_";
    let mut state = 0;

    repository.add_tuple_space(
        String::from("Traffic_light_") + &*traffic_id.to_string(),
        vec![String::from("admin")],
    );
    repository.add_tuple_to_tuple_space(
        String::from("Traffic_light_") + &*traffic_id.to_string(),
        tuple![E::str("state"), E::str("red")],
    );

    let server_tcp = Server::new(Protocol::TCP, &ip_address, &port_tcp, &repository, key);

    let server_launcher = ServerLauncher::new(vec![server_tcp]);

    let (sender, receiver) = channel();

    crossbeam::scope(|scope| {
        scope.spawn(move |_| {
            server_launcher.launch_server();
        });

        scope.spawn(move |_| {
            let ip_address = String::from("127.0.0.1");
            let mut first_layer_client = Client::new();
            let mut traffic_light = Client::new();
            let attribute = String::from("admin");
            let tuple_space_name = String::from("intersection_manager");
            let first_layer = String::from("first_layer");
            let mut port_tcp = String::from("900") + &*traffic_id.to_string();
            if traffic_id >= 10 {
                port_tcp = String::from("90") + &*traffic_id.to_string();
            }
            let key = "an_example_very_";

            let mut port_first_layer = String::from("9015");
            if traffic_id <= 4 {
                port_first_layer = String::from("9013");
            } else {
                if traffic_id <= 8 {
                    port_first_layer = String::from("9014");
                }
            }

            thread::sleep(Duration::new(15, 0));

            first_layer_client.connect(
                ip_address.clone(),
                port_first_layer,
                String::from("tcp"),
                &first_layer,
                key,
            );

            first_layer_client.attach(&first_layer, vec![attribute.clone()], &tuple_space_name);

            traffic_light.connect(
                ip_address.clone(),
                port_tcp.clone(),
                String::from("tcp"),
                &String::from("Traffic_light"),
                key,
            );

            traffic_light.attach(
                &String::from("Traffic_light"),
                vec![String::from("admin")],
                &(String::from("Traffic_light_") + &*traffic_id.to_string()),
            );

            loop {
                let state_tuple = traffic_light.read(vec![tuple![E::str("state"), E::Any]]);
                let car_coming = traffic_light.read(vec![tuple![E::str("nbr_cars_coming"), E::Any]]);

                let mut nb_car_coming = 0;

                if !car_coming.is_empty() {
                    nb_car_coming = match car_coming.rest().first() {
                        E::I(id) => id.clone(),
                        _ => panic!("Not a valid id"),
                    };
                }

                if !state_tuple.is_empty() {
                    state = match state_tuple.rest().first() {
                        E::S(data) => {
                            if data.contains("green") {
                                0
                            } else {
                                1
                            }
                        }
                        _ => {
                            panic!("Error");
                        }
                    };
                    sender.send(state).unwrap();
                }

                let random_number = rand::thread_rng().gen_range(5..15);

                first_layer_client.out(vec![tuple![
                    E::I(traffic_id),
                    E::str("nbr_cars_passing"),
                    E::I(random_number)
                ]]);

                let response = receiver.recv().unwrap();

                if response == 1 {
                    let random_number = rand::thread_rng().gen_range(6..9) + nb_car_coming;

                    first_layer_client.out(vec![tuple![
                        E::I(traffic_id),
                        E::str("nbr_cars_waiting"),
                        E::I(random_number)
                    ]])
                }

                thread::sleep(Duration::new(15, 0));
            }
        });
    })
    .unwrap();
}
