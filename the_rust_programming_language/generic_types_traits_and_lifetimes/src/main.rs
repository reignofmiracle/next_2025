fn main() {
    println!("Hello, world!");

    f1();
    f2();
}

fn f1() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {largest}");
}

fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn f2() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = largest(&number_list);

    println!("The largest number is {largest}");
}
