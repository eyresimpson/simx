use rocket::get;

#[get("/welcome")]
pub fn welcome_info() -> &'static str {
    return "welcome, simx has been started.";
}

