use std::collections::HashMap;

fn main(){
    let mut fruits = HashMap::new();
    fruits.insert("apple", 3);
    fruits.entry("banana").or_insert(4);
    fruits.remove("apple");

    for(i,v) in fruits.iter().enumerate() {
        println!("{:?} : {:?}", i, v);
    }

    if fruits.get("apple") == None {
        println!("apple이 존재하지 않음");
    } else {
        println!("apple이 존재함 : {}", fruits["apple"]);
    }

}