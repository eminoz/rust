fn main() {
    let mut scores :Vec<u16> = vec![1, 2, 3, 4, 5];
    scores.push(6);

    for score in scores.iter() {
        print!("{}, ", score);
    }

    let last = scores.pop();
    println!("\nLast element: {:?}", last);

    let mut color =Vec::new();
    color.push(String::from("Red"));

    let numbers =(8..=27).collect::<Vec<i32>>();
    println!("{:?}", numbers);
    let first_two = &numbers[0..2].to_vec();
    println!("{:?}", first_two);
}
