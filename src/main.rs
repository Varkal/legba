#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

pub mod controllers;
pub mod services;
pub mod forms;
pub mod models;
pub mod overrides;

use services::entity_manager::EntityManager;
use models::medium_post::MediumPost;
use rocket::fairing::AdHoc;

fn main() {
    rocket::ignite()
        .mount("/api/", controllers::routes())
        .attach(AdHoc::on_attach("Entity manager", |server| {
            let table_name = server.config().get_str("dynamo_table").unwrap().to_string();
            let server = server.manage(EntityManager::<MediumPost>::new(table_name));
            Ok(server)
        }))
        .launch();
}
