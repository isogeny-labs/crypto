# Split-graph degree continuation

Full theorem and limitations: [SECANT_GRAPH.md](../SECANT_GRAPH.md).
Illustrated report: [REPORT.pdf](REPORT.pdf), [source](REPORT.typ).

The derived two-summand homogeneous degree is **5 to 3** on the exact
four-abscissa split-graph fixture; the matched nongraph secant control has
degree **4**. These are top-ideal degrees, not measured F4 solving degrees.
External novelty and independent review remain open.

- Curve: `icv1-fp5-tm1-a77cc6af`; exact model and full ICV1 in [fixture.json](fixture.json).
- Candidate: `FB1hed5b19904cc8`; control: `FB1h66fbb7ed3fb6`.
- [identity.json](identity.json): `IDC1h621ed1a51b5f3ca8`, repository v1
  evaluated certificate plus exact integer expansion in the checker.
- [homogeneous-degree.json](homogeneous-degree.json): all cubic upper
  identities and a quadratic lower dual witness, separately for every sign branch.
- [target-10-6.json](target-10-6.json): complete candidate/control systems,
  four branches each. The control includes its exact membership polynomials.
  This is exported input, not a completed target-population run.
- Factor-base dumps use the existing `ecbench.factor_base_dump/v1` and FB1
  preimage, with one column per x and canonical lower-y sign coefficient 1.
  Registry and EC1 joins are explicitly pending; no accepted measurement row
  or leaderboard promotion is claimed.

Build the locked native crate, then run:

```sh
native/target/release/secant-graph-certificate verify secant/identity.json
native/target/release/secant-graph-certificate check-degrees secant/homogeneous-degree.json
lean proof/SecantGraphTop.lean
```

Run these from the parent study directory. Full commands, checks, hashes and
retained failures are in [VERIFICATION.md](VERIFICATION.md). The old root-library
attempt remains incomplete for the separately recorded missing-fixture errors.
No new research or performance experiment ran: S3 checkpoint admission still
fails. The next experiment remains the complete all-target/root/group/F4 audit
specified in the theorem's final section, followed by a preregistered size ladder.

The canonical numerical IC/rho leaderboard, progress timeline and benchmark
tables are unchanged: this continuation contains an algebraic proof and fixed
certificate artifacts, with no measured yield, speed, or admitted IC/rho result.
The new component's exact comparison is shown in
[degree.svg](degree.svg).
