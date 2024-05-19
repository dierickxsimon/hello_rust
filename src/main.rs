#[allow(unused_variables)]
fn main() {
    //just some test code to learn rust
    //================================================================
    let mut x: i32 = 5;
    let z: i32 = 10;

    x += 1;

    assert_eq!(x, 6);
    println!("success");

    let z: i32 = 10;
    let y: i32 = 5;
    {
        let y: i32 = 10; // a variabel is only valid in this scope
        println!("the value of z is {}, and y is {}", z, y);
    }

    println!("the value of z is {}, and y is {}", z, y);

    define_x();

    let y: i32 = 10;
    //shadowing
    let y: &str = "hello";
    println!("the value of y is {}", y);

    let (mut x, y) = (10, 20);
    x +=1;
    assert_eq!(x,11);
    assert_eq!(y,20);

    let (a, b);

    (a, ..) = (3, 4); // .. we dont care
    [.., b] = [1, 2]; // [] is array

    assert_eq!([a,b],[3, 2])


    
}

fn define_x(){
    let x: &str = "hello";
    println!("the value of x is {}", x);
}

