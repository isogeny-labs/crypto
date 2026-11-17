# L3: Mixed-fiber PKM systems with complete priced solving

Status: **draft, not running**. This is an experiment specification, not a measured result. The executable producer/verifier and checkpoints marked TODO in the card do not yet exist.

## Question and decision

Do heterogeneous PKM fibers reduce complete decomposition solving cost after controlling for base size, relation yield and algebraic preprocessing?

The three-leads experiment tested fixed two-summand tower fibers. Existing PKM degree studies and negative Kummer Fourier tests do not establish an advantage for a mixed-fiber m=3 system. This is a proposed local variant, not a new published algorithm or an assumption of extra relations.

## First discriminating experiment

Implement three-summand systems with separate prescribed fibers for the three summands. Use 3 primes, 3 independently generated curves per prime and 20 target points per curve. Train fiber choices on separate targets; freeze all holdouts and exact base identities. Start at N=8,12,16 before considering N=21. Enumerate small cells to certify full solution sets.

## Controls

Compare homogeneous fibers, heterogeneous fibers, matched random sets with the same rational point and sign-orbit counts, and a deliberately scrambled tower. Use full F4/msolve and the fastest available external solver on identical polynomial systems; retain basis order, all roots, filtering, allocations and failures.

## Acceptance and negative outcomes

Promote only a held-out reduction in complete root-and-lift cost with identical verified solution sets, and subsequently a lower cold one-target IC cost. A lower nominal solving degree, faster single successful root, more setup, or lost solutions alone does not pass. If mixed systems increase degree or total cost, stop that configuration and retain the negative result.

## Execution readiness

Freeze exact identities, seeds, protocol, runtime and resource budget before execution. Keep setup, online work and verification separate; preserve complete cold totals and failed attempts. Native Rust performs all mathematics. The card remains draft until code, independent checker and resume path exist; estimates are intentionally unknown. No cloud job has been started for this card.

Source: [https://www.iacr.org/workshops/pkc2016/troll.iis.sinica.edu.tw/pkc16/slides/6-1--Algebraic_approaches_for_the_Elliptic_Curve_Discrete_Logarithm_Problem_over_Prime_Fields.pdf](https://www.iacr.org/workshops/pkc2016/troll.iis.sinica.edu.tw/pkc16/slides/6-1--Algebraic_approaches_for_the_Elliptic_Curve_Discrete_Logarithm_Problem_over_Prime_Fields.pdf). This source motivates the mechanism; the specific proposed experiment above is this search round's design. See the [search report](../../research/new_leads_20261019/README.md).
