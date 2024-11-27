use std::cell::RefCell;


thread_local! {
    static MSG: RefCell<String> = RefCell::new(String::new());
}

#[ic_cdk::update]
fn save_msg(msg: String) -> String {
    let a = String::from("asd");
    let b = String::from("asd");
    MSG.with(|static_msg| *static_msg.borrow_mut() = n);
}



#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
