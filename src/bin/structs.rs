fn main(){
    // Tuple structs
    struct RGB(u8, u8, u8);
    let black = RGB(0, 0, 0);
    println!("Black = ({}, {}, {})", black.0, black.1, black.2);
    struct CMYK (u8, u8, u8, u8);
    let cyan = CMYK(0, 255, 255, 0);
    print!("Cyan = ({}, {}, {}, {})", cyan.0, cyan.1, cyan.2, cyan.3);
}
