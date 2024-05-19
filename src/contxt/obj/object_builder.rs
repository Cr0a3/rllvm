use std::{collections::HashMap, fs::OpenOptions};

use object::{
    write::{Relocation, SectionId, StandardSection, Symbol, SymbolId, SymbolSection}, Architecture, RelocationEncoding, RelocationFlags, RelocationKind, SymbolFlags, SymbolKind, SymbolScope
};

use super::{Decl, Link, ObjectError, Scope};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Enum which specifies the binary format
///
/// E.g: Coff for Windows
pub enum BinFormat {
    Elf,
    Coff,
    Macho,
    // Exe // No support for exe because sus errors
}

impl BinFormat {
    /// Function which returns the native binary format
    ///
    /// For any unknown os it returns elf
    pub fn host() -> BinFormat {
        if cfg!(target_os = "windows") {
            BinFormat::Coff
        } else if cfg!(target_os = "macos") {
            BinFormat::Macho
        } else {
            BinFormat::Elf
        }
    }
}

#[derive(Debug, Clone)]
/// A struct for building object files
pub struct ObjectBuilder {
    decls: Vec<(String, Decl)>,
    sym: HashMap<String, Vec<u8>>,
    links: Vec<Link>,

    outpath: String,
}

impl ObjectBuilder {
    //// Returns empty instance of self
    pub fn new(path: &str) -> Self {
        Self {
            decls: vec![],
            sym: HashMap::new(),
            links: vec![],

            outpath: path.into(),
        }
    }

    /// Adds a list of decls
    pub fn decls(&mut self, decls: Vec<(&str, Decl)>) {
        for decl in decls {
            let decl = (decl.0.into(), decl.1);
            self.decls.push(decl);
        }
    }

    /// Adds a decl
    pub fn add_decl(&mut self, name: &str, decl: Decl) {
        self.decls.push((name.into(), decl));
    }

    /// Defines a symbol
    pub fn define(&mut self, sym: &str, data: Vec<u8>) {
        self.sym.insert(sym.into(), data);
    }

    /// Adds an link to the object file
    pub fn link(&mut self, link: Link) {
        self.links.push(link);
    }

