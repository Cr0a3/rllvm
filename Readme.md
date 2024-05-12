# RLLVM
![Languages](https://img.shields.io/github/languages/top/Toni-Graphics/rllvm?logo=rust)
![GitHub Repo stars](https://img.shields.io/github/stars/Toni-Graphics/rllvm?style=flat)
![GitHub License](https://img.shields.io/github/license/Toni-Graphics/rllvm)
![Dynamic TOML Badge](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fraw.githubusercontent.com%2FToni-Graphics%2Frllvm%2Fmain%2FCargo.toml&query=%24.package.version&label=version)



LLVM alternativ

# ToDo
Here is a bit of ToDo for my libary:
## v0.1.2
 - [x] Starting high level ir struct
    - [x] Use traits `impl Compiler for Ir::Add<Var, Int>` so i can overload the enum variants
    - [x] Make it compilable
    - [ ] Implement `mov`, `add`, `sub`, `mul`, `div` | `ints`, `floats`
  - [ ] Starting high level ir builder

##  v0.1.3
 - [ ] Implement `args` to the high level ir
 - [x] Add option (in `context`) to compile to object file
 - [ ] Naming convention
    - [x] generate
    - [ ] parse