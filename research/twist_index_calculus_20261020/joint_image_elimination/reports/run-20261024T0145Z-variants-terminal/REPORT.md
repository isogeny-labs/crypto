# Six native representation attempts: terminal outcomes

`variant-regression-v1` is terminal, with all six bounded attempts retained.
Inputs and the frozen runner come from corrected exporter `verification-v8`.
Every input is p37/seed0/query2. The two pure successes have independent
signed-set/RREF audits; allocation-censored mixed attempts have no invented
physical yield, rank or recovery result.

![Representation and physical verification](../../figures/joint-mechanism.svg)

| Arm | Encoding | Recorded exit | Physical-set verdict | Rank / columns |
|---|---|---:|---|---|
| Pure L0 | Full symmetry, finite | 0 | complete, independently matched | 1 / 3 |
| Pure L0 | Full symmetry, geometric | 0 | complete, independently matched | 1 / 3 |
| Phi | Implicit inverse, finite | 250 | allocation failure; no complete set | unknown |
| Phi | Implicit inverse, geometric | 250 | allocation failure; no complete set | unknown |
| Doubling | Implicit inverse, finite | 250 | allocation failure; no complete set | unknown |
| Doubling | Implicit inverse, geometric | 250 | allocation failure; no complete set | unknown |

All four failing stderr logs report allocation failure and the watchdog's
core-dump message; core size was disabled. Exit 250 is preserved as emitted
through the wrapper, not recoded into a successful empty result. There is
no complete root summary for those processes. The unchanged limits are one
thread, 8 GiB, 1,180 s cooperative and 1,200 s outer, under the busy guard.

Full symmetry repairs the pure baseline's weak representation on this
fixture. In contrast, the source-block formulation completes for all three
arms and matches their physical oracles, with ranks 1/3, 2/3 and 1/3 for
pure, phi and doubling respectively. This does not establish which method
is fastest on a population or complete collector. No admitted timing,
end-to-end price, GAE band or matched rho claim follows from these runs.

The requested extension now attempts all 297 construction positions using
the same frozen exporter, reusing the three completed regression exports
instead of recomputing them. Construction success does not replace complete
roots, exact physical sets, collection through rank or full cold costing.
