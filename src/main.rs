pub mod dom;

fn main() {
    let a: u32 = 12; 
    let _test_dom = dom::text(String::from("test node"));
    println!("Hello, world! {}", a);
    println!("text dom {:?}", _test_dom.node_type)
}
