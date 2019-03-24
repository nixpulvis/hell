fn main() {
    // Create a Vec `grades`.
    let grades = vec!["A", "B", "C", "D", "E", "F"];

    // Print the list
    println!("{:?}", grades);

    // Print the list mapped with "+" added to each grade.
    println!("{:?}", grades.iter().map(|g| format!("{}+", g)).collect::<Vec<_>>());

    // Print the list mapped and reversed.
    println!("{:?}", grades.iter().rev().map(|g| format!("{}+", g)).collect::<Vec<_>>());

    // On to the fun Rust stuff...

    // let message = "hello".to_string();
    // // `fn String::len(&self) -> usize`
    // println!("message length: {:?}", message.len());
    // // `fn String::into_bytes(self) -> Vec<u8>`
    // let bytes = message.into_bytes();
    // println!("message bytes: {:?}", bytes);
    // println!("message: {:?}", message);

    // // New and exciting stuff, `Vec::into_iter`
    // println!("{:?}", grades.into_iter().map(|g| format!("{}+", g)).collect::<Vec<_>>());
    // // println!("{:?}", grades);

    // let mut numbers = vec![1, 2, 3, 4];
    // let r = &mut numbers;
    // r.remove(0);
    // // println!("{:?}", numbers);

    // let mut v = vec![1, 2, 3];
    // for i in v.iter_mut() {
    //     v.swap(1, 2);
    // }

    // use std::thread;
    // let mut buffer = vec![1, 2, 3];
    // buffer.push(4);
    // buffer.push(5);
    // thread::spawn(|| buffer.push(6));
    // thread::spawn(|| buffer.push(7));
    // println!("{:?}", buffer);
}
