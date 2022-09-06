#[macro_use] extern create rocket;

#[get("/world")]
fn world() -> &'static str {
    "world"
}

#[launch]

fn rocket() -> _ {
    rocket::build(): Rocket<Build>
        .mount(base: "/hello", routes: routes![world])
}
