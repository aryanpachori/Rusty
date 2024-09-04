fn main() {
    stack();
     heap();
    update_string();
}

fn stack(){
    let a : i8 = 32;
    let b : i8 = 20;
    let c = a+b;

    println!("Stack function: The sum of {} and {} is {}", a, b, c);
}

fn heap(){
     let _s1 : String = String::from("Hey");
    let _s2 : String = String::from("there");

   println!("Heap fn: String s1 {} and s2 {}",_s1,_s2);
}

fn update_string(){

    let mut _str : String = String::from("Initial String");
    
    println!("Before adding {}",_str);
    _str.push_str("addtional string");

    println!("after additon{}",_str);

    for _ in 0..10{
        _str.push_str("looping");
        println!("Looping for heap ptr{}, capacity:{}, length:{}, pointer:{:p}",_str,_str.capacity(),_str.len(),_str.as_ptr())
    }
    }
   