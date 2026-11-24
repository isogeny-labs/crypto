# Smooth p+1 four-summand study

The [refinement report](REFINEMENT_RESULT.md) ([PDF](REFINEMENT_RESULT.pdf))
describes the earlier Dickson cutoff, fixed-query controls and native F8191
recovery. Its results have producer-local, unverified standing.

The [production integration report](PRODUCTION_RESULT.md) ([PDF](PRODUCTION_RESULT.pdf))
adds the exact-source cofactor adapter, retains thirteen full-base resource
refusals, and records a separate planted positive integration test. No matched
cost conclusion is established. [pcc:unverified]

The latest [cofactor output report](COFACTOR_OUTPUT_RESULT.md)
([PDF](COFACTOR_OUTPUT_RESULT.pdf)) preserves two partial-RUR failures and adds
a conservative acceptance guard plus fixed parameter forms. Eight degree-16
full-set controls complete; five degree-32 controls time out. Nine Refined
regressions pass on their narrower chart. [pcc:unverified]

- [Initial rank argument](REPORT.md) ([PDF](REPORT.pdf)): collapsed and distinct-image charts.
- [Initial native controls](NATIVE.md) ([PDF](NATIVE.pdf)): original encodings and retained failures.
- [Refinement protocol and argument](REFINEMENT.md).
- [Separately frozen recovery extension](RECOVERY_EXTENSION.md).
- [Production adapter protocol](PRODUCTION_ADAPTER.md).
- [Production reproduction instructions](PRODUCTION_REPRODUCE.md).
- [Fixed-form output protocol](SEPARATING_FORMS.md) and [reproduction](COFACTOR_OUTPUT_REPRODUCE.md).
- [Native source](native/src/main.rs) and [evidence](evidence/).

The original reports and manifests retain their historical revision scope.
New evidence lives in separately versioned directories under `evidence/`.
The [remaining cold-comparison problem](../../problems/refined-rr5-cold-comparison.md)
tracks production integration, complete cost accounting and independent replay.
