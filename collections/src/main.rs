mod vector;
mod string;

fn main() {
    test_vector();
    println!();
    test_string();
}

fn test_string() {
    string::test_string();
    string::test_concat_string();
    string::test_string_iteration();
}

fn test_vector() {
    println!("Hello, vector!");
    vector::test_create_vec();
    vector::test_unsafe_vector_access();
    vector::test_safe_vec_access();
    vector::test_iterate_vector();
    vector::test_mixed_type_vec();
}



