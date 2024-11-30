pub fn read_keyboard_input(input_name: &str) -> String {
    let mut buf = String::new();

    print!("\n{}: ", input_name);

    std::io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read input");

    return buf.trim().to_string();
}
