#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate baby_watcher;

use baby_watcher::start_stream;
use std::thread;
use std::io;
use std::fs::File;
use std::io::prelude::*;
use rocket::response::Stream;
use rocket::response::Response;


#[get("/launch_stream")]
fn launch_stream() -> &'static str {
    thread::spawn(|| {
        start_stream();
    });
    "Stream started"
}

#[get("/stream")]
fn get_stream(){
    let response = Response::build()
        .streamed_body(File::open("fade.h264").unwrap())
        .finalize();
}


#[get("/")]
fn index() -> &'static str {
    "Welcome to baby watcher"
}


fn main() {
    rocket::ignite().mount("/", routes![index, launch_stream, get_stream]).launch();
}
