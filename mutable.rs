pub fn run() {
    // // 1⃣
    // // 変数の定義はletを使う
    // let x = 5;
    // println!("x is: {}", x);
    // // Rustでは変数は標準で不変なので再代入できない
    // x = 6;
    // println!("x is: {}", x);

    // // 2⃣
    // // mutを付けると再代入可能となる
    // let mut x = 5;
    // println!("x is: {}", x); // x is: 5
    // x = 6;
    // println!("x is: {}", x); // x is: 6

    // // 3⃣
    // // 定数定義はconstで
    // // constを使う場合は必ず型注釈する
    // const ONE_KB: u32 = 1024;
    // println!("const is: {}", ONE_KB); // const is: 1024
    // // letとの違いはmutを付けられないこと
    // const mut x: u32 = 1;

    // // 4⃣
    // // Rustでは変数の覆い隠し(=シャドーイング)ができる
    // // これは変数の上書きのようなもの
    // let x = 5;
    // let x = x + 1;
    // {
    //     let x = x * 2;
    //     println!("x is: {}", x); // x is: 12
    // }
    // println!("x is: {}", x); // x is: 6

    // // mutとシャドーイングの違い
    // // mutだと変数の格納場所が同じ
    // let mut y = "hello";
    // println!("y is: {:p}", &y); // y is: 0x38c66ff9ec
    // y = "hi";
    // println!("y is: {:p}", &y); // y is: 0x38c66ff9ec

    // // したがって型を変えることができない
    // y = y.len()

    // // シャドーイングだと変数の格納場所が変わる
    // let z = "hello";
    // println!("z is: {:p}", &z); // z is: 0xb0d16ff960
    // let z = "world";
    // println!("z is: {:p}", &z); // z is: 0xb0d16ff9b8

    // // したがって型を変えることができる
    // let z = z.len();
    // println!("z is: {}", z); // z is: 5
}
