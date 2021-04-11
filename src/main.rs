use std::collections::HashMap;

pub mod dom;

fn main() {
    let mut class_name = HashMap::new();

    class_name.insert(String::from("display"), String::from("block"));

    let _text_dom = dom::text(String::from("text node"));
    let children = vec![_text_dom];
    let _ele_dom = dom::elem(String::from("div"),class_name, children);

    println!("ele dom {:?}", _ele_dom);
}
