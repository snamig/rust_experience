use std::env::{args, Args};

fn main() {
    let mut arg: Args = args();
    let first: f32 = arg.nth(1).unwrap().parse().unwrap();
    let operator = arg.nth(0).unwrap().chars().next().unwrap();
    let second = arg.nth(0).unwrap().parse::<f32>().unwrap();
    let result: f32  = operate(operator, first, second);
    println!("{:?}", output(operator, first, second, result));
}

fn operate(operator: char, x: f32, y: f32) -> f32{
    // if operator == '+' {
    //     return x + y;
    // } else if operator == '-'{
    //     return x - y;
    // } else if operator == '*'{
    //     return x * y;
    // } else if operator == '/'{
    //     return x/y;
    // } else {
    //     return 0.0;
    // }
    match operator {
        '+' => x + y,
        '-' => x - y,
        '*' | 'x' => x * y,
        '/' => x / y,
        _ => panic!("Invalid operator used.")  
    }

}

fn output(operator: char, x: f32, y: f32, result: f32) -> String{
    return format!("{} {} {} = {}", x, operator, y, result);
}