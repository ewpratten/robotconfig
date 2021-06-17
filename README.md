# robotconfig
[![Crates.io](https://img.shields.io/crates/v/robotconfig)](https://crates.io/crates/robotconfig) 
[![Docs.rs](https://docs.rs/robotconfig/badge.svg)](https://docs.rs/robotconfig) 
[![Build](https://github.com/Ewpratten/robotconfig/actions/workflows/build.yml/badge.svg)](https://github.com/Ewpratten/robotconfig/actions/workflows/build.yml)
[![Clippy](https://github.com/Ewpratten/robotconfig/actions/workflows/clippy.yml/badge.svg)](https://github.com/Ewpratten/robotconfig/actions/workflows/clippy.yml)
[![Audit](https://github.com/Ewpratten/robotconfig/actions/workflows/audit.yml/badge.svg)](https://github.com/Ewpratten/robotconfig/actions/workflows/audit.yml)

This crate contains all the needed definitions to parse Lib5K [RobotConfig](https://github.com/ewpratten/lib5k/tree/master/lib5k/src/main/java/io/github/frc5024/lib5k/config) files in Rust. The rust definitions are much smaller than the Java counterparts, since in Rust, we have nice libraries like `serde`, which do most of the work automatically.

This project is part of an effort to preserve my work on [frc5024/lib5k](https://github.com/frc5024/lib5k) in a way that I can use the code without being bound to the team's rules, or the restrictive FRC licencing.

## Installation

This crate can be installed via `cargo` with:

```sh
cargo install robotconfig
```
