enum Command {
    Click,
    Drag,
    Replace(String, String),
}

impl Command {
    fn serialize(self) -> String {
        match self {
            Command::Click => format!(
                "{{\
                \"type\": \"click\",\
            }}"
            ),
            Command::Drag => format!(
                "{{\
                \"type\": \"drag\",\
            }}"
            ),
            Command::Replace(s1, s2) => format!(
                "{{\
                \"type\": \"replace\",\
                \"from\": \"{}\",\
                \"to\": \"{}\"\
            }}",
                s1, s2
            ),
        }
    }
}

fn main() {
    let mut command = Command::Replace(String::from("hello"), String::from("world"));

    match &command {
        Command::Click => println!("Click"),
        Command::Drag => println!("Drag"),
        Command::Replace(s1, s2) => println!("Replace {} with {}", s1, s2),
        _ => println!("Default"),
    }

    if let Command::Replace(s1, s2) = &mut command {
        *s1 = String::from("world");
        println!("Replace {} with {}", s1, s2);
    }

    println!("{}", command.serialize());

    let h = 45;

    match h {
        i32::MIN..=0 => println!("Below 0"),
        1..=10 => println!("Below 11"),
        11..=100 => println!("Below 101, but above 10"),
        101..=i32::MAX => println!("Above 100"),
    }

    let opt = Some("Rainwaker");

    if let Some(name) = opt {
        // assign operator when let is used
        println!("User's name: {name}");
    }

    if None == opt {
        // equality operator when let is not used
        println!("User not found");
    }
}
