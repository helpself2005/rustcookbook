
fn max(array: &[i32]) -> i32 {
    let mut max_index = 0;
    let mut i = 1;
    while i < array.len() {
        if array[i] > array[max_index] {
            max_index = i;
        }
        i += 1;
    }
    array[max_index]
}

#[derive(Debug)]
struct Point_1<T> {
    x: T,
    y: T
}

impl<T> Point_1<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

#[derive(Debug)]
struct Point_2<T1, T2> {
    x: T1,
    y: T2
}

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    let a = [2, 4, 6, 3, 1];
    println!("max = {}", max(&a));

    let p1 = Point_1 {x: 1, y: 2};
    println!("{:?}", p1);
    let x = p1.x;
    let y = x;
    println!("p.x = {}", p1.x());
    println!("x = {}", x);

    let p2 = Point_1 {x: 1.0, y: 2.0};
    println!("{:?}", p2);

    let p3 = Point_2 {x: 1.0, y: 2.0};
    println!("{:?}", p3);

    println!("Hello, world!");
}
