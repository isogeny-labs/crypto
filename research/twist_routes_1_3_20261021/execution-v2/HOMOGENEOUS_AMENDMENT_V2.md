# E25 independent checker repair, version 2

The v2 panel completed 48 requested cells plus its control row. Preserve all
engine outputs, resource exits, missing stock signature exports and unsupported
lex orders. Its complete-basis checker failed on Singular's multiple-variable
declaration/assignment syntax (`int i,j,k,ok=1`). The completion marker did
not override the parser errors: the native gate correctly left every affected
completeness check unaccepted.

Separate declaration from initialization. Reparse each successful engine's
unchanged raw basis and require exact equality with its saved parsed JSON.
Rerun only the corrected independent homogeneous and affine certificates,
including all S-pairs and generator inclusions, into fresh output paths.
Replay all exact affine assignments and retain the earlier infinity/saturation
results unchanged. Re-run the small positive/truncated/wrong-field controls.
Do not rerun or relabel old solver timing/resource measurements. Keep every
source/basis/result hash and old failure. Original fields, inputs, orders,
budgets and unestablished speed/complete-cost claims remain unchanged.
