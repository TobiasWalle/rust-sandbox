use std::collections::HashMap;

macro_rules! impl_json_value {
    ({ $($name:tt($for_type:ty)),* $(,)? }) => {
        #[derive(Debug)]
        pub enum JsonValue {
            $($name($for_type),)*
        }

        $(impl From<$for_type> for JsonValue {
            fn from(value: $for_type) -> Self {
                Self::$name(value)
            }
        })*
    };
}

pub type Json = HashMap<String, JsonValue>;

impl_json_value!({
    String(String),
    ISize(isize),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    F32(f32),
    F64(f64),
    Bool(bool),
    List(Vec<JsonValue>),
    Object(Json)
});

#[macro_export]
macro_rules! json {
    ({ $($key:tt:$value:tt),* $(,)?}) => {{
        let mut map = crate::json::Json::new();
        $(
        map.insert(
          $key.to_string(),
          crate::json::JsonValue::from(json!($value))
        );
        )*
        map
    }};
    ([ $($item:expr),* ]) => ({
        let mut vec = Vec::new();
        $(
        vec.push(crate::json::JsonValue::from($item));
        )*
        vec
    });
    ($value:expr) => { $value }
}
