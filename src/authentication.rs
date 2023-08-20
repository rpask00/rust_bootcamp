mod models;

pub fn authenticate(credentials: models::Credentials) -> models::LoginStatus {
    if credentials.password == "qwerty" {
        models::LoginStatus::Success
    } else {
        models::LoginStatus::Failure
    }
}


