#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;

#[derive(FromFormField)]
enum Lang {
    #[field(value = "en")]
    English
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    greetings.push_str("Hello, world!");
}