    /// Writes all internaly saved symbols etc. to a object file
    ///
    /// Args:
    ///  * `format`   - specifes the binary format of the object file
    ///  * `arch`     - specifes the architecture of the object file
    ///  * `endian`   - specifes the endian of the object file
    pub fn write(
        &mut self,
        format: BinFormat,
        arch: Architecture,
        endian: object::Endianness,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(self.outpath.clone().to_owned())?;

        let obj_format = match format {
            BinFormat::Elf => object::BinaryFormat::Elf,
            BinFormat::Coff => object::BinaryFormat::Coff,
            BinFormat::Macho => object::BinaryFormat::MachO,
            //BinFormat::Exe => object::BinaryFormat::Pe,
        };

        let mut obj = object::write::Object::new(obj_format, arch, endian);

        obj.add_file_symbol(self.outpath.to_owned().into_bytes());

        let mut ids: HashMap<String, SymbolId> = HashMap::new();
        let mut funcs: HashMap<String, ((SectionId, u64), SymbolId)> = HashMap::new();

        for decl in self.decls.iter() {
            let name = &decl.0;
            let decl = &decl.1;

            // get type
            match decl {
                Decl::RData(s) => match s {
                    Scope::Import => {
                        ids.insert(
                            name.to_string(),
                            obj.add_symbol(Symbol {
                                name: name.as_bytes().into(),
                                value: 0,
                                size: 0,
                                kind: SymbolKind::Data,
                                scope: SymbolScope::Dynamic,
                                weak: false,
                                section: SymbolSection::Undefined,
                                flags: SymbolFlags::None,
                            }),
                        );
                    }
                    _ => {
                        let dat_opt = self.sym.get(&name.clone());

                        if dat_opt.is_none() {
                            return Err(Box::from(ObjectError::DeclWithoutSymbol));
                        }

                        let data = dat_opt.unwrap();

                        let scope;
                        if s.to_owned() == Scope::Export {
                            scope = SymbolScope::Linkage
                        } else {
                            scope = SymbolScope::Compilation
                        }

                        let (section, offset) = obj.add_subsection(
                            StandardSection::ReadOnlyData,
                            name.as_bytes().into(),
                            data,
                            16,
                        );
                        let symbol = obj.add_symbol(Symbol {
                            name: name.as_bytes().into(),
                            value: offset,
                            size: data.len() as u64,
                            kind: SymbolKind::Data,
                            scope: scope,
                            weak: false,
                            section: SymbolSection::Section(section),
                            flags: SymbolFlags::None,
                        });

                        funcs.insert(name.into(), ((section, offset), symbol));
                    }
                },

                
                Decl::UData(s) => match s {
                    Scope::Import => {
                        ids.insert(
                            name.to_string(),
                            obj.add_symbol(Symbol {
                                name: name.as_bytes().into(),
                                value: 0,
                                size: 0,
                                kind: SymbolKind::Data,
                                scope: SymbolScope::Dynamic,
                                weak: false,
                                section: SymbolSection::Undefined,
                                flags: SymbolFlags::None,
                            }),
                        );
                    }
                    _ => {
                        let dat_opt = self.sym.get(&name.clone());

                        if dat_opt.is_none() {
                            return Err(Box::from(ObjectError::DeclWithoutSymbol));
                        }

                        let data = dat_opt.unwrap();

                        let scope;
                        if s.to_owned() == Scope::Export {
                            scope = SymbolScope::Linkage
                        } else {
                            scope = SymbolScope::Compilation
                        }

                        let (section, offset) = obj.add_subsection(
                            StandardSection::UninitializedData,
                            name.as_bytes().into(),
                            data,
                            16,
                        );
                        let symbol = obj.add_symbol(Symbol {
                            name: name.as_bytes().into(),
                            value: offset,
                            size: data.len() as u64,
                            kind: SymbolKind::Data,
                            scope: scope,
                            weak: false,
                            section: SymbolSection::Section(section),
                            flags: SymbolFlags::None,
                        });

                        funcs.insert(name.into(), ((section, offset), symbol));
                    }
                },

                Decl::Function(s) => match s {
                    Scope::Import => {
                        ids.insert(
                            name.to_string(),
                            obj.add_symbol(Symbol {
                                name: name.as_bytes().into(),
                                value: 0,
                                size: 0,
                                kind: SymbolKind::Text,
                                scope: SymbolScope::Dynamic,
                                weak: false,
                                section: SymbolSection::Undefined,
                                flags: SymbolFlags::None,
                            }),
                        );
                    }
                    _ => {
                        let dat_opt = self.sym.get(&name.clone());

                        if dat_opt.is_none() {
                            return Err(Box::from(ObjectError::DeclWithoutSymbol));
                        }

                        let scope;
                        if s.to_owned() == Scope::Export {
                            scope = SymbolScope::Linkage
                        } else {
                            scope = SymbolScope::Compilation
                        }

                        let data = dat_opt.unwrap();

                        let (section, offset) = obj.add_subsection(
                            StandardSection::Text,
                            name.as_bytes().into(),
                            data,
                            16,
                        );
                        let symbol = obj.add_symbol(Symbol {
                            name: name.as_bytes().into(),
                            value: offset,
                            size: data.len() as u64,
                            kind: SymbolKind::Text,
                            scope: scope,
                            weak: false,
                            section: SymbolSection::Section(section),
                            flags: SymbolFlags::None,
                        });

                        funcs.insert(name.into(), ((section, offset), symbol));
                    }
                },
            }
        }

        for link in self.links.iter() {
            let link = link.to_owned();

            let func_opt = funcs.get(&link.from);
            if func_opt.is_none() {
                return Err(Box::from(ObjectError::UnknownFunction(
                    link.from.to_owned(),
                )));
            }
            let func = func_opt.unwrap();

            let id = func.0 .0;
            let off = func.0 .1;

            let sym;

            if funcs.contains_key(&link.to) {
                sym = Some(funcs.get(&link.to).unwrap().1);
            } else if ids.contains_key(&link.to) {
                sym = Some(ids.get(&link.to).unwrap().to_owned());
            } else {
                return Err(Box::from(ObjectError::UnknownTargetSymbol(
                    link.to.to_owned(),
                )));
            }

            obj.add_relocation(
                id,
                Relocation {
                    offset: off + link.at as u64,
                    symbol: sym.unwrap(),
                    addend: -4,
                    flags: RelocationFlags::Generic {
                        kind: RelocationKind::PltRelative,
                        encoding: RelocationEncoding::X86Branch,
                        size: 32,
                    },
                },
            )?;
        }

        obj.write_stream(file)?;

        Ok(())
    }
}