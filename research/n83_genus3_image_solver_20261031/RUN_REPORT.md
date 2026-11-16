# Run report: n=83 genus-3 image transfer and solvers

Date: 2026-10-31
Branch: `codex/n83-genus3-transfer-solver-20261031`
Implementation revision: `4970b8003ec9b8daf06a45f6e5d66f89305c4386`
Extension protocol/curve-record revision: `1bf0f96d090e450b2567bdd45aac8f45eda1eef5`

## Frozen request and protocol

The preregistered [primary protocol](PROTOCOL.md) required the exact pullback,
image-order and composition certificates, scalar recovery, two named
destination adapters, and a cold three-arm comparison. The [follow-up
protocol](FOLLOWUP_PROTOCOL.md) was committed before adding `n=23`, after the
frozen `n=7` source arm exhausted. No failed row or seed was replaced.

Conductor coordination was attempted before editing. This checkout had neither
`.conductor/` metadata nor a `conductor` executable, and the check exited 127.
The work continued in the named branch with preregistration and commits.

## Implementation under test

The transfer family is

```text
E_a: y^2 + x*y = x^3 + a*x^2 + 1
C_a: v^2 + u^2*v = u^7 + a*u^4 + u
phi(u,v) = (u^3, u*v + 1), degree 3.
```

The native solvers are in
[`src/binary_ecc/hyperelliptic.rs`](../../src/binary_ecc/hyperelliptic.rs), and
the named `ecbench` adapters are in
[`src/cryptanalysis/ecbench/methods.rs`](../../src/cryptanalysis/ecbench/methods.rs).
Both adapters use the typed cover pullback API for the generator and target,
then operate on canonical Mumford divisors through native Cantor arithmetic.
`jacobian_image.rho3` is the seeded generic destination control;
`jacobian_image.bsgs` is the memory-using generic destination candidate.

Destination group operations are recorded as native uncharged counters. The
adapters deliberately report zero source GAE rather than silently equating a
genus-three Cantor operation with an elliptic addition.

## Reproduction commands

With Rust 1.98 available and the repository root as the working directory:

```sh
cargo build --release --bin ecbench --bin n83_genus3_image_gate

study=research/n83_genus3_image_solver_20261031
toy="$study/sessions/toy-panel-lt-l2-20261031T043600Z"
n23="$study/sessions/n23-extension-lt-l2-20261031T044000Z"

target/release/ecbench run \
  --spec docs/ecbench/specs/n83-genus3-image-toy-panel.json \
  --out "$toy" \
  --cpus auto --wait --allow-busy

target/release/ecbench verify \
  --dir "$toy" --replay-all --exit-code \
  --out "$toy/audit-replay-all.json"

target/release/ecbench run \
  --spec docs/ecbench/specs/n83-genus3-image-n23-extension.json \
  --out "$n23" \
  --cpus auto --wait --allow-busy

target/release/ecbench verify \
  --dir "$n23" --replay-all --exit-code \
  --out "$n23/audit-replay-all.json"

target/release/n83_genus3_image_gate \
  --output "$study/runs/exact-n83-post-panel-20261031T044500Z/certificate.json"
```

For each session, `ecbench resources --dir ... --out ...` produced
`resources.json`; `ecbench compare --save` produced the three saved pairings;
and `ecbench table` over both session paths produced [TABLE.txt](TABLE.txt).
[`plot_results.py`](plot_results.py) reads only [NATIVE_SUMMARY.json](NATIVE_SUMMARY.json)
derived from those sealed resources.

## Chronology and deviations

Observed chronology:

1. An exact gate was run once as implementation preflight at `04:30Z`. This
   preceded the toy panel and is retained under `runs/exact-n83-20261031T043000Z`.
   It is not represented as the final requested family gate.
2. The first strict toy-session launch, without `--allow-busy`, refused before
   any workload because CPU and memory PSI were unavailable.
3. The identical frozen spec was rerun with `--allow-busy`. The host capsule
   records that 22 user threads could not be moved off the reserved CPU, so all
   executions earned L0, not the requested L2.
4. The preregistered `n=7` source reference and its A/A copy exhausted every
   attempt. Both destination arms verified. The sealed failure remains.
5. The follow-up protocol added the next untouched registered family member,
   `n=23`, without changing arms, parameters, targets, seeds, or accounting.
6. After the combined panel contained four successful faithful sizes, the
   final exact n=83 gate was run at `04:45Z`. Its deterministic certificate is
   byte-identical to the earlier preflight certificate.

Thus the final exact family gate follows the completed four-size comparison,
while the earlier implementation preflight remains disclosed.

## Exact n=83 evidence

The final [certificate](runs/exact-n83-post-panel-20261031T044500Z/certificate.json)
has SHA-256
`911fdab8e1d2978fb995de90b743dd1a3e97719e960515f5668e53dcdb51e819`.
It binds:

- field modulus `z^83+z^45+z^2+z+1`, source equation, curve slug, `r`, cofactor,
  `P`, and `Q=[37]P`;
- nonidentity `phi^*(P)` and `phi^*(Q)`, both killed by prime `r`, proving exact
  order `r` for the image generator;
- `[37]phi^*(P)=phi^*(Q)`;
- `phi_*phi^*=[3]` for infinity, the exceptional target `(0,1)` arising from
  `(u,v)=(0,0)`, and generic fibers over `P` and `Q`;
- `3^-1 mod r = 1611901092820530811123793`;
- a bound-64 `jacobian_image.bsgs` solve that received no planted scalar,
  recovered 37, verified in the destination, and replayed `[37]P=Q` on the
  source.

The bounded solve certifies adapter reachability. It is explicitly not a
full-order 81-bit DLP.

## Session integrity

| Session | Planned/written | Statuses | Deterministic measured replays | Audit |
|---|---:|---|---:|---|
| `ECBS1h2f7b31083d9a` (`n=7,9,13,19`) | 768/768 | 672 verified, 96 exhausted | 560/560 identical | pass, no problems |
| `ECBS1hddc5cdbe0292` (`n=23`) | 192/192 | 192 verified | 160/160 identical | pass, no problems |

Audit receipt SHA-256 values are:

```text
toy: d56c2736e0dc545592c52fd16cbc0765c75463e90b1f9edc347445e80beec6e7
n23: 56f389d35fec8e734f0a4bbcac1c82e2c2c38bf123e6d47e69bd95846fcb6da7
```

Across measured rounds, all 400 destination attempts verified. The two source
arms supplied 320 verified attempts on `n=9,13,19,23` and 80 retained
exhaustions at `n=7`. Including warmups, the complete record has 960 rows, 864
verified and 96 exhausted.

## Evidence classification

The [v3 assessment](TRANSFER_ASSESSMENT_V3.json) promotes `valid_transfer` and
`algorithmically_relevant` to supported. It leaves `modeled_advantage` and
`measured_advantage` unknown because no common operation unit or admitted wall
comparison exists. Novelty is not applicable: the cover and generic solvers
are known constructions.

The remaining work is named in
[`problems/genus3-image-destination-calculus-and-common-cost-missing.md`](../../problems/genus3-image-destination-calculus-and-common-cost-missing.md).
