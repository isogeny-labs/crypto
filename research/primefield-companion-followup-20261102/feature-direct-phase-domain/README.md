# Direct squarefree phase domains

Optional recovery initializer using direct positive phase factors from one
second-kind polynomial pair and the already-squarefree negative D_n domain.
See DERIVATION.md for assumptions, retained canonical validation and costs.

From this directory:

```sh
python3 generate.py
rustc --edition=2021 -O -C overflow-checks=yes --emit=link,dep-info driver.rs -o /tmp/direct-phase-check
/tmp/direct-phase-check
/tmp/direct-phase-check --inherited
```

These are deterministic software correctness controls, not a timing panel.
Source scalars and root enumeration are fixture-only. Runtime uses the existing
public-point source and preserves its capacity fallback and point verification.
The original cohort and prior evidence remain unchanged.
