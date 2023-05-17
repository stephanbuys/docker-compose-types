Docker Compose Types
===========

[![crates.io](https://img.shields.io/crates/v/docker-compose-types.svg)](https://crates.io/crates/docker-compose-types)

Contributions are very welcome, the idea behind this crate is to allow for safe serialization of docker-compose files with as little room for error as possible.

## Example Usage

### Parsing a docker-compose file

```rust
use docker_compose_types::Compose;

fn main() {
    let file_payload =
        std::fs::read_to_string("tests/fixtures/v3-full/docker-compose.yml").unwrap();
    let compose_content = match serde_yaml::from_str::<Compose>(&file_payload) {
        Ok(c) => c,
        Err(e) => panic!("Failed to parse docker-compose file: {}", e),
    };
    println!("Parsed docker-compose file: {:#?}", compose_content);
}
```

### Creating a docker-compose file from the crate's types
```rust
use docker_compose_types::{Compose, Service, Services, SingleService};
use serde_yaml;

fn main() {
    let compose_content = Compose {
        version: Some("3.8".to_string()),
        services: {
            let mut services = indexmap::IndexMap::new();
            services.insert(
                "web".to_string(),
                Some(Service {
                    image: Some("nginx:latest".to_string()),
                    ..Default::default()
                }),
            );
            Some(Services(services))
        },
        ..Default::default()
    };

    let target_file = std::path::Path::new("docker-compose.yml");
    // serialize to string
    let serialized = match serde_yaml::to_string(&compose_content) {
        Ok(s) => s,
        Err(e) => panic!("Failed to serialize docker-compose file: {}", e),
    };
    // serialize to file
    std::fs::write(target_file, serialized).unwrap();
}
```




#### License

<sup>
Licensed under either of
<a href="LICENSE-APACHE">Apache License, Version 2.0</a>
or
<a href="LICENSE-MIT">MIT license</a>
at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
