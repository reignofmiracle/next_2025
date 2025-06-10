fn main() {
    println!("Hello, world!");

    another_function();
    another_function2(5);
    print_labeld_measurement(5, 'h');
    func_test();

    println!("The value of x is: {}", five());
}

fn another_function() {
    println!("Another function.");
}

fn another_function2(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeld_measurement(value: i32, unit_label: char) {
    println!("The measure is: {value}{unit_label}");
}

fn func_test() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
    println!("The value of y is: {}", {
        let x = 4;
        x + 1
    });
}

fn five() -> i32 {
    5
}
