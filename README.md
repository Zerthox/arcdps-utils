# GW2 ArcDPS Utils
Utilities for [Rust](https://www.rust-lang.org/learn/get-started)-based [ArcDPS](https://deltaconnected.com/arcdps) plugins.

## Usage
```toml
[dependencies]
arc_util = { git = "https://github.com/zerthox/gw2-arcdps-utils", tag = "0.4.0" }
```

Documentation can be found here: [zerthox.github.io/gw2-arcdps-utils/arc_util](https://zerthox.github.io/gw2-arcdps-utils/arc_util/)

## Optional Features
| Flag | Description |
| --- | --- |
| serde | Adds [serde](https://serde.rs/) Serialize & Deserialize to `game` structs.
| settings | Includes settings utilitites in `settings`.
| log | Includes a Log component in `components`.
