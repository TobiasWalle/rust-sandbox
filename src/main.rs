mod json;

fn main() {
    json!([1, 3, 4]);
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
