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
    let port_tcp = String::from("900") + &*traffic_id.to_string();
    let repository = Repository::new("admin");
    let key = "an_example_very_";
    let mut client = Client::new();
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
            let mut client = Client::new();
            let attribute = String::from("admin");
            let tuple_space_name = String::from("intersection_manager");
            let first_layer = String::from("first_layer");
            let key = "an_example_very_";
            let mut port_first_layer = String::from("9015");
            if traffic_id < 4 {
                port_first_layer = String::from("9013");
            } else {
                if traffic_id < 8 {
                    port_first_layer = String::from("9014");
                }
            }

            client.connect(
                ip_address,
                port_first_layer,
                String::from("tcp"),
                &first_layer,
                key,
            );

            client.attach(&first_layer, vec![attribute.clone()], &tuple_space_name);

            loop {
                let random_number = rand::thread_rng().gen_range(5..15);

                client.out(vec![tuple![
                    E::str("nbr_cars_passing"),
                    E::I(random_number)
                ]]);

                let response = receiver.recv().unwrap();
                dbg!(response);

                if response == 1 {
                    let random_number = rand::thread_rng().gen_range(4..8);

                    client.out(vec![tuple![
                        E::str("nbr_cars_waiting"),
                        E::I(random_number)
                    ]])
                }
                thread::sleep(Duration::new(15, 0));
            }
        });

        client.connect(
            ip_address.clone(),
            port_tcp.clone(),
            String::from("tcp"),
            &String::from("Traffic_light"),
            key,
        );

        client.attach(
            &String::from("Traffic_light"),
            vec![String::from("admin")],
            &(String::from("Traffic_light_") + &*traffic_id.to_string()),
        );

        loop {

            let state_tuple = client.read(vec![tuple![E::str("state"), E::Any]]);

            if !state_tuple.is_empty() {
                state = match state_tuple.rest().first() {
                    E::S(data) => {
                        if data == "green" {
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

            thread::sleep(Duration::new(15, 0))
        }
    })
    .unwrap();
}
