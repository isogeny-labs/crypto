# C63: bijective quadratic lifting, field-specific proof and measured memory

The fresh hypothesis trades degree and term count against added variables, preserving a bijection onto every ordered original polynomial root. Source/input hashes, training fixture, coefficient cases and budgets were frozen before execution. Three experiments ran through checked resume/run-ready; all retained90s/4GiB/onecore limits. Sources,original input and all prior attempts remain unchanged.

## Binary result

The implementation proves and independently checks the two-link ordinary binary chain.3152 original assignments,23776single auxiliary-bit corruptions and576full augmented assignments passed,including zero coefficients,repetitions,GF2 and odd/even binary degrees.24full-width residual receipts agree with literal original arithmetic; all6known original roots extend correctly. The n83,d18 encoding has272variables,301equations,max degree2 and27558term occurrences versus137variables,166equations,degree3 and124560terms. Thus the term ratio is0.221243; the preregistered <half prediction passed. Input SHA256356e9f12aff3827098ea8b3359c93c2146ab068eb05085b61b9753eb2a277f0b is bound separately from expected roots.

## Changed engine diagnostic and memory

The worker regenerated the encoding,byte-compared frozen input,checked25source/binary/library bindings,and charged0.307349s encoding in the diagnostic. On this changed encoding,genuineMSolveF4 exhausted5s (5.041290s charged child lifetime,wait4 peak486196KiB),referenceF5B exhausted5s (5.049070s,31352KiB),and source-derived inheritedF4 aborted on allocation under its3GiB child address-space cap (5.491390s,2587584KiB). Its stderr explicitly reports a failed49152-byte allocation. It is notF6. All returned original-root lower bounds are0 and all completeness flags are false. No timeout ratio,no hard-instance qualification and no end-to-end gain follow. The narrower representation did not produce a completed solve in this budget; added-variable memory risk is observed. The exact cause of internal fill-in was not instrumented.

Per-child wait4 high-water RSS is now recorded independently,along with child CPUtimes,supervisor memory and charged lifetime. These are development measurements,not qualified isolation or simultaneous aggregate peak. The engine child cap3GiB sits inside the declared4GiB worker budget. No larger-memory unchanged retry is queued.

## Odd fields

A separate no-division identity uses P=ab,V=(a+b)u,W=(a-b)u to quadratize the odd-characteristic S3 equation. It applies in characteristic3; a final fixed-target link needs only one further product. Independent literal/unfactored comparisons passed33698original assignments and101094auxiliary corruptions overF3,F5,F7,F9,F25. Full3^6augmented enumeration(729states) matched the original root set. The five requestedNIST prime moduli passed24identities each with explicitly artificial coefficientsA=-3,B=7. These120checks are arithmetic identities,not standardNIST-curve solving,hardness,all-root completeness at large scale or performance coverage. The binary and odd formulas remain separate.

## Decision and next bounded experiment

The representation prediction passed; the performance hypothesis remains inconclusive and the inherited-memory expectation failed. New hypothesis: eliminate the constant31-row r->high(z*r^2) block analytically,then Gray-update the residual48x52matrix and low-degree Gold constraints without adding product variables to a genericGB solve. NEXT_EXPERIMENT.md derives the rank,updates,retained rank-defect obligations and oracle/timing plan. This targets actual candidate cost rather than increasing an unchanged timeout. Odd coordinate encoding/domain predicates and global quintic whole-cell search remain separate active tasks.

PRIMARY_REVIEW.md records inspected primary implementation documentation and limits. Product-variable degree reduction is standard; no algorithmic novelty is claimed. The full twenty-hard-instance,many-field,complete genuineF4/F5/F6 repeated/memory gate and the user's8xgoal remain unpassed. The immutable2xrelease floor is unchanged; qualified gain is null.
