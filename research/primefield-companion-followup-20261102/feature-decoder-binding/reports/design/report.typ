#set page(paper:"a4",margin:17mm)
#set text(size:10pt)
= Repaired program: preserved decoder binding
2026-11-03 · Software API / recorded-oracle checks · Fresh source cases: zero

*Integration.* The original recovery path ultimately calls the evaluator in
target-aware-decoder/compiler.py. This module is preserved byte-for-byte.
The adapter binds the verified repaired coefficients and supplies the missing
legacy metadata: field479, window16, complete prefix0..15 and degree3.

#image("pipeline.svg",width:100%)

*Public boundary.* The decoder receives an immutable program and offset oracle.
The four recorded oracle fixtures retain public points, signs and call logs;
expected scalar labels stay in the checker. They correspond to O,P,2P,−P on
E/F509, subgroup order479. This is recorded-source API replay, not fresh
elliptic-source computation or a new recovery panel.

#table(columns:(2fr,2fr),inset:5pt,
 [Local exact checks],[Outcome],
 [Recorded public oracle cases],[4 passed; full prefix queried once in order],
 [Program mutation],[Rejected before oracle calls],
 [Malformed sign values],[bool,478,float,None rejected],
 [Preserved synthesis entrypoints],[Guarded against calls],
 [Factor reload],[Same coefficients; all479 identity points verified],
 [Test methods / attempts],[3 passed / 1])

*Accounting.* Byte hashing, JSON parse, support scan and fresh view allocation
are additional native work on every decode. Original decoder field counters
remain separate. No complete-cost or timing improvement is measured. Existing
one-CPU local test used512MiB/30second limits and exited0; no new cloud job.

*Full cohort.* The separate correctness plan retains4,563 targets on all
three original prime-field curves and all three arms. It is specified, not
frozen or launched. Source/runtime/runner/protocol publication remains needed.
It does not claim cold compiler cost while consuming precompiled programs.

*Admission.* Live NodePool read was unauthorized and namespace quota read
Forbidden under the configured EKS identity. Overview pool values are compiled
defaults. Iso Lab requires actual limits/quota readback; no role or quota was
changed. The four local API cases do not replace or shrink the full cohort.

*Sources / next.* Original recovery runner, public Oracle/guards, preserved
evaluator, corrected-span certificate and Iso Lab instructions. Finish the
fresh-source runner and exact runtime manifest, obtain authoritative admission,
freeze the complete protocol, then submit the full cohort. See DERIVATION.md,
contract-check-receipt.json, admission.json and validation-plan.json.
