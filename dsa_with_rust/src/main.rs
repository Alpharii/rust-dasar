fn reverse_string(s: &str) -> String {
    let mut result = String::new();
    let string = s.to_string();
    for i in string.chars().rev(){
        result.push(i);
    };
    return result;

    // return s.chars().rev().collect(); //yang benar
}

fn main() {
    let reverse = reverse_string("rust");
    println!("reverse string = {}", reverse);
}