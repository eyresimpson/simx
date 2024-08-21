use rocket::get;

#[get("/welcome")]
pub fn welcome_info() -> &'static str {
    "welcome, simx has been started."
}

