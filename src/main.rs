use std::collections::HashMap;

pub mod dom;

fn main() {
    let a: u32 = 12; 
    let mut class_name = HashMap::new();
    class_name.insert(String::from("display"), String::from("block"));
    let _text_dom = dom::text(String::from("text node"));
    let _ele_dom = dom::elem(String::from("div"),class_name, Vec::new());
    println!("Hello, world! {}", a);
    println!("text dom {:?}", _text_dom);
    println!("ele dom {:?}", _ele_dom);
}
