use rocket::get;

#[get("/")]
pub fn hello() -> &'static str {
    "Hello, World!"
}
