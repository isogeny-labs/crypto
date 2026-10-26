# Quartic-extension genus-five cover — toy reproduction (in progress)

Status: work in progress. The full results report, PDF and `benchmarks/` entry are
not written yet. The attack sweep for q = 2003–8009 was still running when this
was committed; `results/attack.jsonl` holds the rows completed so far.

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
