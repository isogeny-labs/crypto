# Broader recorded-census scope check

Preparation,30October2026. The fixed lower strict-splitting rule is to be
checked on every recorded prime-order seed position in the earlier census:
69positions,82certified branches. This is a retrospective analysis of
existing certified records, with all labels visible. Actual cohort arithmetic
has not started at this preparation commit; no result is asserted here.

[RUNPLAN.md](RUNPLAN.md) fixes the original691rule,100–5000sensitivity range,
branch handling, native arithmetic checks, independent verification and three
corruption refusals. [sources/provenance.json](sources/provenance.json) pins
the exact upstream sources. The certificate file hash equals the one bound
by the earlier acceptance analysis. flatten.py only joins labels and copies
fields; it performs no elliptic-curve arithmetic or statistical calculation.

Native preparation builds without warnings and both bounded toy controls pass.
The F17group control is independent of the production census. PARI's installed
primary documentation for isprime and kronecker was checked: isprime is a
proved yes/no decision routine, not ispseudoprime; no standalone prime
certificates are claimed. Earlier primary-source interpretation and attack
limitations remain in the mechanism note linked from RUNPLAN.md.

After this source freeze, execute:

```sh
sh research/nist_skip_criterion_20261025/broader_census_reanalysis_20261030/run.sh \
 /workspace/nist-broader-census-build-001 /workspace/nist-broader-census-run-001
```

Keep all outcomes, all branches and every cutoff. No historical decision,
attack, timing or sampling inference follows from the resulting counts.
The60000random-curve baseline remains unrun. No S3checkpoint or upload is
claimed; analysis uses AGENTS9a's explicit existing-results development
exception. The previously denied old paper payload remains outside this work.
