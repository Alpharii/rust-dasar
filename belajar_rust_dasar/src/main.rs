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

//scalar type

//number
#[test]
fn test_num(){
    let a: i8= 9;
    println!("{}", a);

    let b: f32= 0.5;
    println!("{}", b);
}

//number conversion
#[test]
fn test_conversion_num(){
    let a: i8= 10;
    println!("{}", a);

    let b: i16 = a as i16;
    println!("{}", b);

    let c: i32 = b as i32;
    println!("{}", c);
}

//numeric operator
#[test]
fn test_numeric_operator(){
    let a = 20;
    let b = 10;

    let c = a + b;
    println!("{}", c);

    let d = a - b;
    println!("{}", d);

    let e: i32 = a * b;
    println!("{}", e);

    let f = a / b;
    println!("{}", f);

    let g = a % b;
    println!("{}", g);
}

//augmented assignment
#[test]
fn test_augmented_assignment(){
    let mut a = 10;
    println!("{}", a);
    
    a += 10;
    println!("{}", a);

    a -= 20;
    println!("{}", a);
}

//boolean
#[test]
fn boolean(){
    let a = true;
    let b: bool = false;

    println!("{} {}", a, b)
}

//comparison operator
#[test]
fn comparison(){
    let a = 10;
    let b = 20;

    let result = a > b;

    println!("{}", result)
}

//boolean operator
#[test]
fn boolean_operator(){
    let absen = 80;
    let nilai_akhir = 80;

    let lulus_absen = absen >= 75;
    let lulus_nilai_akhir = nilai_akhir >= 75;
    let lulus_final = lulus_absen && lulus_nilai_akhir;

    println!("{}", lulus_final)
}

//compound type