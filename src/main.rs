use core::str;

fn main() {
    let name = "Emin";
    greeting(name);
    let x = 12.0;
    let y = 13.3;
    let a = sum(x, y);
    println!("{}", a);

    let numbers = vec![24, 5, 6];
    let numbers2 = [24, 5, 6, 12, 7];
    let res = get_odds(&numbers);
    let res2 = get_odds(&numbers2);

    print!("{:?}", res);
    print!("{:?}", res2);
let (a,b)=move_postition(32.0,12.8,2.4);
    print!("{:?}",a)
}

fn greeting(name: &str) {
    print!("Hello, {}!", name);
}

fn sum(x: f32, y: f32) -> f32 {
    x + y
}

fn get_odds(numbers: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    for i in numbers {
        if i % 3 == 0 {
            result.push(*i);
        }
    }
    result
}

fn move_postition(mut x: f32, mut y: f32, acceleration: f32) -> (f32, f32) {
    x += acceleration;
    y += acceleration;

    (x, y)
}
