use std::mem;
use std::ptr;

use libc::c_void;

#[cfg(not(windows))]
use libc::{c_int, MAP_ANON, MAP_FAILED, MAP_PRIVATE, PROT_EXEC, PROT_WRITE};
#[cfg(windows)]
use winapi::um::{
    memoryapi::{VirtualAlloc, VirtualFree},
    winnt::{MEM_COMMIT, PAGE_EXECUTE_READWRITE},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct JitFunction<T> {
    pub code: Vec<u8>,
    tmp: Vec<T>,
}

impl<T> JitFunction<T> {
    pub fn new(code: Vec<u8>) -> Self {
        Self {
            code: code,
            tmp: vec![],
        }
    }

    /// use code:
    /// ```
    /// func.req();
    /// let inner = mem::transmute(
    ///     func.req();
    /// );
    /// ```
    unsafe fn req(&mut self) -> *mut c_void {
        let mem = alloc_executable_memory(self.code.len());
        if mem.is_null() {
            println!("Error allocating memory");
            return mem;
        }

        ptr::copy_nonoverlapping(self.code.as_ptr(), mem as *mut u8, self.code.len());

        mem
    }

    unsafe fn free(&mut self, mem: *mut c_void) {
        dealloc_executable_memory(mem, self.code.len());
    }
}

#[cfg(not(windows))]
unsafe fn alloc_executable_memory(size: usize) -> *mut c_void {
    let ptr = libc::mmap(
        ptr::null_mut(),
        size,
        PROT_WRITE | PROT_EXEC,
        MAP_PRIVATE | MAP_ANON,
        -1,
        0,
    );
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
    VirtualFree(
        ptr as *mut winapi::ctypes::c_void,
        0,
        winapi::um::winnt::MEM_RELEASE,
    );
}

macro_rules! impl_unsafe_fn {
    (@recurse $first:ident $( , $rest:ident )*) => {
        impl_unsafe_fn!($( $rest ),*);
    };

    (@recurse) => {};

    ($( $param:ident ),*) => {
        impl<Output, $( $param ),*> JitFunction<unsafe extern "C" fn($( $param ),*) -> Output> {
            /// Calls function
            #[allow(non_snake_case)]
            #[inline(always)]
            pub unsafe fn call(&mut self, $( $param: $param ),*) -> Output {
                let mem = self.req();
                let inner: unsafe extern "C" fn($( $param ),*) -> Output = mem::transmute(mem);
                let out = (inner)($( $param ),*);
                self.free(mem);

                out
            }
        }

        impl_unsafe_fn!(@recurse $( $param ),*);
    };
}

impl_unsafe_fn!(
    A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A20, A21,
    A22, A23, A24, A25, A26, A27, A28, A29, A30, A31, A32, A33, A34, A35, A36, A37, A38, A39, A40,
    A41, A42, A43, A44, A45, A46, A47, A48, A49, A50
);
