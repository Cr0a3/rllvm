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

    pub unsafe fn get_func_pointer<T: Copy>(&self, name: &str) -> Result<T, Box<dyn std::error::Error>> {
        let func: Symbol<*const T> = self.libary.get(name.as_bytes().into())?;
        let casted: *const T = func.cast();

        Ok(*casted)
    }
}