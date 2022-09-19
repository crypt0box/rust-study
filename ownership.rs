pub fn run() {
    // // 1⃣
    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{}, {}", s1, s2);

    // // 2⃣
    // let s1 = String::from("hello");
    // let s2 = s1.clone();
    // println!("{}, {}", s1, s2);
    // println!("{:?}, {:?}", s1.as_ptr(), s2.as_ptr());

    // // 3⃣
    // let s1 = String::from("hello");
    // let len = calc_string_length(s1);
    // println!("{}, {}", s1, len);

    // 4⃣
    let s1 = String::from("hello");
    let len = calc_string_length(&s1);
    println!("{}, {}", s1, len);
}

// // 3⃣
// fn calc_string_length(s: String) -> usize {
//     s.len()
// }

// 4⃣
fn calc_string_length(s: &String) -> usize {
    s.len()
}
