# Exact primality fast paths — 2026-10-30

The requested change is to remove the practical prime-certification setup wall
without accepting probable primes as certified primes. The delivery baseline is
crypto `fc9af16fa1ce0654ee9df3ce63101c487ed215fb`, after the checked hash-context
change landed in PR 2924. Its unmodified original-input validation still stops
at `resource limit: AKS multiplicative order`, exit 1. The initial baseline was crypto
`3b34d504bc13ce668e86d8e68cdf57e3fbaba89a`; the peer integration starts at
cryptanalysis `fc506a9341915fe00add8e156bc9ff7708380431`.

## Implementation

1. Preserve the existing exact u64 path and tiny-input behavior.
2. Use thirteen strong witnesses strictly below
   `3317044064679887385961981`. At and above that number, passing witnesses
   leaves primality unknown; a failed witness still proves compositeness.
3. Automatically verify the existing bundled ECCp-131 public proofs when the
   requested integer is covered. Only JSON parsing is cached. Integer proof
   verification, width/vector checks and charged work recur on every call.
   The original input file now exercises this path without manual proof fields.
4. Attempt a deterministic bounded Pocklington certificate for other larger
   inputs, using small trial factors and Pollard rho to obtain enough certified
   factors of n-1. Verify successful output with the existing independent
   verifier. Stop factoring once F squared exceeds n; full factorization is
   unnecessary. Proof generation has caps of 16,384 rho iterations, 32 levels,
   64 proof nodes and 256 common witnesses per node. Generated proof leaves
   above u64 use the same strictly bounded deterministic test.
5. Retain AKS when the bounded attempt produces no proof. Resource exhaustion
   remains an error. No proof-search failure is returned as a composite result.

The code changes no random-source calls or rejection-sampling distribution.
It changes neither the network encoding nor the contraction admission guard.
The [flow diagram](flow.svg) and [PDF](report.pdf) describe this exact scope.

## Sources checked before implementation

