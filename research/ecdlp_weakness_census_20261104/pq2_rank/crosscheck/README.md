# Independent cross-check of the iterated closure

`naive_closure.py` recomputes the degree-d closure without the Rust program's
incremental shortcut (multiplying only rows with new pivots). It saturates the
full span under multiplication by every variable until the rank stops growing,
in pure Python, and reads off dim(S ∩ P≤1). It shares no code with `src/main.rs`.

Inputs are the descended equations and solution sets that the Rust binary writes
when `PQ2_DUMP=<dir>` is set (`n1{4..8}_i0.json`). `rust_same_instances.txt` is
the Rust output on the same instances, `closure_d3.txt` and `closure_d4.txt` the
Python output.

| n | k | Rust iter3 | Python d=3 | Python d=4 |
|---|---|---|---|---|
| 14 | 13 | 13 | 13 | |
| 15 | 15 | 1 | 1 | 15 |
| 16 | 15 | 15 | 15 | |
| 17 | 17 | 1 | 1 | 17 |
| 18 | 17 | 1 | 1 | 17 |

Both implementations agree on every instance. The degree-4 column goes beyond
the Rust run's degree-4 range (n ≤ 16).

    python3 naive_closure.py 3 n1*_i0.json
    python3 naive_closure.py 4 n15_i0.json n17_i0.json n18_i0.json
