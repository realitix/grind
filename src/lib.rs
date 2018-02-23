/* Explanation
 * You don't need no_mangle if the function is not exported directly
 * Is extern mandatory ? No because it affects symbol but WE are using the address
 * is pub mandatory ? Yes if it's an inner module
*/
#[cfg(feature = "foo")]
pub mod modu;


#[no_mangle]
pub extern "C" fn getProcAddr() -> *mut() {
    modu::toto();
    modu::hello as *mut()
}

