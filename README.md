# collect-rs <!-- omit in toc -->

Special and custom Collections and Containers for Rust

![Language](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)
[![License](https://img.shields.io/badge/License-BSD_3--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)
[![Crates.io](https://img.shields.io/crates/v/collect-rs.svg)](https://crates.io/crates/collect-rs)
[![GitHub release](https://img.shields.io/github/v/release/synesissoftware/collect-rs.svg)](https://github.com/synesissoftware/collect-rs/releases/latest)
![MSRV](https://img.shields.io/badge/MSRV-1.74-lightgrey)
[![CI](https://github.com/synesissoftware/collect-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/synesissoftware/collect-rs/actions/workflows/ci.yml)
[![docs.rs](https://docs.rs/collect-rs/badge.svg)](https://docs.rs/collect-rs)


## Table of Contents <!-- omit in toc -->

- [Introduction](#introduction)
- [Installation](#installation)
- [Components](#components)
  - [Constants](#constants)
  - [Enumerations](#enumerations)
  - [Features](#features)
  - [Functions](#functions)
  - [Macros](#macros)
  - [Structures](#structures)
  - [Traits](#traits)
- [Examples](#examples)
- [Project Information](#project-information)
  - [Where to get help](#where-to-get-help)
  - [Contribution guidelines](#contribution-guidelines)
  - [Dependencies](#dependencies)
    - [Efferent (fan-out)](#efferent-fan-out)
      - [Runtime Dependencies](#runtime-dependencies)
      - [Build Dependencies](#build-dependencies)
      - [Development Dependencies](#development-dependencies)
    - [Afferent (fan-in)](#afferent-fan-in)
  - [Related projects](#related-projects)
  - [License](#license)


## Introduction

**collect-rs** provides specialized collection types for counting and
measuring unique values, including generic keys and Unicode code points.

## Installation

Reference in **Cargo.toml** in the usual way:

```toml
collect-rs = { version = "0.2.1" }
```


## Components

### Constants

No public constants are defined at this time.


### Enumerations

No public enumerations are defined at this time.


### Features

The following placeholder features are defined in **Cargo.toml**:

* `"_NEVER_TO_BE_ENABLED"` — a feature that must never be specified;
* `"null-feature"` — a feature with no effect, useful for simplifying driver
  scripts;


### Functions

No public functions are defined at this time.


### Macros

No public macros are defined at this time.


### Structures

* `FrequencyMap<K>` - a container that measures the frequencies of the unique elements it contains;
* `UnicodePointMap` - a specialised container that measures the frequencies of Unicode code-points;


### Traits

No public traits are defined at this time.


## Examples

The following are terse examples of some of the components provided in the crate. See the **examples** directory for more.

### `FrequencyMap`

```rust
	let mut fm = FrequencyMap::default();

	fm.push("cat");
	fm.push("dog");
	fm.push("dog");

	assert_eq!(1, fm.get("cat"));
	assert_eq!(2, fm.get("dog"));
	assert_eq!(0, fm.get("mouse"));
```

### `UnicodePointMap`

```rust
	let upm = UnicodePointMap::from_iter("The quick brown fox jumps over the lazy dog".chars().into_iter());

	assert_eq!(1, upm['a']);
	assert_eq!(1, upm['b']);
	assert_eq!(1, upm['c']);
	assert_eq!(1, upm['d']);
	assert_eq!(3, upm['e']);
	assert_eq!(1, upm['f']);
	assert_eq!(1, upm['g']);
	assert_eq!(2, upm['h']);
	assert_eq!(1, upm['i']);
	assert_eq!(1, upm['j']);
	assert_eq!(1, upm['k']);
	assert_eq!(1, upm['l']);
	assert_eq!(1, upm['m']);
	assert_eq!(1, upm['n']);
	assert_eq!(4, upm['o']);
	assert_eq!(1, upm['p']);
	assert_eq!(1, upm['q']);
	assert_eq!(2, upm['r']);
	assert_eq!(1, upm['s']);
	assert_eq!(1, upm['t']);
	assert_eq!(2, upm['u']);
	assert_eq!(1, upm['v']);
	assert_eq!(1, upm['w']);
	assert_eq!(1, upm['x']);
	assert_eq!(1, upm['y']);
	assert_eq!(1, upm['z']);
	assert_eq!(8, upm[' ']);
	assert_eq!(1, upm['T']);

	assert_eq!(0, upm['0']);
	assert_eq!(0, upm['-']);
	assert_eq!(0, upm['_']);
	assert_eq!(0, upm['.']);
	assert_eq!(0, upm[',']);
```


## Project Information

### Where to get help

[GitHub Page](https://github.com/synesissoftware/collect-rs "GitHub Page")


### Contribution guidelines

Defect reports, feature requests, and pull requests are welcome on https://github.com/synesissoftware/collect-rs.


### Dependencies

#### Efferent (fan-out)

Libraries upon which **collect-rs** depends:

##### Runtime Dependencies

* [**base-traits**](https://github.com/synesissoftware/base-traits);


##### Build Dependencies

No build dependencies are required.


##### Development Dependencies

* [**criterion**](https://github.com/bheisler/criterion.rs);
* [**test_help-rs**](https://github.com/synesissoftware/test_help-rs);


#### Afferent (fan-in)

Projects that depend on **collect-rs**:

* [**shwild.Rust**](https://github.com/synesissoftware/shwild.Rust);


### Related projects

None at this time.


### License

**collect-rs** is released under the 3-clause BSD license. See [LICENSE](./LICENSE) for details.


<!-- ########################### end of file ########################### -->
