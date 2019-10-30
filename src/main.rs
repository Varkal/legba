#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

pub mod overrides;
pub mod controllers;
pub mod models;
pub mod fairings;
pub mod forms;

fn main() {
    rocket::ignite()
        .mount("/api/", controllers::routes())
        .launch();
}
