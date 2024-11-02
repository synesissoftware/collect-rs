# collect-rs <!-- omit in toc -->

Special and custom Collections and Containers for Rust

[![Crates.io](https://img.shields.io/crates/v/collect-rs.svg)](https://crates.io/crates/collect-rs)

## Introduction


## Table of Contents <!-- omit in toc -->

- [Introduction](#introduction)
- [Installation](#installation)
- [Components](#components)

## Installation

Reference in **Cargo.toml** in the usual way:

```toml
collect-rs = { version = "0.1" }
```


## Components

### Constants

No public constants are defined at this time.


### Enumerations

No public enumerations are defined at this time.


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

```Rust
	let mut fm = FrequencyMap::default();

	fm.push("cat");
	fm.push("dog");
	fm.push("dog");

	assert_eq!(1, fm.get("cat"));
	assert_eq!(2, fm.get("dog"));
	assert_eq!(0, fm.get("mouse"));
```

### `UnicodePointMap`

```Rust
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

Crates upon which **collect-rs** depend:

* [**base-traits**](https://github.com/synesissoftware/base-traits);


##### Dev Dependencies

Crates upon which **collect-rs** depend:

* [**criterion**](https://github.com/bheisler/criterion.rs);
* [**test_help-rs**](https://github.com/synesissoftware/test_help-rs);


### Related projects

None at this time.


### License

**collect-rs** is released under the 3-clause BSD license. See [LICENSE](./LICENSE) for details.


<!-- ########################### end of file ########################### -->
