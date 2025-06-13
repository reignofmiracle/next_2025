fn main() {
    println!("Hello, world!");
}

fn c1() {
    let v: Vec<i32> = Vec::new();
}

fn c2() {
    let v = vec![1, 2, 3];
}

fn c3() {
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
}

fn c4() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
}

fn c5() {
    let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
}

fn c6() {
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);
    // println!("The  first element is: {first}");
}

fn c7() {
    let v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
}
