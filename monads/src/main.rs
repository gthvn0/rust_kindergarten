struct NumberWithLogs {
    result: i32,
    logs: Vec<String>,
}

fn square(x: i32) -> NumberWithLogs {
    let result = x * x;
    let mut log = vec![];
    log.push(format!("Squarred {} to get {}", x, result));

    NumberWithLogs {
        result: result,
        logs: log,
    }
}

fn add_one(x: i32) -> NumberWithLogs {
    let result = x + 1;
    let mut log = vec![];
    log.push(format!("Added 1 to {} to get {}", x, result));

    NumberWithLogs {
        result: result,
        logs: log,
    }
}

fn run_with_logs(input: NumberWithLogs, transform: fn(x:i32) -> NumberWithLogs) -> NumberWithLogs {
    let new_n = transform(input.result);

    // Transformation
    let mut vlogs1 = input.logs.clone();
    let mut vlogs2 = new_n.logs.clone();
    vlogs1.append(&mut vlogs2);

    NumberWithLogs {
        result: new_n.result,
        logs: vlogs1,
    }
}

fn wrap_with_logs(x: i32) -> NumberWithLogs {
    NumberWithLogs {
        result: x,
        logs: vec![],
    }
}
// Call with logging
//
// add_one(square(2)) => {
//  result: 5,
//  logs: [
//      "Squarred 2 to get 4.",
//      "Added 1 to 4 to get 5."
//  ]
// }

fn main() {
    let x:i32 = 2;

    let mut a: NumberWithLogs = wrap_with_logs(x);
    println!("{} | {:?}", a.result, a.logs);

    a = run_with_logs(a, add_one);
    println!("{} | {:?}", a.result, a.logs);

    a = run_with_logs(a, square);
    println!("{} | {:?}", a.result, a.logs);
}
