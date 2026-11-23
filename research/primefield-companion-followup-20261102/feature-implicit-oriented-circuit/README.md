# Implicit oriented-mode circuit

T-1932 builds an arithmetic DAG for the complete retained three-mode transcript
without a dense phase polynomial. See DERIVATION.md, RESULTS.md and report.pdf.
This is a circuit builder, public observation collector and conditional phase
extractor; it does not solve for the unknown image Y. Three modes do not encode
all individual signs when m>3. Caller certifies prime fields, subgroup/order,
matching source and curve configuration, and exact root order for collection.

Runtime contains no scalar/root enumeration. Enumeration in checks.rs is the
independent correctness oracle. Circuit construction accepts at most4 radii,
m<=64 and an explicit node cap; public collection has a source-call cap.
Polynomial specialization accepts a supplied modulus and coefficient-slot cap.

From the repository root:

```sh
set -e
timeout 90 rustc --edition=2021 -O -C overflow-checks=yes --emit=link,dep-info research/primefield-companion-followup-20261102/feature-implicit-oriented-circuit/driver.rs -o /tmp/implicit-oriented-check
timeout 60 /tmp/implicit-oriented-check
```

Use fresh immutable output paths for replay. record.py records original paths;
it is producer metadata capture, not independent verification. Validation01/02
failures are retained. Final evidence is validation03. The supplied-image phase
checks cannot substitute for an image-root extraction algorithm.
