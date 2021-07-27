# Improve Rust skills by making an ECS library

## Videos

- [ ] Introduction
  - basing the ECS library on HECS
- [ ] Deriving Debug forces everything to derive Debug
- [ ] Cargo expand to see what a macro is expanding to
- [ ] Documentation and doctests (use statements to have access to the code)
  - [ ] cargo doc
- [ ] What is ECS and why would a game developer want to use one?
  - Memory management to minimizse cache misses
  - Forces a data oriented design and programming style
- [ ] Introduction (why build our own ECS?)
- [ ] dynamic dispatch
  - [ ] Using the Any trait
  - [ ] Using custom traits
  - [ ] 'static trait (we know how much space it takes in memory)
- [ ] Generics
  - [ ] What are generics in Rust as far as we are concerned at a high level?
  - [ ] How to restrict the types
- [ ] Options
- [ ] Programming Patterns
  - [ ] Builder Pattern
- [ ] dot notation (call a function, and then . to call something on the functions return value)
  - .something will auto dereference whatever it is attached to
- [ ] data structures
  - [ ] HashMap
  - [ ] enum
    - [ ] compare together
      - [ ] matches
      - [ ] PartialEq derive macro
- [ ] Types
  - [ ] TypeId
  - [ ] usize vs u32 vs u8...
- [ ] Module system
- [ ]

# Alternate architectures to consider

- LordMZTE: https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=86dc4ac7e3592af412655d05e38df082
