# Rasp
A toy LISP interpreter in Rust.

# Compiling
`cargo build`

# Using
`rasp file1.rasp [ file2.rasp ... ]`

# Current status:
* [x] Parser
* [x] AST
* [x] Bytecode (tentatively complete)
* [x] Interpreter (tentatively complete)
* [ ] Builtin functions
    * [*] `&print` (needs vararg compatibility)
    * [ ] `+`
    * [x] `-`
    * [x] `*`
    * [ ] `/`
    * [ ] `&input`, or some sort of "readline" function. (Function name up for debate)
* [ ] Language features
    * [ ] Vararg compatibility
    * [ ] Variable definitions (`&def` or `&let`)
    * [ ] BigInt as IntLit value (this is included in Cargo.toml; it just needs to be implemented)
    * [ ] List types
* [ ] Misc features
    * [ ] Imports and modules (Still need to choose keywords)
    * [ ] Foreign function interface (`&extern` or something like that)
    * [ ] REPL
# License
GPLv2, chex it out in the LICENSE file
