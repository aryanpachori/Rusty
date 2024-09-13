use std::collections::HashMap;



fn main() {
   let mut vec = Vec::new();
   vec.push(1);
   vec.push(2);
   vec.push(5);
   vec.push(8);
   vec.push(6);
   vec.push(0);
    even_values(&vec);
  // println!("vector: {:?}",vec);
   hashmap();
}

fn even_values(vec:&Vec<i32>){
    let mut new_vec: Vec<i32> = Vec::new();
   for i in vec{
    if i%2 != 0{
        continue;
    }
    new_vec.push(*i);
    
   }
  // println!("Even elements {:?}",new_vec)
}

fn hashmap(){
    let mut map : HashMap<String,i32> = HashMap::new();

    map.insert(String::from("Aryan"), 32);
    map.insert(String::from("Raman"), 18);

    let find_user = map.get("Aryan");
    println!("Before matching:{:?}",find_user);
   
    match find_user {
        Some(age)=>{
            println!("after matching :{}",age);
        }
        None  => {
            println!("Value not found");
        }
    }

}