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

//char
#[test]
fn char_type(){
    let char1 = 'a';
    let char2 = 'b';

    println!("{} {}", char1, char2);
}

//compound type

//tuple
#[test]
fn tuple_test(){
    let data: (i32, f64, bool) = (10, 10.5, true);
    println!("{:?}", data);

    let a = data.0;
    let b = data.1;
    let c = data.2;
    println!("{} {} {}", a, b ,c);
}

//destructuring tuple
#[test]
fn destructuring_tuple(){
    let data: (i32, f64, bool) = (10, 10.5, true);
    println!("{:?}", data);

    let (a,b,c) = data;
    println!("{} {} {}", a, b ,c);
}

//mutable tuple
#[test]
fn mutable_tubple(){
    let mut data: (i32, f64, bool) = (10, 10.5, true);
    println!("{:?}", data);

    let (a,b,c) = data;
    println!("{} {} {}", a, b ,c);

    data.0 = 20;
    data.1 = 20.5;
    data.2 = false;
    println!("{:?}", data);
}

//unit (tuple kosong)
fn unit(){
    println!("Hello")
}

#[test]
fn test_unit(){
    let result = unit();
    println!("{:?}", result);

    let test = ();
    println!("{:?}", test);
}

//array
#[test]
fn test_array(){
    let arr: [i32; 5] = [1,2,3,4,5];
    println!("{:?}",arr);
}

//acces array
#[test]
fn acces_array(){
    let arr: [i32; 5] = [1,2,3,4,5];
    println!("{:?}",arr);

    let a = arr[0];
    let b = arr[1];
    println!("{} {}", a, b);
}

//mutable array
#[test]
fn mutable_array(){
    let mut arr: [i32; 5] = [1,2,3,4,5];
    println!("{:?}",arr);

    let a = arr[0];
    let b = arr[1];
    println!("{} {}", a, b);

    arr[0] = 10;
    arr[1] = 20;
    println!("{:?}",arr);
}

//hitung panjang array
#[test]
fn array_length(){
    let mut arr: [i32; 5] = [1,2,3,4,5];
    println!("{:?}",arr);

    let a = arr[0];
    let b = arr[1];
    println!("{} {}", a, b);

    arr[0] = 10;
    arr[1] = 20;
    println!("{:?}",arr);

    let length: usize = arr.len();
    println!("{:?}", length);
}

//two dimensional array
#[test]
fn two_dimension_array(){
    let matrix: [[i32; 3]; 2] = [
        [1,2,3],
        [4,5,6],
    ];

    println!("{:?}", matrix);
    println!("{:?}", matrix[0]);
    println!("{:?}", matrix[1]);
    println!("{}", matrix[0][0]);
    println!("{}", matrix[0][1]);
    println!("{}", matrix[1][0]);
    println!("{}", matrix[1][1]);
}

//constant
const MAXIMUM: i32 = 100;
#[test]
fn constant_test(){
    const MINIMUM: i32 = 0;
    println!("{} {}", MAXIMUM, MINIMUM)
}

//variable scope
#[test]
fn variable_scope(){
    let eko = 1;

    //inner scope
    {
        println!("inner eko: {}", eko);
        let kurniawan = 2;
        println!("inner kurniawan: {}", kurniawan);
    }

    // println!("inner kurniawan: {}", kurniawan); //error
}