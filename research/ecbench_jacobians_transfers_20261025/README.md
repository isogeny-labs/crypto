# ecbench on Jacobians and through transfers: first measured sessions (2026-10-25)

**What this is.** These are the first ecbench sessions on the workload kinds
and methods this PR adds:

- genus-2 Jacobians and `ic.hyperelliptic`;
- plane-quartic (non-hyperelliptic genus-3) Jacobians;
- GHS curves over `F_{p⁶}` and `transfer.ghs_cover`;
- `transfer.isogeny` and `transfer.quartic_cover` on a prime curve.

Every arm is measured against its matched rho on the same workloads, in
one counted unit, with every answer verified and every measured run
replayed exactly.

**Classification.** This is new capability, measured. **It is not an
improvement over the index-calculus baseline** (AGENTS.md "Success
criterion"). No row here lowers the cost of any registered IC pipeline. No
row goes to the IC leaderboard or `benchmarks/README.md`; neither the
leaderboard's tables A–C nor the validation gate's row format covers
these families. Each row's distance to rho is stated below.

Run report, graphs and PDF:
[`reports/run-20261025T0158Z/`](reports/run-20261025T0158Z/) (`REPORT.typ`, figures as SVG and PNG)
(`REPORT.pdf` is in the artifact store; `artifacts.py pull` restores it).

## Sessions

| family | spec | session | records | verified | replayed exactly (`verify --replay-all`) |
|---|---|---|--:|--:|--:|
| genus-2 Jacobians | [`specs/genus2.json`](specs/genus2.json) | `ECBS1h1a7b060b52e9` | 160 | 160 | 120 of 120 measured |
| plane-quartic Jacobians | [`specs/quartic.json`](specs/quartic.json) | `ECBS1hcb9aa3261765` | 96 | 96 | 72 of 72 |
| GHS curves over `F_{p⁶}` | [`specs/ghs.json`](specs/ghs.json) | `ECBS1h31140d2225ae` | 96 | 96 | 72 of 72 |
| prime curve (16 bits) | [`specs/prime.json`](specs/prime.json) | `ECBS1hd12205a7f257` | 80 | 80 | 60 of 60 |

Common to all four sessions:

- Commit `6111849ed` (clean); binary `e0dc91fa…`; host class
  `ECBENV3h8be61a9c1732` (cloud container).
- Isolation: runs earned L1 and L2. L3 is blocked: no isolcpus, no
  performance governor, boost not known off.
- Each session runs 3 rounds plus 1 warm-up. There are 4 targets per curve
  and two curves per family; the prime family has one curve.
- Each session has an A/A arm. The operation counts are deterministic, so
  the A/A arm's operations equal the reference's exactly (ratio
  1.0000 [1.000, 1.000]). Its wall-time ratios are 0.85–1.03.

The verify outputs are in `sessions/<family>.verify.txt`. The comparisons
are in `sessions/<family>/comparisons/` and the §5a band in
`out/band-<family>.{txt,json}`.

## Results (operations; S = GAE / √r)

Units: on the Jacobian workloads one GAE is one Jacobian operation. On
`ghs_fp6` and prime curves one GAE is one addition on `E`, including
inside a transfer, whose Jacobian work is converted at `c_E` measured in
the run (docs/ecbench/README.md §7a).

| family, curve (log₂ r) | arm | mean S [95%] | paired ratio to matched rho [95%] | §5a band (w = 0 / 0.1 / 1) |
|---|---|---|---|---|
| genus 2, `jcv1-g2-fp10-4c1c114a` (17.5), `jcv1-g2-fp12-7c76fc39` (20.2) | `rho.negation` (reference) | 1.528, 1.249 | 1 | |
| | `rho.plain` | 1.528, 1.545 | 1.106 [0.875, 1.399] | unresolved |
| | `bsgs.negation` | 0.952, 1.096 | 0.737 [0.609, 0.892] | below rho at every weight |
| | `ic.hyperelliptic` | 1.807, 2.284 | **1.473 [1.158, 1.874]** | 1.47 / 37.5 / 216: above rho at every weight |
| plane quartic, `jcv1-pq-fp6-529ca663` (9.7), `jcv1-pq-fp7-aee0e2be` (15.0) | `rho.plain` (reference) | 4.185, 1.887 | 1 | |
| | `bsgs.textbook` | 1.676, 1.344 | 0.497 [0.378, 0.654] | below rho at every weight |
| GHS, `ghs-fp13x6-9fa1271f` (20.2), `ghs-fp19x6-e6d00613` (23.5) | `rho.negation` on `E` (reference) | 1.215, 0.957 | 1 | |
| | `transfer.ghs_cover` (inner `rho.negation` on `Jac_H(F_{p²})`) | 6.810, 7.580 | **6.62 [5.13, 8.54]** | 6.62 / 6.17 / 3.98: above rho at every weight |
| prime, `prime_search` 16 bits seed 3 (16.0) | `rho.negation` (reference) | 2.031 | 1 | |
| | `rho.plain` | 2.023 | 0.996 [0.625, 1.587] | below rho-neg at every weight (rho-neg's canonicalisations are priced from w = 0.1) |
| | `transfer.isogeny` ℓ = 3 (inner `rho.negation`) | 1.897 | 0.934 [0.744, 1.172] | 0.93 / 1.54 / 4.72: unresolved |
| | `transfer.quartic_cover` (inner `rho.plain` in `J(C)`) | 31,065 | **15,292 [11,065, 21,134]** | above rho at every weight |

The prime family has four workloads on one curve, so `compare` marks its
intervals INDICATIVE. Every BSGS arm keeps a table of about √r entries, so
its ratio is not a like-for-like comparison with rho's.

**Measured conversion constants** (recorded per run in `detail`; ranges
over the measured runs):

- GHS cover: `c_E` = 331 `F_p` multiplications per addition on `E(F_{p⁶})`.
  One genus-3 Cantor operation costs 6.26–6.37 additions on `E`.
- Plane-quartic cover: `c_E` = 30. One Jacobian operation costs
  13,775–13,776 additions on `E`.

## What the results say, and do not

1. **The transfers are correct and priced; none is cheaper than rho.** All
   three are verified on `E` and replayed exactly.
   - GHS cover: the generic inner solve pays rho's `√ℓ` times one Jacobian
     operation in `E`-additions. The measured ratio, 6.6 [5.1, 8.5], agrees
     with the recorded conversion factor of 6.3. GHS pays off only through
     index calculus on the cover (Joux–Vitse decomposition: `jv_cover`, not
     yet an inner method).
   - Plane-quartic cover: its linear-algebra group law is about 13,800
     `E`-additions per operation. No plane-quartic index calculus (Diem)
     exists here to use it.
   - Isogeny transfer: it moves the instance to a curve with the same
     subgroup order. At w = 0 its ratio is consistent with no change
     (0.93 [0.74, 1.17]). Its kernel search and maps are unpriced field
     work, so from w = 0.1 it costs more than rho.
2. **Genus-2 index calculus is 1.47× rho at these sizes at w = 0, and far
   more once its native work is priced.** Its smoothness multiplications
   and elimination row operations are the §5a band's 37–216×. This is a
   distance to rho, not a verdict: the first measured `ic.hyperelliptic`
   rows are its baseline in this harness. Gaudry's genus-2 index calculus
   is `q²` and the double-large-prime variant reaches `q^{2−2/g}` = `q¹`,
   rho's exponent. So no exponent gain is expected in genus 2; what can be
   improved is the variant (`large_primes`) and its constant.
3. **Small-r effects are visible.** Plane-quartic rho at r = 2^9.7 has
   S = 4.2: the jump-table set-up is a large share of so few steps. Read the
   quartic rows at 2^15.0.

## Next improvement to try

The genus-2 IC baseline is now measured: S = 1.81 (2^17.5) and 2.28 (2^20.2)
at w = 0. The next step is the double-large-prime relation search
(`large_primes=1`, already a parameter), measured against these rows on the
same workloads. Next after that, the Joux–Vitse decomposition on the GHS
cover as an inner method of `transfer.ghs_cover`, measured against this
session's `rho.negation` rows.

## Reproduce

```sh
cargo build --release --bin ecbench
B=target/release/ecbench
for n in genus2 quartic ghs prime; do
  $B run --spec research/ecbench_jacobians_transfers_20261025/specs/$n.json --out /tmp/$n --cpus auto
  $B verify --dir /tmp/$n --replay-all --exit-code
  $B table --dir /tmp/$n
done
```

The sessions' `records.jsonl` stay in Git, on the artifact policy's `keep`
list: ecbench CI audits every committed session, and its workflow has no
store pull step. The report PDF and the PNG render are in the artifact
store (`python3 tools/artifacts/artifacts.py pull research/ecbench_jacobians_transfers_20261025`).

None of the curves here is registered: the Jacobian and GHS slugs
(`jcv1-…`, `ghs-…`) have no ICV1 name by construction. The prime curve is
named above by its construction (`prime_search` 16 bits, seed 3); its ICV1
slug is in the records.
