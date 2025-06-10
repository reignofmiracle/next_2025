fn main() {
    f1();
    // f3();
    f4();
    f5();
    f6();
    f7();
    f8();
    f9();
}

fn f1() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
}

// fn f2() {
//     let condition = true;
//     let number = if condition { 5 } else { "six" };
//     println!("The value of number is: {number");
// }

fn f3() {
    loop {
        println!("again!");
    }
}

fn f4() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn f5() {
    let mut count = 0;

    let result = 'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up 10;
            }

            remaining -= 1;
        }

        count += 1;
    };

    println!("End count = {count}");

    println!("result = {result}");
}

fn f6() {
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("LEFTOFF!!!");
}

fn f7() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
}

fn f8() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn f9() {
    for number in (1..4).rev() {
        println!("the value is: {number}");
    }
    println!("LIFTOFF!!!");
}
