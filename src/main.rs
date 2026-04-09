fn main() {
    println!("--- Section 3 ---");
    section3();

    println!("--- Section 4 ---");
    section4();

    println!("--- Section 6 ---");
    section6();
}

fn section3() {
    let x = 5;
    println!("x = {}", x);
}

fn section4() {
    let x = 5;
    let x = x + 1;
    println!("x = {}", x);
}

fn section6() {
    const MAX: u32 = 100000;
    println!("{}", MAX);
}