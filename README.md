# ast_topology

[![Crates.IO](https://img.shields.io/crates/v/ast_topology.svg)](https://crates.rs/crates/ast_topology)
[![Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/ast_topology/)
[![Build Nightly](https://github.com/andrew-johnson-4/ast_topology/workflows/BuildNightly/badge.svg)](https://github.com/andrew-johnson-4/ast_topology)
[![Build](https://github.com/andrew-johnson-4/ast_topology/workflows/Build/badge.svg)](https://github.com/andrew-johnson-4/ast_topology)
[![Donate using Liberapay](https://liberapay.com/assets/widgets/donate.svg)](https://liberapay.com/andrew-johnson-4/donate)

Differentiable programming for Rust

This package is no more than sugar for existing Rust autograd libraries.

```rust
autograd!{
   let x; let y;
   let z = 2.*x*x + 3.*y + 1.;

   assert_eq!(grad (dz/dy), 3.0);
   assert_eq!(grad [x=2.] (dz/dx), 8.0);
   assert_eq!(grad (ddz/dx), 4.0);
};
```

# Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in ast_topology by you, shall be dual licensed under the MIT and
Apache 2.0 license without any additional terms or conditions.

# License

Code is dual licensed under either Apache or MIT Licenses.
