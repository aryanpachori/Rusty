

fn main() {
    let a = String::from("Aryan");
    let b = String::from("Hello World");
    let longest = longest_str(&a, &b);
    println!("longest string is: {}",longest);
}
fn longest_str<'a>(a: &'a str , b : &'a str)-> &'a str{
    if a.len() > b.len(){
        return a;
    }else{
        return b;
    }
}