# Eighth Y7 coset: source correction before build02

The first native unit run, retained in `evidence/native-tests-01/`, finished
with **27 passed and 5 failed**. Every new B677 test stopped in fixture
construction because the derived eighth coset did not match the new source's
expected constant. The failure was not an extraction result.

The two historical spellings were already known to disagree. The initial
implementation provisionally used `1852955275`, found in the later
`examples/prime31_coset_cubic_factor.rs` and `prime31_coset_signed25.rs`, and
required an exact derivation check. The earlier
`examples/prime31_coset_xlift_f4.rs` instead gives `1852957275`.

The independent native diagnostic in `controls/coset7.rs` computes the same
seventh power by seven repeated modular products and by a separate addition
chain. Its immutable build and execution receipts are
`evidence/coset7-build-01/` and `evidence/coset7-check-01/`. Both computations
returned:

```text
p = 2147483647
y0 = 136769428
y0^7 mod p = 1852957275
```

The check exited 0. Thus the earlier spelling is correct, and the later
`1852955275` is false. The new fixture constant and independent assertion now
use `1852957275`; the wrong value remains an explicit negative assertion.
No historical source, archived point, target-selection rule, equation, budget,
or assertion count was changed.

Before making the correction, exact copies of the first-build fixture and
control sources were saved under `evidence/source-snapshots/build-01/`. Their
SHA256 values match `evidence/build-01/pre-command-sources.sha256`; the snapshot
receipt explicitly records that these were direct copies, not reconstructions.
The original failure and diagnostic records remain unchanged.

The corrected implementation requires a new immutable build and a rerun of the
same 32 native unit tests. This source note does not claim that the rerun passed
or that any registered B677 extraction call executed.
