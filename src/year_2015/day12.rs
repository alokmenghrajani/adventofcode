use serde_json;
use serde_json::Value;

pub fn run(input: &[u8]) {
    let part1 = solve_part1(input);
    println!("part 1: {}", part1);
    assert_eq!(part1, 119433);

    let part2 = solve_part2(input);
    println!("part 2: {}", part2);
    assert_eq!(part2, 68466);
}

fn solve_part1(buf: &[u8]) -> i64 {
    let v: Value = serde_json::from_slice(buf).unwrap();

    return sum(v, true);
}

fn solve_part2(buf: &[u8]) -> i64 {
    let v: Value = serde_json::from_slice(buf).unwrap();

    return sum(v, false);
}

fn sum(v: Value, include_red: bool) -> i64 {
    return match v {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::Number(n) => n.as_i64().unwrap(),
        Value::String(_) => 0,
        Value::Array(v) => v.into_iter().map(|e| sum(e, include_red)).sum(),
        Value::Object(ref v) => {
            let mut max = 0;
            for v in v.values() {
                if (v == "red") && !include_red {
                    return 0;
                }
                max += sum(v.clone(), include_red);
            }
            return max;
        }
    };
}
