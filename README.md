<header>
    <h1 align="center">lutgen-rs</h1>
    <p align="center">
        <a href="https://crates.io/crates/lutgen"><img alt="crate" src="https://img.shields.io/crates/v/lutgen?style=for-the-badge" /></a>
        <a href="./LICENSE"><img alt="license" src="https://img.shields.io/badge/license-MIT-blue?style=for-the-badge" /></a>
        <a href="https://github.com/ozwaldorf/lutgen-rs/actions/workflows/rust.yml"><img alt="ci" src="https://img.shields.io/github/actions/workflow/status/ozwaldorf/lutgen-rs/rust.yml?label=CI&style=for-the-badge" /></a>
        <a href="https://github.com/ozwaldorf/lutgen-rs/actions/workflows/publish.yml"><img alt="publish" src="https://img.shields.io/github/actions/workflow/status/ozwaldorf/lutgen-rs/publish.yml?label=Publish&style=for-the-badge" /></a>
    </p>
    <p align="center">
        A blazingly fast interpolated LUT generator using gaussian distribution for arbitrary and popular color palettes.
    </p>
</header>

---

## Usage

### CLI

```bash
cargo install lutgen
```

```text
Usage: lutgen [OPTIONS] [CUSTOM_COLORS]...

Arguments:
  [CUSTOM_COLORS]...  List of custom colors to add to the palette. If `-p` is not used to specify a base palette, at least 1 color is required

Options:
  -p <PALETTE>                   Add colors from a predefined base palette. Use `lutgen -p` to view all options
  -a <ALGORITHM>                 Algorithm to generate the LUT with [default: v1] [possible values: v1, v0]
  -o, --output <OUTPUT>          Path to write the generated file to. Defaults to the current dir with some parameters (ex: `./hald_clut_v1_4_20_512.png`)
  -l, --level <LEVEL>            HaldCLUT color depth. 8 bit = 512x512 image [default: 8]
  -m, --mean <MEAN>              Mean for the gaussian distribution [default: 4]
  -s, --std-dev <STD_DEV>        Standard deviation for the gaussian distribution [default: 20]
  -i, --iterations <ITERATIONS>  Number of iterations to average together [default: 512]
  -h, --help                     Print help (see more with '--help')
  -V, --version                  Print version
```

### Library

> By default, the `bin` feature and dependencies are enabled.
> When used as a library, it's recommended to use `default-features = false` to minimalize the dependency tree and build time.

Simple usage:

```rust
use exoquant::{
    Color,
    SimpleColorSpace,
};
use lutgen::{
    interpolated_remap::{
        GaussianV0Params, GaussianV0Remapper, GaussianV1Params, GaussianV1Remapper
    },
    generate_lut,
};

// Setup the palette and colorspace for nearest neighbor lookups.
let palette = vec![
    Color::new(255, 0, 0, 255),
    Color::new(0, 255, 0, 255),
    Color::new(0, 0, 255, 255),
];
let colorspace = SimpleColorSpace::default();

// Generate a lut using the slower v0 algorithm
let params = GaussianV0Params {
    mean: 4.0,
    std_dev: 20.0,
    iterations: 512,
    seed: 80085,
    colorspace: SimpleColorSpace::default(),
};
let output = generate_lut::<GaussianV0Remapper<_>>(8, &palette, params);
output.save("v0_hald_8.png").unwrap();
    
// Generate a lut using the faster v1 algorithm
let params = GaussianV1Params {
    mean: 4.0,
    std_dev: 20.0,
    iterations: 512,
    seed: 80085,
    colorspace: SimpleColorSpace::default(),
};
let output = generate_lut::<GaussianV1Remapper<_>>(8, &palette, params);
output.save("v1_hald_8.png").unwrap();
```

Advanced usage:

```rust
use exoquant::Color;

// Generate the base identity
let identity = lutgen::identity::generate(8);

// Setup the palette and parameters
let palette = vec![
    Color::new(255, 0, 0, 255),
    Color::new(0, 255, 0, 255),
    Color::new(0, 0, 255, 255),
];
let mean = 4.0;
let std_dev = 20.0;
let iters = 512;
let seed = 0;

// Remap the identity
```

