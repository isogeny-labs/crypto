#set page(paper: "a4", margin: (x: 1.6cm, y: 1.6cm))
#set text(size: 9.5pt)
#show heading.where(level: 1): set text(size: 14pt)
#show heading.where(level: 2): set text(size: 11pt)

= F4 on a real index-calculus consumer: jv_cover (run-20261120T2241Z)

*Standing:* `[pcc:unverified]` — one shared 4-vCPU cloud VM, isolation L0, no PMU. *Scope:* wall-clock and CPU time of
`jv_cover --exp dlp` (Joux–Vitse cover index calculus, $E(FF_(p^6))$, p = 53), whose relation search is 19,020 Nagao
F4 calls. Counted operations (S) are unchanged by every change here. *Protocol:* `CONFIRM_PROTOCOL.md`, frozen and
pushed (`758f540110`) before data. *Record:* `benchmarks/records/20261120T2241Z-jv-cover-f4-allocation-confirm.json`.

== Result

#table(
  columns: (auto, auto, auto, auto),
  align: (left, left, left, left),
  table.header([*comparison (8 rounds)*], [*predicted*], [*measured median (rounds favouring)*], [*outcome*]),
  [*primary:* wall, baseline / jemalloc], [[1.10, 1.30], ≥ 7 of 8], [1.256 (8/8); 18.15 → 14.60 s], [met],
  [CPU, baseline / jemalloc], [[1.08, 1.25]], [1.251 (8/8); 68.0 → 54.7 s], [just above the band],
  [wall, baseline / move fix], [[0.98, 1.06], may be noise], [1.033 (5/8)], [not significant, as predicted],
  [A/A, move fix / its copy], [within ±5 % every round], [1.021, worst round 1.081], [failed: noise up to 8 %],
)

All 32 runs did identical work: 19,020 residuals, 31 relations, 17,873,891,895 counted oracle operations, a correct
logarithm. The primary effect (1.256) exceeds the worst A/A deviation (1.081).

#figure(image("walls.svg", width: 100%), caption: [Per-round wall time of the four arms and the per-round paired
ratios; values from `data/confirm/runs.jsonl`, plotted by `scripts/plot_report.py`; run-20261120T2241Z.])

== How the lever was found

- *Shipped default checked on the consumer.* \#4473's f4-parallel default against `F4_FP_PAR_OLD=1`: 19.33 s vs
  21.32 s median over 6 rounds, identical work (record `20261120T2059Z-jv-cover-f4-dispatch-ab`).
- *Nested-call dispatch: null.* Serializing F4's inner parallel sections for calls nested in the outer batch (the
  F_p analog of the GF(2) engine's Stage 190): never 19.15 s, build-serial 19.27 s, all-serial 18.75 s; ranges ±7 %
  (record `20261120T2149Z-...`). The code was reverted.
- *Faugère–Lachartre threshold: not a lever.* Counted F4 multiplications per test 727,075 (default), 723,414 (cascade
  only), 902,576 (FL always) at p = 53; wall within noise (record `20261120T2209Z-...`).
- *Profile.* Callgrind of 104 Nagao tests at 1 thread: F4 is 77.6 % of a test. Inside F4 (1.60 G instructions), glibc's
  allocator takes 30.4 %, monomial interning 13.8 %, the step loop 11.7 % and the elimination arithmetic 19.5 %; F4 makes about 22,700 heap allocations per call, mostly one `Vec<u32>` exponent per
  polynomial term.

#figure(image("f4_breakdown.svg", width: 85%), caption: [Self-instruction shares inside F4 on jv_cover's systems,
read from `data/callgrind/jv53_f4only_self.txt` (callgrind, `--toggle-collect` on `f4_with_arithmetic`).])

- *Allocator.* jemalloc preloaded against glibc, 6 rounds: 18.18 → 15.40 s (record `20261120T2212Z-...`). This run
  confirms it with jemalloc built in (`--features jemalloc`).

== Changes

- `f4_fp`: the final interreduction moves the live basis instead of deep-copying it (about 2,000 polynomials per call);
  output unchanged; not measurable on its own here.
- Cargo feature `jemalloc` (optional `tikv-jemallocator` 0.6.1): jemalloc as `jv_cover`'s global allocator; off by
  default so other wall-clock series stay comparable.
- `F4_FP_FL_MIN_COLS`: a measurement knob for the FL threshold (default 256 unchanged).
- `f4_fp_bench xl` tier and `f4_ab_confirm` overrides (from the earlier part of this PR).

== Limits

One host; one curve size (p = 53); the A/A noise was larger than predicted. The allocator change is a property of the
binary, not of F4: any consumer that allocates this way should see a similar effect, but only `jv_cover` was measured.
The structural cause, one heap exponent vector per term in `f4_fp::Poly`, is unchanged.
