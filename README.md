# GW2 ArcDPS Utils
Utilities for [Rust](https://www.rust-lang.org/learn/get-started)-based [ArcDPS](https://deltaconnected.com/arcdps) plugins.

## Usage
```toml
[dependencies]
arc_util = { git = "https://github.com/zerthox/gw2-arcdps-utils", branch = "master" }
```

Documentation can be found here: [zerthox.github.io/gw2-arcdps-utils/arc_util](https://zerthox.github.io/gw2-arcdps-utils/arc_util/)

## Optional Features
| | |
| --- | --- |
| serde | Adds [serde](https://serde.rs/) Serialize & Deserialize to `game` structs.
| log | Includes a Log component in `components`.
