# Source-only review: integer Hasse ceiling for odd extension degrees

Date: 2026-11-23. Lane: [PR #4977](https://github.com/isogeny-labs/crypto/pull/4977). Reviewed source: `e3f78ff8330a5acb2c805938f44a943aa9bafc08`; reference candidate blob `616422524711c1dd0e54b6bb6c557348b999e262`. **Producer-side mathematical/source review only; [pcc:unverified].** No original-root Rust compilation, curve order census, native solver experiment, or signed independent validation.

## Finding: the candidate preserves a pre-existing too-small upper endpoint

The complete-union candidate, both valid parents (blobs `12758d4edc3f1d5f6c6c66ebb32397d04c991af7` and `89a190d53bcd20a8141c219db2f0f8fd963b9ce2`) and the malformed current main source `f001a1b641abd48bb0130c1a9a246e37c28bcc3a` all contain this expression in `hasse_interval`:

```rust
let q = BigUint::one() << curve.m as usize;
let root = q.sqrt(); // integer floor sqrt
let g = curve.genus as u32;
let hi = (&root + BigUint::one()).pow(2 * g);
```

For an integer field size `q=2^m`, the Hasse-Weil Jacobian bound is

```text
(sqrt(q) - 1)^(2g) <= #Jac(C)(F_q) <= (sqrt(q) + 1)^(2g).
```

Let `s=floor(sqrt(q))` and `t=ceil(sqrt(q))`. If `m` is odd, `s<sqrt(q)<t=s+1`, so

```text
(s+1)^(2g) < (sqrt(q)+1)^(2g) < (t+1)^(2g).
```

The existing `hi=(s+1)^(2g)` is **not a certified upper bound** for nonsquare `q`. A simple conservative integer repair is to use `hi=(ceil(sqrt(q))+1)^(2g)`, retaining the existing conservative `lo=(floor(sqrt(q))-1)^(2g)` for `q>=2`. One possible implementation, for future review, is:

```rust
let floor_root = q.sqrt();
let ceil_root = if &floor_root * &floor_root == q {
    floor_root.clone()
} else {
    &floor_root + BigUint::one()
};
let hi = (&ceil_root + BigUint::one()).pow(2 * g);
```

This adds at most one to the integer root before the power. The alternative `ceil(sqrt(q))` is an upper estimate for `sqrt(q)`, not a claim that the resulting `hi` is the *tightest* possible integer endpoint.

## Bounded exact integer examples (not group-order experiments)

| m | q | genus | current hi | conservative hi |
|--:|--:|--:|--:|--:|
| 3 | 8 | 2 | 81 | 256 |
| 5 | 32 | 2 | 1,296 | 2,401 |
| 6 | 64 | 3 | 531,441 | 531,441 |
| 10 | 1024 | 2 | 1,185,921 | 1,185,921 |

The first two show an **invalid bounding derivation**, not that a specific curve in the repository was observed to have order outside the old endpoint. The even-degree cases are unchanged. The displayed values were checked by coordinator BigInt integer arithmetic; no elliptic/hyperelliptic arithmetic or Rust tests ran. [pcc:unverified]

## Consequence for the exact root correctness gate

The candidate's `divisor_order_bsgs` calls `hasse_interval`, uses `width=hi-lo`, and scans candidate annihilating multiples starting from `lo`. A too-small `hi` can make the search incomplete for a sufficiently large-order divisor, producing `None` even when `max_baby_steps` is adequate. It does **not** imply that every odd-m input fails: smaller divisor orders may have multiples inside the shortened interval.

The preserved parent test suite checks `hasse_interval_is_ordered` only on `m=10,g=2`; the source's genus-three recovery fixture uses `m=6`. Both use square `q`, so those checks would not detect this missing upper reach. The issue predates the malformed merge and must **not** be attributed to the complete-union reconstruction or silently patched into its immutable candidate.

## Sequential validation / ownership

1. Independently review the mathematical inequality and the exact `hasse_interval` caller contract; distinguish a certified conservative endpoint from tightness.
2. In a separately authorized root-source repair lane, add an odd-degree `m=3` and `m=5` integer-bound regression, preserving all inherited tests and both parent APIs. Test both endpoints as exact integer inequalities; an optional genus-two curve-specific regression must use original Rust arithmetic and a separately certified order.
3. Freeze repaired root source, preserve the original manifest/lock and the guard restore, then rerun the original five compilation commands and signed completeness/root/sign/duplicate/miss/rank controls. Do not use this source-only finding to claim a root pass or an S3 query speedup.

Source custody: candidate `research/prime31_summation_compression_20261122/iteration10/source_reconstruction_followup/BINARY_HEC_COMPLETE_UNION_CANDIDATE.rs`; parent audit and review in the same directory; direct current `main` and lane `src/cryptanalysis/binary_hyperelliptic_index_calculus.rs` readback blob `f001a1b...`. Literature: standard Hasse-Weil Jacobian interval (e.g. [background reference](https://en.wikipedia.org/wiki/Hyperelliptic_curve_cryptography#Order_of_the_Jacobian)); Semaev 2015 is the existing unrelated S3 prior-art anchor. Shared KB, Conductor and native child-worker dispatch were not exposed in this runtime.

Workers: one coordinator performed source/algebra review and bounded BigInt arithmetic; no independent algebra, Rust or validation workers were spawned. Rust worker scope stays reserved/deferred; validation work package is the explicit m-odd regression and original-root gate above. This is a **source-reconstruction correctness dependency**, distinct from the still-open Prime31 exact sublinear query objective. [pcc:unverified]
