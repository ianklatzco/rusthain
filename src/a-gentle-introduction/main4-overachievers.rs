// this one is optional and only for those that want a challenge ;3c
// https://github.com/ianklatzco/learning/blob/master/rust/function_pointer/src/main.rs
// grab this repo, write some x86-64 / arm shellcode to set RAX to 69, 
// compile main4,
// and then get the assert to pass ^^

extern crate libc;

use std::io::Read;
use std::ptr::copy_nonoverlapping;
use libc::{mmap, mprotect, munmap, PROT_READ, PROT_WRITE, PROT_EXEC, MAP_PRIVATE, MAP_ANON};

fn main() {
    println!("Feed me shellcode!! (on stdin)");
    // TODO PR in a read from a hardcoded vec
    // Read from stdin into a Vec<u8>
    let mut buffer = Vec::new();
    std::io::stdin().read_to_end(&mut buffer).expect("Failed to read from stdin");
	dbg!(&buffer);

    // Map memory of the size of the buffer as writable.
    let ptr = unsafe {
        mmap(
            std::ptr::null_mut(),
            buffer.len(),
            PROT_READ | PROT_WRITE,
            MAP_PRIVATE | MAP_ANON,
            -1,
            0
        )
    };

    if ptr == libc::MAP_FAILED {
        panic!("Failed to map memory");
    }

    // Copy the buffer into the mapped memory.
    unsafe {
        copy_nonoverlapping(buffer.as_ptr(), ptr as *mut u8, buffer.len());
    }

    // Make the memory executable.
    if unsafe { mprotect(ptr, buffer.len(), PROT_READ | PROT_EXEC) } != 0 {
        unsafe { munmap(ptr, buffer.len()); }
        panic!("Failed to set memory as executable");
    }

    // Cast the pointer to a function and call it.
    let func: extern "C" fn() -> u64 = unsafe { std::mem::transmute(ptr) };



    // exec
    let retval: u64 = func();
	dbg!(&retval);
    assert_eq!(retval, 69);

    // Clean up.
    unsafe { munmap(ptr, buffer.len()); }

	println!("didn't panic!");
}

// TODO (PRs welcome) write a test like in the first 3 files
// #[cfg(test)]
// mod tests {

//     #[test]
//     fn test_main3() {
//         assert_neq!(

//         );
//     }

// }