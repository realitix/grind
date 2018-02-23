#[no_mangle]
pub extern "C" fn hello() {
    println!("Hello from rust");
}

#[no_mangle]
pub extern fn toto() {
}
