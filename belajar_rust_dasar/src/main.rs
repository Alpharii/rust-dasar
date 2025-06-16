fn main() {
    println!("Hello, world!");
    println!("hello bintang");
    println!("hello rust");
}

//Hello world
#[test]
fn hello_test(){
    println!("hello test");
}

//immutable variable
#[test]
fn variable_test(){
    let name: &'static str = "Muhammad Bintang Alphari";
    println!("Hello {}", name);
}

//mutable variable
#[test]
fn test_mutable(){
    let mut name = "Muhammad Bintang Alphari";
    println!("Hello {}", name);
    
    name = "Alphari";
    println!("Hello {}", name);
}

//shadowing
#[test]
fn test_shadowing(){
    let name: &'static str = "Muhammad Bintang Alphari";
    println!("Hello {}", name);

    let name: &'static str = "Alphari";
    println!("Hello {}", name);
}