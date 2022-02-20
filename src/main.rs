use std::env::{args,Args};

fn main() {
    let mut args:Args = args();
    println!("{:?}",&args);
    let first_arg:String = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second_arg:String = args.nth(0).unwrap();
    
    let first_num:f32 = first_arg.parse::<f32>().unwrap();
    let second_num:f32 = second_arg.parse::<f32>().unwrap();
   
    let result = resolve_operation(operator, first_num, second_num);
    println!("{}",result)    
}

fn resolve_operation(operator:char,first_number:f32, second_number:f32 ) -> f32 {
    match &operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '*' | 'x' => first_number * second_number,
        '/' => first_number / second_number,
        _ => panic!("Invalido Operation")
    }

}