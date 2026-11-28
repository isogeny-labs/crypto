# Conditional doubled-order phase rescue

T-1879 adds doubled.rs and a separate recovery adapter. It recombines existing opposite-radius observations and attempts singleton phase recovery only after the original first-radius path proceeds to its opposite radius. On a miss it continues the original compressed state and history. Claim PCC-20261107-67816b is a producer attestation, independent signed verdict pending. [pcc:unverified]

Read DERIVATION.md for domains, formulas, no-extra-query argument and charged costs; RESULTS.md for complete fixed-panel outcomes. report.pdf is the inspected visual report, with editable report.py, report.typ and SVG files.

Reproduce into a fresh directory:

```sh
python3 validate-recovery.py /tmp/doubled-rescue-replay
```

The validator compiles Rust with optimization and overflow checks, captures compiler output and dependency hashes, and limits native execution to 60 seconds. Python orchestrates only. Source enumeration is confined to checker fixtures; the recovery helper takes three public observations from each orbit. The excluded check executable is rebuilt locally from its pinned compiler and sources.

The public panel is E/F_257: y^2=x^3+x+7, P=(1,3), r=281, m=5, all k=0..280. Radii [1,280,2,3], extra offsets 0..16, call cap 128, fixed degree caps [8,10,16,20,32,64]. At cap 8 both implementations leave 10 unresolved cases; all other panels recover all targets. This is a separate correctness panel; the original 4563-case cohort remains immutable.

validation01 passes on the first attempt. No compile/native/render failure, cloud job or timed comparison is introduced. Existing full-Cargo fixture/deadline issues and ledger divergence remain documented in prior session logs. generation.json pins copied predecessor sources; publication.json records hashes and byte counts for the bundle.
