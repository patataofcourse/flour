use flour::BCCAD;
fn main() {
    let bccad = BCCAD::from_bccad("../bread/build/libs/agb_tap.bccad");
    match bccad {
        Ok(c) => match c.to_json() {
            Ok(d) => println!("{}", d),
            Err(e) => eprintln!("Error in loading BCCAD: {}", e),
        },
        Err(e) => eprintln!("Error in loading BCCAD: {}", e),
    }
}
