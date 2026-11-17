# Initial character seeding

Optional Rust candidate initialization that uses the initial public-source
character before phase-polynomial construction. See DERIVATION.md for the
identity, assumptions, source contract and costs. The original cohort and
predecessor adapters are unchanged.

Run the deterministic software qualification from this directory:

```sh
python3 generate.py
rustc --edition=2021 -O -C overflow-checks=yes --emit=link,dep-info driver.rs -o /tmp/seeded-image-check
/tmp/seeded-image-check
/tmp/seeded-image-check --inherited
```

Tests compare materialized states to a separately formed polynomial gcd with
Z^L-sign, check true/wrong signs and zero exclusion, and exercise recovery from
public points. Scalar enumeration appears only in fixture checks. The adapter
retains source extraction and point verification. No timing panel or cloud run.

Evidence: validation01/receipt.json, RESULTS.md and report.pdf.
