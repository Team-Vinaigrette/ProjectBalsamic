use view;

pub fn main_loop() {
    view::console::print_hello_world();
    let test = view::console::Hello { id: 0 };
    println!("{}", test.id);
}