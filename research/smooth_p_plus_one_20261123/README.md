# Smooth p+1 four-summand study

The current [refinement report](REFINEMENT_RESULT.md) ([PDF](REFINEMENT_RESULT.pdf))
describes the earlier Dickson cutoff, fixed-query controls and native F8191
recovery. Its results have producer-local, unverified standing.

- [Initial rank argument](REPORT.md) ([PDF](REPORT.pdf)): collapsed and distinct-image charts.
- [Initial native controls](NATIVE.md) ([PDF](NATIVE.pdf)): original encodings and retained failures.
- [Refinement protocol and argument](REFINEMENT.md).
- [Separately frozen recovery extension](RECOVERY_EXTENSION.md).
- [Native source](native/src/main.rs) and [evidence](evidence/).

The original reports and manifests retain their historical revision scope.
New evidence lives in `evidence/refinement_v1` and `evidence/refinement_v2`.
The [remaining cold-comparison problem](../../problems/refined-rr5-cold-comparison.md)
tracks production integration, complete cost accounting and independent replay.
