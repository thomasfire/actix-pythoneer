extern crate actix_web;
extern crate regex;

use std::process::Command;
use std::sync::{Arc, Mutex};

use actix_web::{App, HttpResponse, HttpServer, middleware, Responder, web};
use regex::Regex;

use crate::config::Config;

fn main_page() -> impl Responder {
    HttpResponse::Ok().body(format!("
    <!DOCTYPE html>
    <html>
    <head>
        <title>Pythoneer Main</title>
    </head>
    <body>
    <a href=\"/login\" class=\"login_btn\">Hello world!!!</a>
    </body>
    </html>"))
}


fn calc_expr(info: web::Path<(String)>) -> impl Responder {
    let expr = info.as_str().to_string()
        .replace("div", "//")
        .replace("opb", "(") // open bracket
        .replace("clb", ")") // close bracket
        .replace("mul", "*")
        .replace("add", "+")
        .replace("sub", "-");

    let re = Regex::new(r"^[-0-9)(*/+]*$").unwrap();
    if !re.is_match(&expr) {
        return HttpResponse::BadRequest().body(format!("Unexpected error"));
    }

    let output = match Command::new("python3")
        .arg("calc_expr.py")
        .arg(expr)
        .output() {
        Ok(res) => res,
        Err(err) => return HttpResponse::InternalServerError().body(format!("Failed to run python3: {}", err)),
    };

    let res_str = String::from_utf8_lossy(&output.stdout);
    eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    let results: Vec<&str> = res_str.trim().split("\n").collect();
    if results[0] == "0" {
        return HttpResponse::Conflict().body("Bad formation");
    }
    println!("{:?}", results);
    HttpResponse::Ok().body(format!("{}", results[1]))
}

pub fn run_server(a_config: Arc<Mutex<Config>>) {
    let config = { a_config.lock().unwrap().clone() };

    match HttpServer::new(||
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            .service(web::resource("/main").to(main_page))
            .service(web::resource("/calc/{expr}").to(calc_expr))
    )
        .bind(config.bind_address)
        .unwrap()
        .run() {
        Ok(_) => println!("Server has been started."),
        Err(err) => eprintln!("Error on starting the server: {:?}", err)
    };
}