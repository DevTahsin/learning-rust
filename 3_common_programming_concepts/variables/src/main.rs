fn main() {
    tuples();
}



// fn variable(){
//     let x = 5;
//     println!("The value of x is: {x}");
//     let x = 6;
//     println!("The value of x is: {x}");
// }

fn shadowing(){
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

fn tuples() {
    let tup:(i32,u8,char) = (25212,244,'A');
    let (x, y, z) = tup;
    println!("With destructed let x,y,z: {x},{y},{z}");
    
}