# End-to-end timing coverage of secret-key Rust APIs is not established

- **Status:** `open`
- **Kind:** implementation and verification gaps
- **Affected work:** [Rust extensibility and side-channel hardening](../docs/rust-extension-hardening.md), [security policy](../SECURITY.md)
- **Next action owner:** unassigned; Rust cryptographic backend maintainers
- **Last updated:** 2026-10-17

## Problem

The request for an “ironclad”, side-channel-resistant codebase cannot be
established by the existing fixed-width arithmetic alone: secret-key APIs
still cross variable-time BigUint representations and legacy backends, and
target-specific generated-code and independent audit evidence is missing.

The relevant threat model includes attackers observing runtime, memory-access
patterns or accidental diagnostics of valid secret-key operations. Cryptanalysis
and tracing APIs intentionally use public fixtures and variable-time arithmetic;
they must not be mistaken for secret-key backends. This record preserves the
broader requirement beyond the targeted engineering changes.

## Evidence and uncertainty

- **Observed:** the parent revision `459eceff` converts private ECC scalars and
  RSA CRT parameters from `BigUint` using `Uint::from_biguint`, whose conversion
  and leading-byte handling depend on representation. Unsupported RSA sizes
  fall back to `utils::mod_pow_ct`, which uses variable-time BigUint arithmetic.
  `SECURITY.md` documents variable-time Patterson decoding, nonstandard toy
  Kyber and incomplete zeroization of BigUint allocations.
- **Observed:** Montgomery setup previously divided by secret CRT primes and
  private-key derived `Debug` exposed secret fields. The current change replaces
  setup with fixed-width doubling, redacts diagnostics and uses `zeroize` for
  owned PQC secret buffers. Its arithmetic tests do not establish whole-API
  timing behavior.
- **Derived or proved:** no end-to-end timing proof or third-party audit has
  been completed here. A fixed loop bound does not prove fixed machine timing.
- **Suspected explanation:** no unknown mathematical obstruction is needed;
  the remaining work is implementation coverage and target-specific verification.
- **Missing capability:** a complete secret-data-flow inventory, fixed-width
  key representations on all claimed paths, and independent timing/code review.
- **What this evidence does not establish:** a practical attack on a deployed
  system, resistance on arbitrary hardware, full memory erasure, or production
  readiness of the crate.

## What would unblock it

- [ ] Inventory every supported secret-key entry point, its secret inputs,
  backend, conversions, fallbacks, error behavior, diagnostics and erasure
  contract; distinguish deliberately variable-time research APIs.
- [ ] For each path claimed resistant, remove or explicitly exclude all
  secret-dependent BigUint conversion/arithmetic and legacy fallback behavior;
  use validated fixed-width key types and preserve known-answer results.
- [ ] Review optimized code for each declared compiler/target, check timing with
  a preregistered leakage-detection protocol and positive controls, and obtain
  independent cryptographic review. Record the exact remaining threat-model
  exclusions instead of inferring arbitrary-platform security from tests.

Meeting these conditions supports a scoped claim for the reviewed paths and
targets. It cannot establish universal absence of side channels or standards
compliance for this educational library.

## Next discriminating step

- **Action:** start the secret-data-flow inventory at ECC key generation and
  RSA private-operation import/conversion boundaries; identify the next
  fixed-width key representation change before timing experiments.
- **Inputs and controls:** supported P-256/secp256k1 and 1024–4096-bit RSA known
  fixtures; existing arithmetic/PKCS known-answer tests; public-input paths as
  separate controls, not substitutes for secret paths.
- **Execution limits and stop condition:** source review and local regression
  checks in this task. No timing experiment, research campaign or paid job was
  launched for this record. Any measurement requires its own frozen protocol.
- **Outcomes and next actions:** complete data-flow coverage defines a bounded
  backend migration; missing paths remain explicitly unresolved; tool failures
  are access limits, not evidence of resistance.
- **Recheck trigger:** fixed-width key/API migrations or new independent
  generated-code, timing or audit evidence.

## Resolution and history

2026-10-17: opened while implementing caller-owned Rust plug-ins and targeted
hardening. Corrected the earlier “unavoidable setup leakage” and “all supported
secret paths closed” wording. Unresolved; passing this change's regression
tests does not satisfy the broader end-to-end side-channel requirement.
