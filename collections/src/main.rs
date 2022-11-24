mod vector;
mod string;
mod hashmap;

fn main() {
    test_vector();

    println!();
    test_string();

    println!();
    test_hashmap();
}

fn test_hashmap() {
    hashmap::test_hashmap();
    println!();
    hashmap::test_update_hashmap();
    println!();
    hashmap::test_exists_and_if_not_create();
    println!();
    hashmap::test_string_word_count();
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



