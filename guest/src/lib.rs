extern "C" {
    fn ocall(thread: i32, nth: i32, p: i32);
}

#[inline(never)]
fn use_more_stack(n: i32) -> i32 {
    let mut bug_buffer = [n; 32];
    unsafe {
        ocall(n, 2, bug_buffer.as_mut_ptr() as i32);
    }
    bug_buffer[0]
}

#[no_mangle]
extern "C" fn entry(thread: i32) {
    let mut b = [thread; 1];
    unsafe {
        ocall(thread, 1, b.as_mut_ptr() as i32);
    }
    let b = use_more_stack(thread);
    unsafe {
        ocall(thread, 3, b);
    }
}
