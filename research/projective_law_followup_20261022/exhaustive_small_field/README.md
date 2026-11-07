# Exhaustive finite weighted-law gate

Schema: `projective-law-exhaustive-small-field/v1`.

The generated Rust test from [gate_tests.rs](gate_tests.rs) and the pinned
`../src/bin/projective_law_gate.rs` enumerates every coefficient
tuple for monic degree-one and degree-two Mumford supports over `F_7` and
`F_11` on `y²=x⁵+x+2`. It retains exactly the pairs satisfying
`u | (f-v²)` according to the repository's `Jacobian::valid` predicate.
The polynomial is square-free in both fields: a common root of `f` and
`f'=5x⁴+1` would have to be `x=-5/2`, but `f'(-5/2)=3141/16` is nonzero
modulo both 7 and 11. Every ordered pair is added by the compiled weighted
law with unit weights and with `(z,w)=(2,3)` and `(3,5)` on the inputs. Each
returned divisor is normalized and compared with the separate Cantor
implementation in `src/cryptanalysis/three_leads.rs`.

| Field | Valid reduced divisors | Ordered pairs | Weighted checks | Mismatches | Unresolved `None` |
| --- | ---: | ---: | ---: | ---: | ---: |
| `F_7` | 66 | 4,356 | 8,712 | 0 | 0 |
| `F_11` | 114 | 12,996 | 25,992 | 0 | 0 |

The [runner](run.py) copies the exact production source and appends the test
module into a temporary auto-discovered Cargo binary, compiles it, then removes
the temporary file. The production source and its frozen SHA-256 remain unchanged.

The [raw cargo log](cargo-test.log) records exact branch labels, counts, and
test status. [RESULT.json](RESULT.json) pins the law source, the Cantor oracle
source, and the log by SHA-256. [branch-counts.csv](branch-counts.csv) is the
standardized per-branch table; [branch-counts.svg](branch-counts.svg) and
[REPORT.pdf](REPORT.pdf) render the same finite result. The validator checks
the count totals, source hashes, smoothness arithmetic, CSV consistency,
and report signatures. Reproduce from the repository root with:

```sh
python3 research/projective_law_followup_20261022/exhaustive_small_field/run.py
python3 research/projective_law_followup_20261022/exhaustive_small_field/render.py --check
```

This establishes complete coverage of those two finite model state spaces
under the specified two weight assignments. It does not establish a theorem
that every compiled Rust branch agrees with the published formulas over all
odd fields, nor the smooth-projective divisor/Pic⁰ class equation. The
published-law source audit already identifies a specific non-unit-`W`
historical misreading of author Magma's `WW1` alias. The author source
sets `WW1=W1²` and uses it in `N6`, matching Rust; the W¹ alternative is a
synthetic mutation control.
The frozen 1,260-target collector operation-cost ratios are unchanged.
