# T1944 streaming lifted character trace

## What was done (commands, files, features)
Read latest session/T1943 source and trace derivation. Primary source: Harvey-Sutherland, Computing Hasse-Witt matrices II, https://math.mit.edu/~drew/conm13352.pdf, section2 equation2 and remarks2.1-2.2. Coefficient recurrence is established prior work; denominators divisible by characteristic require precision lifts. Paper's many-prime average bound does not apply to our single target. KB unavailable: no MCP and wrapper no executable.

## Decisions made (and why)
Use a degree-d rolling coefficient window for f^h, lifting modulo r^(1+v_r(D!)), dropping precision exactly at characteristic factors of the step index. Accumulate only two trace sums. Expand word factors over subsets with an explicit subset cap; no scalar enumeration outside fixtures. This removes dense coefficient state but retains linear coefficient stepping and exponential subset count. Finite native compile90s/check60s, max4 revisions, steps500000, subsets4096, modulus<=i64MAX, existing source/node/correction caps. No cloud/timing ranking or original-cohort changes.

## Open threads (unfinished, pick up next time)
Native coefficient/trace/word/public checks, singularity counterexample, lift and budget refusal; proof/custody/report/claim/publication.

## Errors (what went wrong, how it was handled)
Shared scopes held by T1942; component journal used. No native run yet.

## What was done (commands, files, features)
Validation01 build/check passes.45 polynomial cases/7965 coefficients/90 character traces,60 precision drops; window<=8 and largest modulus6234839521.36 word systems:27 exact moment matches/9 caps,14 singleton phases; successful cases1265016 steps/15580180 modular products. Public29:12 recovered/11 endpoint/6 subset caps;332 nominal+27 guard calls,2654208 steps/38436864 modular products,19844 endpoint-correction field multiplications. Frobenius counterexample shows a direct mod-r differential equation can have another polynomial with the same constant/leading coefficients and a different weighted trace. Large step/precision caps refuse before recurrence work. [pcc:unverified]

## Decisions made (and why)
Retain this as a small coefficient-state construction, not a work improvement: subset expansion and linear stepping remain. Add explicit per-capped-word diagnostics to bind failed work rather than report only successful-case aggregates. Preserve validation01 test snapshot; production source unchanged.

## Open threads (unfinished, pick up next time)
Fresh validation02 diagnostics, derivation/report/claim. Shared scopes still held by T1942.

## Errors (what went wrong, how it was handled)
No compile/assertion failures. Validation01 summary omitted operation totals for capped word systems; add retained per-refusal rows before final reporting.

## What was done (commands, files, features)
Validation02 adds all9 refusal rows; both receipts/hash readbacks pass. One-page report inspected. Registered PCC-20261108-d2f2f7 after shared scopes cleared;513-entry chain verifies. Current session log updated. [pcc:unverified]

## Decisions made (and why)
Successful plus refused word work is2764200 steps/35982184 modular products. Keep separate modulus/precision and inversion costs. Derived fixed-observation bound scoped to word-only image identification. No whole-pipeline memory or speed claim.

## Open threads (unfinished, pick up next time)
Publish on existing PR3602; next matrix-product stepping and whole-word trace without subsets.

## Errors (what went wrong, how it was handled)
No native failure. Own component lease renewed during shared-scope wait; no conflicting writes. Artifact-only successor abfa135c3248 inspected and fast-forwarded.
