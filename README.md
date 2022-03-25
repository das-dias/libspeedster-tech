# Speedster - Tech : Semiconductor Technology parser for Speedster
Speedster is a Rust language project separated into multiple crates to allow for modular development, debugging, testing and launch. Speedster-Tech one of the crates, allowing users to immport a semiconductor technology into a dedicated database. This crate also allows for the serde-serialization and de-serialization of the technology data structure, in order to export it into JSON and YAML.
---
---
Version: 0.1.0
---
---
### Importing a new technology
```Rust
mod speedster::tech;

let new_tech: TechnologyNode = tech::read(technology_file);

```
