//@ dont-require-annotations: NOTE

#![feature(core_intrinsics)]
#![feature(const_heap)]
use std::intrinsics;

const FOO: i32 = foo(); //~ ERROR evaluation of constant value failed
const fn foo() -> i32 {
    unsafe {
        let _ = intrinsics::const_allocate(4, 3) as *mut i32; //~ NOTE inside `foo`
    }
    1
}

fn main() {}