- Existing `src/arithmetic.rs`, `src/primality.rs`, `src/curve.rs` and
  `src/reduction.rs` in the native crate; the earlier
  [input-validation receipt](../../docs/hashed-ecdl-eccp131-validation.md),
  `research/btn_ecdl_20261029/GOAL.md`, and the existing checked-hash work in
  [crypto PR 2924](https://github.com/isogeny-labs/crypto/pull/2924).
  That PR retains checked prime contexts through hash setup and has now merged.
  This follow-up addresses uncovered ordinary primality calls on current main,
  preserving its new square-root counters and contraction-cache changes.
- Sorenson and Webster, *Strong Pseudoprimes to Twelve Prime Bases*,
  [Theorem 1.1](https://arxiv.org/pdf/1509.00864): read the definition of
  the least strong pseudoprime and both exact cutoffs. The thirteen-base
  cutoff is exclusive, and its composite endpoint is a regression control.
- Menezes, van Oorschot and Vanstone, *Handbook of Applied Cryptography*,
  [Chapter 4, Fact 4.40](https://cacr.uwaterloo.ca/hac/about/chap4.pdf):
  read the partial-factorization, certified-prime and modular/gcd conditions.
  The existing verifier's stronger F squared > n condition is retained.
- The same handbook,
  [Chapter 3, Algorithm 3.9 and Note 3.12](https://cacr.uwaterloo.ca/hac/about/chap3.pdf):
  read Floyd-cycle rho factoring and its failure/restart conditions. Rho finds
  untrusted factors; only bounded exact tests or checked proofs establish
  their primality. This implementation makes no new mathematical claim.

## Validation and limits

The final native package panel passes **74 tests** (50 library, 5 runner,
3 panel, 9 finite-extension, 3 strict-JSON, 4 width). Strict all-target clippy,
formatting and the Rust 1.78 all-target check pass. The peer passes its
86-test package panel and the separately added cached-proof resource test;
its inherited checked-hash behavior and tests are retained.

The original `eccp131.json` validates p/r/target in **17,471 uncalibrated charged
steps**, below its unchanged 2,000,000-step policy, in both integrations.
The 65-bit known prime completes in 1,672 charged steps and the 127-bit known
prime in 45,983; the unchanged AKS branch exhausts 2,000,001 charged steps in
each regression control. These are development admission diagnostics,
not calibrated costs or elapsed-time measurements.

Validation commands, exact source and input hashes, host identity, successful
logs and failed development attempts are retained beside this file.
The host is a Linux x86-64 development container with five logical CPUs on an
Intel Xeon Platinum 8573C, using Rust 1.99.1 and Rust 1.78.0 for compatibility.
It has no timing-isolation admission. Cargo's displayed durations are build/test
diagnostics, not benchmark measurements.

The 65-bit cutoff regression and a 127-bit known prime compare successful exact
decisions against the original AKS branch under the same two-million-step
development policy. The meter counts uncalibrated abstract operations; no
runtime ratio or pipeline speedup is inferred. Other controls cover 89/107/127-bit
proof production and replay, the composite cutoff itself, forged proofs,
exhausted proof searches, and fresh resource limits after cached parsing.

The package tests also retain the seeded reduction/replay, encoding and forced
recursion controls. Raw ECCp-131 input preflight checks parameter/target setup;
it is not relation collection or a challenge solve.

| Requirement | Status and acceptance evidence |
| --- | --- |
| Preserve certified decisions | Exact cutoff controls, independent proof replay, corruption and limit tests |
| Remove the 65-bit AKS cliff | Bounded deterministic test, regression under the original work policy |
| Ordinary ECCp-131 parameter setup | Bundled proofs verified automatically; original-input preflight |
| More general large primes | Bounded proof generation; hard n-1 cases still need supplied proofs or AKS |
| Native parity | Same primality module and shared arithmetic additions in the peer; independent peer changes retained |
| Admitted timing and external comparison | Not run; no fastest-tool or elapsed-time claim |
| Recursion crossover and feasible challenge search | Unchanged, unresolved |

The next certification improvement is a checked ECPP/APR-CL backend or externally
prepared proof import for hard factorizations. Uniform random hash candidates
can still miss this bounded proof search. Existing explicit checked hash-modulus
work remains separate. No source theorem, smaller recursion cutoff, end-to-end
IC/rho result, challenge scalar, or independent general audit is established.

The canonical scoreboard, progress timeline and leaderboard remain unchanged:
this change adds development correctness/admission checks, not an admitted
performance row. A matched native L2 benchmark and external prime prover
comparison remain open under the existing execution/checkpoint workflow.

## Delivery integration and construction control

The source patch is carried on current main as a separate follow-up to merged
PR 2924. The peer implementation is merged in
[cryptanalysis PR 1322](https://github.com/isogeny-labs/cryptanalysis/pull/1322),
merge `60cd28292fbb49a8f2675983cd97a914e174bb34`, with expected head
`28006afb786cac698bc0048f5e0875a965fb81e9`. Its final integration passed 94 tests.
Historical first-stage records above remain separate from the delivery records.

`native-setup-check` constructs the official published-Q decomposition network
after ordinary p/r certification and verification of a fixed 132-bit hash
modulus, then creating two checked factor-base predicates. The supplied proof
list deliberately omits p/r, requiring the automatic ordinary primality path.
The unchanged baseline package was verified file-for-file against `fc9af16f`;
the exact same client source and lockfile exhaust its AKS budget, exit 1. With
the patch it constructs the network in 19,722 setup / 19,925 total charged steps.
This is a functional admission comparison, not an elapsed-time comparison.
This is a construction control with threshold 256 and
explicit valid seeds `(s/3,s/7)` and `(s/5,s/11)`. It neither enumerates a factor
base nor calls a count/witness/solver operation. The graph vector policy is
2,000,000 (the original input's 200,000-vector cap is too small for this graph);
the step policy remains 2,000,000 and integer policy 4,096 bits. This control
does not replace the frozen default-solver workload or establish its coverage.

The earlier integrated construction completed with 1,684,796 edges, 249,620
vertices, degree 1,032 and 526 interface bits. Its charged totals were 5,695
for curve/base setup and 5,898 through construction. These are uncalibrated
admission diagnostics, not calibrated work or timing measurements. Delivery
results and exact command/source identities are retained in the additive
`delivery-*` records and [final integration report](integration.pdf).
The delivery panel passes **94 native tests**, strict all-target clippy,
formatting and Rust 1.78 checks. Its original-input and fixed-hash preflights
charge 17,471 and 5,897 steps respectively; the construction control reproduces
the graph above on current main plus this patch. `automatic-*` records identify
the matched no-p/r-proof control; `delivery-network` retains the earlier
explicit-all-proofs control. `source-snapshots/network-explicit-proofs.rs` is
the earlier client's exact source; the prior snapshot manifest's `main.rs`
entry refers to that version, not the final client.

`source.sha256` identifies the historical first stage at `95cf0a81`; its lock
snapshot is `initial-native-Cargo.lock`. `delivery-source.sha256` and
`delivery-native-Cargo.lock` identify the current integration. Restore the
applicable lock snapshot to the package's `Cargo.lock` before using
the corresponding `--locked` commands. Gzip raw records retain their original
bytes; `raw.sha256`/`delivery-raw.sha256` record uncompressed identities.

## Parent-library validation limitation

The full parent-library command was attempted at source `95cf0a81`. Earlier
attempts lacked tracked compile inputs; the compiled run then hit 18 missing
runtime-fixture failures in the sparse checkout.
All 18 exact-name replays pass after materialization against the identical
executable, SHA256
`b1d584c5a85a4407e20731acc2978bca4fb8f93e06f6d1d3e5196ead5d328fe0`.
The sole case still unreturned was
`cryptanalysis::jv_isogeny_walk::tests::end_to_end_from_a_non_weak_curve_recovers_and_verifies`.
Both cargo and the executable later became defunct with exit status 9
(SIGKILL); the cause of that signal is not established. There is no remaining
running validation job and no full parent-suite pass is claimed. The 123
default ignored cases were not enabled or reclassified. This does not describe
a failure of prime-proof verification or a mathematical impossibility.

See [the specific validation obstruction](../../problems/root-library-isogeny-walk-runtime.md).
The affected native package and construction control are validated separately.
Auto-merge of this follow-up remains held while the required broader validation
is incomplete; no approval or merge gate is bypassed.
