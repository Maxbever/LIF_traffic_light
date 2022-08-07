use rustupolis_server::client::Client;
use rustupolis_server::repository::Repository;
use rustupolis_server::server::{Protocol, Server};
use rustupolis_server::server_launcher::ServerLauncher;
use rustupolis_server::tuple::Tuple;
use rustupolis_server::{tuple, E};
use std::thread;
use std::time::Duration;

pub fn first_layer(id: i32) {
    let ip_address = String::from("127.0.0.1");
    let port_tcp = String::from("90") + &*id.to_string();
    let repository = Repository::new("admin");
    let key = "an_example_very_";
    let mut client = Client::new();
    let traffic_light_link: [i32; 13] = [0, 4, 3, 2, 1, 8, 7, 6, 5, 12, 11, 10, 9];
    let traffic_light_other_sens: [i32; 13] = [0, 2, 1, 1, 2, 6, 5, 5, 6, 10, 9, 9, 10];

    repository.add_tuple_space(
        String::from("intersection_manager"),
        vec![String::from("admin")],
    );
    let server_tcp = Server::new(Protocol::TCP, &ip_address, &port_tcp, &repository, key);

    let server_launcher = ServerLauncher::new(vec![server_tcp]);

    crossbeam::scope(|scope| {
        scope.spawn(move |_| {
            server_launcher.launch_server();
        });

        client.connect(
            ip_address.clone(),
            port_tcp.clone(),
            String::from("tcp"),
            &String::from("First_Layer"),
            key,
        );

        client.attach(
            &String::from("First_Layer"),
            vec![String::from("admin")],
            &String::from("intersection_manager"),
        );

        loop {
            let data_car_passing =
                client.in_instr(vec![tuple![E::Any, E::str("nbr_cars_passing"), E::Any]]);
            let data_car_waiting =
                client.in_instr(vec![tuple![E::Any, E::str("nbr_cars_waiting"), E::Any]]);

            if !data_car_passing.is_empty() {
                let id_car_passing = match data_car_passing.first() {
                    E::I(id) => id.clone(),
                    _ => panic!("Not a valid id"),
                };

                let nb_car_passing = match data_car_passing.rest().rest().first() {
                    E::I(id) => id.clone(),
                    _ => panic!("Not a valid id"),
                };

                send_message_to_second_layer(
                    id_car_passing,
                    tuple![E::I(id_car_passing), E::I(nb_car_passing)],
                );
            }

            if !data_car_waiting.is_empty() {
                let id_car_waiting = match data_car_waiting.first() {
                    E::I(id) => id.clone(),
                    _ => panic!("Not a valid id"),
                };

                let nb_car_waiting = match data_car_waiting.rest().rest().first() {
                    E::I(id) => id.clone(),
                    _ => panic!("Not a valid id"),
                };

                if nb_car_waiting >= 8 {
                    send_message_to_light(id_car_waiting, tuple![E::str("state"), E::str("green")]);
                    send_message_to_light(
                        traffic_light_link[(id_car_waiting as usize)],
                        tuple![E::str("state"), E::str("green")],
                    );

                    let id_to_pass_red_light = traffic_light_other_sens[(id_car_waiting as usize)];

                    send_message_to_light(
                        id_to_pass_red_light,
                        tuple![E::str("state"), E::str("red")],
                    );
                    send_message_to_light(
                        traffic_light_link[(id_to_pass_red_light as usize)],
                        tuple![E::str("state"), E::str("red")],
                    );
                }
            }

            thread::sleep(Duration::new(5, 0))
        }
    })
    .unwrap();
}

pub fn send_message_to_light(id: i32, tuple: Tuple) {
    let ip_address = String::from("127.0.0.1");
    let mut port_tcp = String::from("900") + &*id.to_string();
    if id > 10 {
        port_tcp = String::from("90") + &*id.to_string();
    }
    let key = "an_example_very_";
    let mut client = Client::new();

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
        &(String::from("Traffic_light_") + &*id.to_string()),
    );

    client.out(vec![tuple]);
}

fn send_message_to_second_layer(id: i32, tuple: Tuple) {
    let ip_address = String::from("127.0.0.1");
    let mut port_tcp = String::from("900") + &*id.to_string();
    if id > 10 {
        port_tcp = String::from("90") + &*id.to_string();
    }
    let key = "an_example_very_";
    let mut client = Client::new();

    client.connect(
        ip_address.clone(),
        port_tcp.clone(),
        String::from("tcp"),
        &String::from("Second_layer"),
        key,
    );

    client.attach(
        &String::from("Second_layer"),
        vec![String::from("admin")],
        &String::from("district_manager"),
    );

    client.out(vec![tuple]);
}
