# Quartic-extension genus-five cover — toy reproduction (in progress)

Status: complete. Results report: [`REPORT.pdf`](REPORT.pdf) (source `REPORT.typ`,
rendered with Typst from the files in `results/` and `figures/`); tables in
[`results/summary.md`](results/summary.md); benchmark copy in
[`benchmarks/20261026-humbert-genus5-cover/`](../../benchmarks/20261026-humbert-genus5-cover/).
The attack sweep covers 90 runs (q = 61–8009, 3 seeds, full and iota arms): 90/90 logs
recovered, every invariant check passed, and matched rho is faster at every q.

Toy fields only (q prime, 17 ≤ q ≤ 8009; subgroup orders r < 2^62).
Native Rust in `src/`, no dependencies. `plots.py` and `summarize.py` only lay out
recorded values.

## Reproduce

    cargo test --release            # 4 regression tests
    ./run_sweep.sh classify         # results/classify.jsonl
    ./run_sweep.sh verify           # results/verify.jsonl
    PRIMES="61 101 ... 8009" ./run_sweep.sh attack   # results/attack.jsonl
    python3 summarize.py            # Markdown tables of the recorded rows
    python3 plots.py                # figures/*.svg, *.png

## Records

- `results/verify.jsonl` — L-polynomial of the explicit model vs L_E(T^4) L_D(T)
  (q = 17, 19, 23; both twists; 2 seeds).
- `results/classify.jsonl` — binary-pattern classifier, d ∈ {2,3,4}, source genus 1–3.
- `results/attack.jsonl` — end-to-end index calculus on C vs matched Pollard rho.
- `results/attack_v1_superseded.*` — first sweep, kept as a record; it contains one
  wrong log (q = 1511) from a linear-algebra bug (unused factor-base columns),
  fixed before the current sweep.
- `results/host.txt` — host (4 vCPU shared cloud VM, no isolation).
