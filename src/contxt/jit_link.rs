use std::collections::HashMap;

use super::jit::JitFunction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Link {
    pub from: String,
    pub to: String,
    pub at: usize,
    pub size: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JitLinker {
    pub labels: HashMap<String, Vec<u8>>,
    pub funcs: HashMap<String, (Vec<u8>, bool)>,
    pub relocs: Vec<Link>,
}

impl JitLinker {
    pub fn new() -> Self {
        Self {
            labels: HashMap::new(),
            funcs: HashMap::new(),
            relocs: vec![],
        }
    }

    pub fn link(&mut self, base: usize) -> Vec<u8> {
        let mut ret: Vec<u8> = vec![];
        let mut ret_hash: HashMap<&String, Vec<u8>> = HashMap::new();

        let mut funcs_p: HashMap<&String, (&Vec<u8>, usize)> = HashMap::new();

        let cloned = self.funcs.clone();

        for func in &cloned {
            if func.1.1 { // func is first
                let code = &func.1.0;
    
                for byte in code {
                    ret.push(*byte);
                }
    
                ret_hash.insert(&func.0, code.to_vec());
    
                let offset = ret.len() as isize - code.len() as isize;
                let offset = offset as usize + base;
    
                funcs_p.insert(&func.0, (&code, offset));
            }
        }

        for func in &cloned {
            if func.1.1 { continue; } // func allready added
            let code = &func.1.0;

            for byte in code {
                ret.push(*byte);
            }

            ret_hash.insert(func.0, code.to_vec());

            let offset = ret.len() as isize - code.len() as isize;
            let offset = offset as usize + base;

            funcs_p.insert(func.0, (&code, offset));
        }

        ret.push(0xC3); // ret so code | labels are split

        for label in &self.labels {
            for byte in label.1 {
                ret.push(*byte);
            }

            ret_hash.insert(&label.0, label.1.to_vec());

            let offset = ret.len() as isize - label.1.len() as isize;
            let offset = offset as usize + base;

            funcs_p.insert(&label.0, (&label.1, offset));
        }

        for link in self.relocs.iter() {
            let offset = funcs_p.get(&link.from).unwrap().1;
            let pos = funcs_p.get(&link.to).unwrap().1 as isize;
            let pos = pos - link.size as isize - 1;

            let at = offset + link.at;
            
            for b in 0..(link.size - 1) {
                ret[((at + b) - base) as usize] = pos.to_le_bytes()[b];
            }
        }

        ret
    }

    pub unsafe fn engine<T>(&mut self) -> JitFunction<T> {

        let func: JitFunction<T> = JitFunction::new(self.link(0));

        /*
        Let's don't change the memory adresses to exact
        because it will get loaded
        */

        func
    }
}
