#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::response::content::{Html, JavaScript};
use rocket::response::status;
use rocket::State;
use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::sync::Mutex;
use uuid::Uuid;

struct Users {
    // we are going to mutate it later
    user_map: Mutex<HashMap<Uuid, Vec<IpAddr>>>,
}

#[get("/")]
fn index() -> Html<&'static str> {
    Html(include_str!("assets/index.html"))
}

#[get("/vpnKiller.js")]
fn js() -> JavaScript<&'static str> {
    JavaScript(include_str!("assets/vpnKiller.js"))
}

#[get("/<uuid_string>")]
fn get_ip(
    uuid_string: String,
    remote_addr: SocketAddr,
    users: State<Users>,
) -> status::Custom<&'static str> {
    let uuid = match Uuid::parse_str(&uuid_string[..]) {
        Ok(uuid) => uuid,
        // some browsers make requests like favicon.ico, that also gets routed here
        Err(_) => return status::Custom(Status::BadRequest, "invalid uuid"),
    };
    let ip_addr = remote_addr.ip();
    let mut users_lock = users.user_map.lock().unwrap();

    if users_lock.contains_key(&uuid) {
        // we need a way to mutate users_lock[&uuid]
        let vec_lock = users_lock.get_mut(&uuid).unwrap();

        if !vec_lock.contains(&ip_addr) {
            // found a different IP address, remember and print it
            vec_lock.push(ip_addr);
            println!("{} IP changed | known IPs: {:?}", &uuid, &vec_lock);
        }
    } else {
        // the vector containing all IPs of this new user must be mutable
        users_lock.insert(uuid, vec![ip_addr]);
        println!("{} connected | Initial IP: {}", &uuid, &ip_addr);
    }

    status::Custom(Status::Ok, "")
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, js, get_ip])
        .manage(Users {
            user_map: Mutex::new(HashMap::new()),
        })
}

fn main() {
    rocket().launch();
}
