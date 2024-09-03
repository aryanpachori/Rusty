
fn main() {
    let  x: i32 = 10;
    println!("{}",x);

    let age: bool = true;
    if age  {
        println!("hehe")
    }

    let greeting = String::from("hello");
    println!("{}",greeting);
    //println!("{}",greeting.chars().nth(1000))

    let sentence : String = String::from("Hey there");
    let first_word = get_first(sentence);
    println!("First word is:{} ",first_word);
}
   fn get_first(sentence : String) -> String {

    let mut ans : String = String::from("");
    for char in sentence.chars(){
       ans.push_str(char.to_string().as_str());
       if char == ' '{
        break;
       }
    }
    return ans;
 }
