use std::mem;
use std::ptr;

use libc::{c_void, c_int};

#[cfg(not(windows))]
use libc::{PROT_WRITE, PROT_EXEC, MAP_PRIVATE, MAP_ANON, MAP_FAILED};
#[cfg(windows)]
use winapi::um::{memoryapi::{VirtualAlloc, VirtualFree}, winnt::{MEM_COMMIT, PAGE_EXECUTE_READWRITE}};

fn main() {
    // Machine code for:
    //   mov eax, 0
    //   ret
    let code: [u8; 6] = [0xb8, 0x05, 0x00, 0x00, 0x00, 0xc3];

    // Allocate writable/executable memory.
    let mem = unsafe { alloc_executable_memory(code.len()) };
    if mem.is_null() {
        println!("Error allocating memory");
        return;
    }

    unsafe {
        ptr::copy_nonoverlapping(code.as_ptr(), mem as *mut u8, code.len());
    }

    // Convert the memory to a function pointer.
    let func: unsafe extern "C" fn() -> i32 = unsafe { mem::transmute(mem) };

    println!("{}", unsafe { func() });

    // Deallocate memory.
    unsafe {
        dealloc_executable_memory(mem, code.len());
    }
}

#[cfg(not(windows))]
unsafe fn alloc_executable_memory(size: usize) -> *mut c_void {
    let ptr = libc::mmap(ptr::null_mut(), size, PROT_WRITE | PROT_EXEC, MAP_PRIVATE | MAP_ANON, -1, 0);
    if ptr == MAP_FAILED {
        ptr::null_mut()
    } else {
        ptr
    }
}

#[cfg(windows)]
unsafe fn alloc_executable_memory(size: usize) -> *mut c_void {

    VirtualAlloc(ptr::null_mut(), size, MEM_COMMIT, PAGE_EXECUTE_READWRITE) as *mut c_void
}

#[cfg(not(windows))]
unsafe fn dealloc_executable_memory(ptr: *mut c_void, size: usize) {
    libc::munmap(ptr, size);
}

#[cfg(windows)]
unsafe fn dealloc_executable_memory(ptr: *mut c_void, _size: usize) {
    VirtualFree(ptr as *mut winapi::ctypes::c_void, 0, winapi::um::winnt::MEM_RELEASE);
}
