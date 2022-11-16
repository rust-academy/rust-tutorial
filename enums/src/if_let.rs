
pub fn test_if_let() {

    let config_max:Option<i32> = Some(42);

    match config_max {
        Some(max) => println!("The maximum: {}", max),
        _ => (),
    }

    if let Some(max) = config_max {println!("The maximum: {}", max)}



}