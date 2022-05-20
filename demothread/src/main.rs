use std::thread;

fn main() {
    println!("数组线程demo thread");

    let vec1 = vec![B(1), B(2), B(3), B(4)];
    let handle1 = thread::spawn(move || {
        println!("{:?}", vec1)
    });
    let result1 = handle1.join();
    match result1 {
        Err(e) => println!("出现运行错误{:?}", e),

        _ => println!("v1运行正确")
    }

    let vec2 = vec![B(1), B(2), B(3), B(4)];
    let handle2 = thread::spawn(|| {
        for b in vec2 {
            println!("Vec2 循环值{:?}", b)
        }
    });
    let result2 = handle2.join();
    match result2 {
        Err(e) => println!("出现运行错误{:?}", e),

        _ => println!("v2运行正确")
    }

    let mut vec3 = vec![B(1), B(2), B(3), B(4)];
    let mut b3 = vec3.get_mut(0);
    b3 = None;
    println!("b3 值{:?}", b3);
    println!("v3 值{:?}", vec3);

    let x = 10;
    for i in 0..x+1 {
        println!("current i={:?}", i);
        println!("current x-i = {:?}", x-i);
        println!("-------------------------------");
    }

    let i : i32 = 32;
    i.printhello();

    let points: [(i32, i32); 5] = [(100,200),(101,201),(102,202),(103,204),(104,204)];

    for point in points {
        println!("current point {:?}", point);
    }

    let cpoint = points.get(0);
    println!("current point {:?}", cpoint);


}


#[derive(Debug)]
struct B(i32);

trait hello {
    fn printhello(&self) -> (){
        println!("hello i32");
    }
}


impl hello for i32 {

}


