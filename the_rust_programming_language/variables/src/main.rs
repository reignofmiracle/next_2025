fn main() {
    f1();
    f2();
    f3();
}

fn f1() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

fn f2() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
}

fn f3() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

fn f4() {
    let spaces = "   ";
    let spaces = spaces.len();
}

fn f5() {
    let mut spaces = "  ";
    spaces = spaces.len();
}
