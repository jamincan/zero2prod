#[macro_use]
extern crate rocket;

mod health_check;
mod subscriptions;

#[get("/<name>")]
fn greet(name: &str) -> String {
    format!("Hello {name}!")
}

#[get("/")]
fn index() -> String {
    greet("World")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![
            index,
            greet,
            health_check::health_check,
            subscriptions::subscribe
        ],
    )
}
