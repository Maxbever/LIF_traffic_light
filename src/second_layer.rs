use rustupolis_server::client::Client;
use rustupolis_server::repository::Repository;
use rustupolis_server::server::{Protocol, Server};
use rustupolis_server::server_launcher::ServerLauncher;

pub fn second_layer(id: i32) {
    let ip_address = String::from("127.0.0.1");
    let port_tcp = String::from("90") + &*id.to_string();
    let repository = Repository::new("admin");
    let key = "an_example_very_";
    let mut client = Client::new();

    repository.add_tuple_space(String::from("Second_layer") + &*id.to_string(), vec![String::from("admin")]);
    let server_tcp = Server::new(Protocol::TCP, &ip_address, &port_tcp, &repository,key);

    let server_launcher = ServerLauncher::new(vec![server_tcp]);
}