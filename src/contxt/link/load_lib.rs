use std::fmt::Display;

#[cfg(target_os = "windows")]
mod platform {
    use std::ffi::CString;
    use winapi::um::libloaderapi::{LoadLibraryA, GetProcAddress, FreeLibrary};

    pub fn load_library(library_path: &str) -> Result<*mut std::ffi::c_void, Box<dyn std::error::Error>> {
        let library_path_c = CString::new(library_path)?;
        Ok( unsafe { LoadLibraryA(library_path_c.as_ptr()) as *mut std::ffi::c_void} )
    }

    pub fn resolve_symbol(library: *mut std::ffi::c_void, symbol_name: &str) -> Result<*mut std::ffi::c_void, Box<dyn std::error::Error>> {
        let symbol_name_c = CString::new(symbol_name)?;
        Ok( unsafe { GetProcAddress(library as *mut winapi::shared::minwindef::HINSTANCE__, symbol_name_c.as_ptr())  as *mut std::ffi::c_void } )
    }

    pub fn unload_library(library: *mut std::ffi::c_void) {
        unsafe { FreeLibrary(library as *mut winapi::shared::minwindef::HINSTANCE__) };
    }
}

#[cfg(target_os = "linux")]
mod platform {
    use std::ffi::CString;
    use libc::{dlopen, dlsym, dlclose};

    pub fn load_library(library_path: &str) -> Result<*mut std::ffi::c_void, Box<dyn std::error::Error>> {
        let library_path_c = CString::new(library_path)?;
        Ok ( unsafe { dlopen(library_path_c.as_ptr(), libc::RTLD_NOW) } )
    }

    pub fn resolve_symbol(library: *mut std::ffi::c_void, symbol_name: &str) -> Result<*mut std::ffi::c_void, Box<dyn std::error::Error>> {
        let symbol_name_c = CString::new(symbol_name)?;
        Ok ( unsafe { dlsym(library, symbol_name_c.as_ptr()) } )
    }

    pub fn unload_library(library: *mut std::ffi::c_void) {
        unsafe { dlclose(library) }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum LibLoadError {
    ExternalFailed,
    FailedAsmFuncLoad,
}

impl Display for LibLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            LibLoadError::ExternalFailed => "failed to load external libary",
            LibLoadError::FailedAsmFuncLoad => "failed to load function",
        };

        write!(f, "{}", msg)
    }
}

impl std::error::Error for LibLoadError {}

pub struct SharedLibary {
    libary: *mut std::ffi::c_void,
}

impl SharedLibary {
    pub fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let loaded = platform::load_library(path)?;

        if loaded.is_null() {
            return Err( Box::from(LibLoadError::ExternalFailed) );
        }

        Ok(
            Self {
                libary: loaded,
            }
        )
    }

    pub unsafe fn get_func_pointer<T: Copy>(&self, name: &str) -> Result<T, Box<dyn std::error::Error>> {
        let func = platform::resolve_symbol(self.libary, name)?;

        if func.is_null() {
            return Err( Box::from(LibLoadError::FailedAsmFuncLoad) );
        }

        let func: *const T = unsafe { std::mem::transmute(func) };

        Ok(*func)
    }

    pub fn free(&self) {
        platform::unload_library(self.libary);
    }
}