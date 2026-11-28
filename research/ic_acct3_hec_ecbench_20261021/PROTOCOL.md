# Protocol: the hyperelliptic head-to-head as ecbench records

Frozen 2026-10-21, before the first session of this directory was run.

## Question

`research/hec_bitop_20261020/` priced hyperelliptic index calculus and its
Pollard rho reference in `bitop/v1`, but through a standalone example
(`examples/hyperelliptic_ic_vs_rho_bitops.rs` →
`hyperelliptic_ic_bench::head_to_head`): no sealed records, no replay, no
bound fit. This item ports both arms into ecbench as methods and asks one
question:

> Run through ecbench on the study's genus-2, `p = 41` row, with the study's
> instance, target and seeds, do the records' `bitop/v1` totals equal the
> study's `bitops_per_run`, run for run, for both arms?

This is an accounting port. It is not an improvement and claims none
(CLAUDE.md, success criterion: relabelling and accounting changes are not
improvements).

## What is ported

- **Curve** `hec_thread {p, genus}` (`ecbench::hec::HecInstance`): the
  thread's instance rule `hyperelliptic_ic_bench::thread_instance`, unchanged.
- **Target law** `planted_hec_thread` (`hec_thread_scalar_v1`):
  `k = ⌊r/3⌋ + 7 + index`, the study's `k = N/3 + 7` at index 0. The seeded
  default law (`uniform_scalar_sha256_v1`) also works on these curves; this
  one exists only to reproduce the study's target.
- **Methods**
  - `hec.ic_fb_walk` → `hec_index_calculus_dlp`, factor-base walk
    (`branches`, `d_step_in`), sparse elimination, gcd smoothness, full
    factor base, no large primes;
  - `hec.rho_full_width` → `pollard_rho_jacobian_with(…,
    DistinguishedPointsFullWidth)`.
  Neither receives the planted scalar; the runner verifies `[k]D₁ = Q`.
- **Seeds.** ecbench derives one algorithm seed per (workload, round). The
  study used `20260916 + t` (IC, t < 5) and `0xC0FFEE + t` (rho, t < 40).
  Both methods take `fixed_seed`, which replaces the harness seed; the spec
  has one arm per study seed and one round, so each record is one study run.
- **Counts.** Each `prime_hyperelliptic::ops::Phase` is a record phase whose
  `native` map holds its primitive counts; `ecbench work` prices them one to
  one at `bits(p)` and `bits(r)`, the study's widths. The totals include the
  checked arithmetic's operand re-validation (the study's primary measure);
  the validation subset is reported as `validation_<prim>` counters, not
  added in.

## Spec and commands

`specs/g2-p41.json`: `hec_thread` genus 2, `p = 41`; 5 IC arms (`branches 64,
d_step_in 4, extra_relations 8, max_trials 5000000`) and 40 rho arms
(`max_steps 200000000`); one round, no warm-up, L0 (no wall-clock figure is
used).

```sh
ecbench run --spec specs/g2-p41.json --out sessions/g2-p41 --cpus none
ecbench verify --dir sessions/g2-p41 --replay 2 --out sessions/g2-p41.audit.json
ecbench work --counted sessions/g2-p41 > analysis/work.jsonl
```

## Decision rule

- **Equivalent** if every one of the 45 verified records' `work.bitops`
  equals the study's `bitops_per_run` entry for its seed
  (`research/hec_bitop_20261020/raw/g2.jsonl`, the `p = 41` row), and the
  replay audit reproduces.
- Otherwise **not equivalent**, with the differing runs listed; the port is
  then not faithful and no other figure here is used.

The comparison is exact integer equality, done by a Rust example
(`examples/hec_ecbench_equivalence.rs`), not by inspection.
