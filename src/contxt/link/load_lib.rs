use libloading::{Library, Symbol};

pub struct SharedLibary {
    libary: Library,
}

impl SharedLibary {
    pub fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(
            Self {
                libary: unsafe { Library::new(path)? },
            }
        )
    }

    pub unsafe fn get_func_pointer<T>(&self, name: &str) -> Result<unsafe extern "system" fn() -> isize, Box<dyn std::error::Error>> {
        let func: Symbol<*const T> = self.libary.get(name.as_bytes().into())?;
        let raw = func.into_raw();
        let raw = raw.into_raw();
        let raw = raw.unwrap();

        Ok(raw)
    }
}