pub fn test_string() {
    let s1 = String::new();
    let _ref_string = &s1;

    let s2 = "Hell world";
    let _string_from_ref = s2.to_string();

    let mut s = String::from("foo");

    s.push_str("bar");
    s.push_str("baz");
    s.push('!');

    println!(" Final string {}", s);
}

pub fn test_concat_string() {
    let protocol = String::from("https://");
    let host = String::from("localhost");
    let port = String::from("8080");

    //let s3 = protocol +&host +&port;

    let s3 = format!("{}{}:{}", protocol, host, port);

    println!("Resulting string: {}", s3)
}

pub fn test_string_iteration() {
    let hello = String::from("नमस्ते");

    for b in hello.as_bytes() {
        print!(" {} ", b);
    }

    println!();

    for b in hello.chars() {
        print!(" {} ", b);
    }

    // unicode_segmentation
    // cargo add unicode_segmentation
    // grapheme
}
