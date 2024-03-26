#[get("/welcome")]
pub fn welcome_info() -> &'static str {
    return "simx has been started.";
}