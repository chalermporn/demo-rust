#[macro_use] extern crate rocket;

#[get("/")]
fn world() -> &'static str {
    "Hello, world!"
}
#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("You're a cool {} year old!", name)
}


#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/api", routes![world])
    .mount("/api", routes![hello])
}