# E00: frozen polynomial preflight

Status: executable correctness preflight only. This source and protocol must be committed before issuance. It does not run the broader research campaign.

## Hypothesis and fixed inputs

The five statement ASTs compiled in `tools/src/main.rs` represent exact polynomial identities: the chord division, its quartic discriminant, the two cleared complement coefficients, and the cleared complement model conversion. The ordered variable lists and syntactically computed degree bounds are part of those statements. Any statement change changes its certificate ID.

## Commands

Run with Rust 1.90.0 and the committed Cargo.lock:

```sh
cargo build --release --locked --manifest-path research/twist_routes_1_3_20261021/tools/Cargo.toml
research/twist_routes_1_3_20261021/tools/target/release/twist-route-preflight issue NEW_CERTIFICATE_DIRECTORY
research/twist_routes_1_3_20261021/tools/target/release/twist-route-preflight check NEW_CERTIFICATE_DIRECTORY
```

Issuance refuses an existing output directory. Preserve stdout, stderr and exit status separately for both commands. Record source commit, compiler, host, executable/source hashes and the immutable output paths in the receipt. This is an un-timed correctness run; no wall-time or operation speed claim is made.

## Checks and acceptance

- Expand both sides independently into sparse integer-coefficient polynomials; checked i128 arithmetic must report overflow rather than wrap. All coefficient maps must agree.
- Issue standard `identity.certificate/v1` records using the repository implementation at modulus 2^61-1, seed 20261021 and 32 issuer points.
- Check them with the same implementation and 32 extra points from checker seed 0x747769737463686b, in its separate stream.
- Reconstruct every expected statement from the compiled specification before accepting a loaded certificate.
- Add 1 to the right side of each identity. Exact expansion and modular issuance must both reject all five false variants.
- Exit nonzero on the first mismatch, malformed record, overflow or failed control. A partial output directory is a failed run, never a completed certificate set.

Success means five arithmetic preconditions passed these checks and five false variants were rejected. It does not establish group-theoretic interpretation, exceptional-case coverage, a useful factor base, an actual Jacobian transfer, solver benefit, novelty, or NIST security consequences. The records have proof=null; Lean proofs and independent-host replay remain pending.

## Resources and stop condition

One local process; five fixed, low-degree identities. There is no search, paid host, adaptive trial count or performance timing. Stop on any error and preserve its records. No E01–E17 execution is implied.
