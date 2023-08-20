pub struct Credentials {
    pub username: String,
    pub password: String,
}

pub enum LoginStatus {
    Success,
    Failure,
}
