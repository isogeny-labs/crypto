# Native fast-probe hit regression

An isolated Rust reproducer copied the current Curve25519 source, used the repository Fp256 implementation, and extracted the original GroupOps and CountedGroup definitions. The unrelated Fp64 primality entrypoint was a panic-only shim; it was not reached. This is a native diagnostic, not a full repository-test result or experiment.

For the base G through 64G and Q=-67G, direct affine enumeration finds its first hit at t=3. The original fast probe reports (0,8) and the equality assertion exits 101. Subtracting A/3 before the Montgomery recurrence and adding it back before the Weierstrass-key lookup produces (1,3), exit 0. No timing or speedup was measured.

The original isolated compile failed because the Fp64 primality module was absent; its log is retained. The second original build and corrected build passed.

The main source remains unchanged while its full library gate runs. The patches are unapplied. To reproduce with the real crate, apply regression.patch and run cargo test --release --lib reproduce_probe_hit, retaining the expected failure; then apply coordinate-fix.patch and rerun. Use the repository build/resource settings. Extend the regression to identity crossings and batch boundaries before integrating the fix. This single hit fixture does not establish behavior for those edge cases.
