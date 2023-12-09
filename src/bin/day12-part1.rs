use serde_json::Value;

fn main() {
    let parsed =
        serde_json::from_str::<Value>(include_str!(r"..\..\input\day12.txt").trim()).unwrap();

    let mut total = 0;

    read(&parsed, &mut total);

    dbg!(total);
}

fn read(value: &Value, total: &mut i64) {
    match value {
        Value::Null | Value::Bool(_) | Value::String(_) => {}

        Value::Number(num) => *total += num.as_i64().unwrap(),

        Value::Array(arr) => {
            for v in arr {
                read(v, total);
            }
        }

        Value::Object(obj) => {
            for v in obj.values() {
                read(v, total);
            }
        }
    }
}
