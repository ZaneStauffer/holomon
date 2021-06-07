use rocket::*;

#[get("/hello")]
pub fn hello_world() -> &'static str{
    "hello world"
}