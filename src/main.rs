use std::collections::HashMap;

macro_rules! enum_wrap {
    ($enum:ty, $member:tt, $type:ty) => {
        impl From<$type> for $enum {
            fn from(value: $type) -> Self {
                Self::$member(value)
            }
        }
    };
}

#[derive(Debug)]
enum JsonValue {
    String(String),
    NumberI32(i32),
    NumberF32(f32),
    Boolean(bool),
    List(Vec<JsonValue>),
    Object(Json),
}

enum_wrap!(JsonValue, String, String);
enum_wrap!(JsonValue, NumberI32, i32);
enum_wrap!(JsonValue, NumberF32, f32);
enum_wrap!(JsonValue, Boolean, bool);
enum_wrap!(JsonValue, List, Vec<JsonValue>);
enum_wrap!(JsonValue, Object, Json);

type Json = HashMap<String, JsonValue>;

macro_rules! json {
    ({ $($key:tt:$value:tt),* $(,)?}) => {{
        let mut map = Json::new();
        $(
        map.insert(
          $key.to_string(),
          JsonValue::from(json!($value))
        );
        )*
        map
    }};
    ([ $($item:expr),* ]) => ({
        let mut vec = Vec::new();
        $(
        vec.push(JsonValue::from($item));
        )*
        vec
    });
    ($value:expr) => { $value }
}

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
