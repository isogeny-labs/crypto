# CT229 — precommitted reduced factor-base experiment (F101 cubic cover)

2026-11-19 · [crypto #4715](https://github.com/isogeny-labs/crypto/pull/4715) · [pcc:unverified]

## Fixed problem and hypothesis

Reuse CT226's native genus-4 Cantor implementation and the same odd cubic cover
`C/F101: y²=u⁹+u³+46 -> E/F101: y²=x³+x+46`,
with the same public source generator `G=(2,37)`, public target `Q=(64,19)`,
prime source subgroup order 109, signed rational point roster of size 54
and the same 154 lexicographically generated offsets from the first eight
rational point representatives. The source scalar 37 appears **only** in
post-solve checks and the BSGS baseline. No preassigned generator logarithm
or source-DLP oracle enters relation finding or linear algebra.

**Hypothesis:** truncating the signed rational factor base decreases the
matrix dimension but reduces full-degree-4 smooth relation and target yield;
a smaller matrix does not imply a cheaper *complete* cold solver. If the
reduced base cannot reach full rank or recover the target under the fixed
candidate budget, record the failure, not an estimated ECDLP cost.

## Frozen design, before hosted results

* Choose deterministic lexicographic **prefix** basis sizes
  `[12,18,24,36,48,54]`. This is deliberately **not** an optimized factor
  base and uses no selection based on observed smoothness or subgroup logs.
* Use the unchanged CT226 signed offset schedule. Per basis, inspect at most
  60 offsets and 109 consecutive source-group multiples per offset
  (6,540 relation candidates); stop collection **only** at full column rank.
* Test degree-4 squarefree **fully split** reduced Mumford supports with all
  four signed roots inside the selected prefix. Reject partial/large-prime
  relations; count candidates, smooth events and rank exactly.
* If the column rank is full, recover all factor-base source logs by
  modular elimination and independently check the resulting point logs.
  Then use at most 60 target offsets times 109 shifts and report whether a
  valid target discrete logarithm is recovered. Verify `[a]G=Q`.
* For each base collect separate setup/relation/linear algebra/target times,
  full candidate elapsed and a directly executed same-job BSGS control,
  alternating comparator ordering. Report all six records including
  missing ranks and failures.
* The 54-column control **must** reproduce CT226's exact
  `(rank,smooth,candidates)=(54,62,1421)` and correct scalar 37.
  Malformed relations trigger a hard assertion. CI requires exactly six
  structured records and a successful test, on the exact branch source.
* Keep this within existing bounded GitHub-hosted native CI (no EKS/Spot
  launch, original S3 authorization, hardware PMU, secrets or paid compute).

## Reproduce

```sh
cargo test --locked --release --test ct229_reduced_base_probe ct229_ -- --nocapture
```

Source: [ct229_reduced_base_probe.rs](../../../tests/ct229_reduced_base_probe.rs).
The source includes [CT226's existing Rust implementation](../../../tests/ct226_native_genus4_transferred_ic.rs)
directly, rather than implementing a new or divergent arithmetic library.
CI: [ct205-toy-transfer.yml](../../../.github/workflows/ct205-toy-transfer.yml).
Parse `CT229_PANEL_JSON` lines; each has rank, smooth/candidate counts,
target success/failure, elapsed phase timings, baseline, and operation
counts. There is no post hoc pruning of unsuccessful bases.

## Expected scope and existing evidence

This is a **finite, public F101, deliberately bounded diagnostic**. It is
neither a scaling theorem nor an independently reproduced PCC. Any same-host
speed ratio is unqualified without cold comparable optimized rho, BSGS,
frontier IC, calibrated primitive costs, peak RSS, blind fresh targets,
confidence intervals and independent certification. The existing CT227
full-base solve is approximately 26,020x slower than direct BSGS, and CT228
proves the full rational factor base grows as Theta(p) for p≡2 mod3.
CT229 asks whether a smaller base already loses too much relation/target
yield at the fixed toy instance; it does not manufacture a winning arm.

Prior art: [Diem (2006)](https://arxiv.org/abs/math/0606607) and
[Gaudry–Thomé–Thériault–Diem (2007)](https://doi.org/10.1090/S0025-5718-06-01900-4)
show why varying a factor base and *large-prime relation combination*
are established methods for fixed small genus. This pilot deliberately
omits large-prime combining and makes no novelty claim. In particular,
the cyclic full-Jacobian hypotheses of some asymptotic theorems do not
apply directly to CT224's composite split Jacobian.

Cross-project overlap check: [crypto #4215](https://github.com/isogeny-labs/crypto/pull/4215)
and [cryptanalysis #1523](https://github.com/isogeny-labs/cryptanalysis/pull/1523)
own selected-call norm-frontier data; [cryptanalysis #1766](https://github.com/isogeny-labs/cryptanalysis/pull/1766)
owns the original-model cold adjudicator and
[crypto #4178](https://github.com/isogeny-labs/crypto/pull/4178)
owns GPU IC comparisons. Their authenticated 695/701-record S3 replay
and physical C169/C13a/C13b/D pricing remain independently gated; this
pilot does not duplicate them. Research-KB MCP and checkout-local wrapper
were unavailable here; source review used the linked repository reports
and primary literature.
