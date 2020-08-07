use std::ptr;

fn foo(data: Option<i32>) {
    let p = match data {
        Some(i) => vec![i].as_mut_ptr(),
        None => ptr::null_mut(),
    };
    unsafe { bar(p); }
}

unsafe fn bar(p: *mut i32) {
    if p != ptr::null_mut() {
        println!("{}", *p);
    }
}

fn main() {
    let data = Some(1);
    foo(data);
}
