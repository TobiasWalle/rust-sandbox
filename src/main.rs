mod json;

fn try_json_macro() {
    println!("-- Json --");
    dbg!(json!([1, 3, 4]));
    dbg!(json!({
        "Cheese": 1,
        "Pizza": 5.,
        "Pasta": 0.99,
        "Test": true,
        "array": [1, 2, 3],
        "object": {
            "a": 1
        }
    }));
}

fn try_first_macro() {
    println!("-- First Macro --");
    first_macro::communicate!(say loud "My first procedural macro works :)");
}

fn main() {
    try_json_macro();
    try_first_macro();
}
