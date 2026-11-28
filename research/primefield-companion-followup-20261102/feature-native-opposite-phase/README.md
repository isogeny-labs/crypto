# Opposite-radius parity supports

T-1878 extends the direct-phase recovery instrument with a separate paired-radius initializer and public recovery adapter. See DERIVATION.md for assumptions, exact orientation, costs and the next construction; RESULTS.md for retained attempts. Claim PCC-20261107-6848bb is a producer attestation, with independent signed verdict pending. [pcc:unverified]

Reproduce from this directory into a fresh directory:

```sh
python3 validate-recovery.py /tmp/opposite-phase-replay
```

The validator compiles Rust with optimization and overflow checks, emits dependencies, limits native execution to 60 seconds, and captures compiler/stdout/stderr plus input hashes. Rust performs all field and curve arithmetic; Python only orchestrates. No random seeds, cloud resources or timed comparison are used. The executable is excluded from Git; rebuild it with the pinned compiler/source. Generation pins the direct-phase predecessor and preserves it. Run generate.py only when deliberately regenerating copied sources; it does not run validation.

validation01 passed arithmetic and public recovery, but its subsequent-update counter remained attached to the predecessor counter. validation02 corrects that counter and adds domain/fallback controls. The changed sources from validation01 are retained with the validation01- prefix; compare their hashes to that receipt when replaying the first attempt. All other native inputs are unchanged. Full Cargo historical fixture/deadline failures are documented in the preceding session log; the standalone checks here do not resolve them.

Runtime opposite.rs and opposite-adapter.rs contain no root enumeration. Enumeration in opposite-checks.rs is confined to supplied finite reference fixtures. Public panels use E/F_257: y^2=x^3+x+7, P=(1,3), prime order r=281, m=5, radii [1,280,2,3], all k=0..280, caps 64 and 16, extra offsets 0..16 and call cap 128. Neither original cohort nor earlier adapters are changed.
