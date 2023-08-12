const PLAYER_AGE: u8 = 10;
const PLAYER_NAME: String = String::from("John Doe");
static CASINO_NAME: &str = "The Venetian";
static COMPANY_NAME: String = String::from("The Venetian co");

// default approach should be to use const, unless you need to use large amount of data with single instance

fn main() {
    //  const
    let pa1 = PLAYER_AGE;
    let pa2 = PLAYER_AGE; // const primitive is copied

    let pn1 = PLAYER_NAME;
    let pn2 = PLAYER_NAME; // const primitive is copied

    // static
    let c1 = CASINO_NAME;
    let c2 = CASINO_NAME; // static primitive is copied

    let co1 = COMPANY_NAME; // static complex type is moved
    let co2 = COMPANY_NAME;
}
