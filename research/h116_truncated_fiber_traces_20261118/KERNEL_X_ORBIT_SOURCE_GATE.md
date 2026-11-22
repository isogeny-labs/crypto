# H116 kernel x-orbit source gate

This gate records the `H116-KERNEL-X-ORBIT-001` source hashes that were sealed
before its first affected panel execution.

```text
7ebfb9e0780c3dc82db95e6391d219160043f5c434a6effcffbe0a844d52ca07  KERNEL_X_ORBIT_PROTOCOL.json
c4ca6bb3bc3fa9b717862c1e73190c9525d65fc4682a0051e15d05b54187189d  h116_kernel_x_orbit.rs
d7cd388324280b53c5daf0a20101f9a8ea99fdbd2712e9088ebcab7fd8a5008f  h116_kernel_x_orbit_verify.py
577015b44fae9828b9bddf1b2afc0403e4f44d732b3a01325f42253f0982cc82  h116_field.rs
469ef0cd5ca96329a9543b4f1afbc55f0c58027e927151dbe56429917df2126c  h116_pointwise_charpoly_verify.py
```

The producer compiled without warnings using Rust 1.100.0 and
`--edition=2021 -O -D warnings`.  The verifier passed Python bytecode
compilation.  Both multiplicity/derivative self-tests passed after the source
hashes were sealed.  The protocol and both source files did not change between
this gate and the single recorded execution. [pcc:unverified]
