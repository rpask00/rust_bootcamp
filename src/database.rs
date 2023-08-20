mod database {
    pub struct User {
        pub username: String,
        pub password: String,
    }

    pub enum Status {
        Success,
        Failure,
    }

    pub fn connect_to_database() -> Status {
        Status::Success
    }

    pub fn get_user() -> User {
        connect_to_database();
        User {
            username: String::from("Niko"),
            password: String::from("qwerty"),
        }
    }
}
