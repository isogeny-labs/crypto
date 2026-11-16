# Next execution contracts (design requirements, not launch approval)

No scientific panel below has run. Before launch, encode the selected panel as
the lab's frozen `experiment.protocol/v1`, commit exact source/inputs, retain
the checkpoint and confirm a supported authenticated execution route. This file
is a development handoff, not a replacement for that protocol or a run receipt.

## Symmetric PKM

1. Finish correctness and equal-output checks on actual curve targets. Separate
   planted positives, naturally decomposable targets, certified refutations and
   capped/unknown results. Preserve omitted repeated-abscissa cases explicitly.
2. Compare plain-chain, symmetric, same-system exhaustive reference and pinned
   external msolve on equal systems. Include A/A, shape-matched nulls and positive
   controls that check successful extraction, not merely polynomial evaluation.
3. Freeze multiple primes, seeds, independent target generation, bounds and
   heldout selections before the measured panel. Use m>=4 for the substantive
   cost question. The recorded m=3,N=21 and m=4,N=20 cells remain unrun here.
4. Charge setup, target construction, F4, root extraction, lifting/signs,
   verification, failed trials, memory and unrestricted branches. A low F4
   multiplication count alone is insufficient. Keep clocks diagnostic unless
   the isolated host admits them.
5. Only a complete verified relation/LA/descent pipeline against same-target
   strong rho can support a whole-ECDLP comparison. Any growing degree or
   resource censoring receives its actual finite scope; neither proves an
   impossibility theorem for other presentations.

## Five-sum replay

1. Hydrate all eight P20..P48 sessions from PR2852 at its exact source revision.
   Verify every manifest hash and size before parsing; keep files immutable.
2. Build that source with recorded compiler/flags. Bind source, binary hash,
   host, input hashes and command; a build of current main is a different arm.
3. Run native `ecbench verify --replay-all` on every frozen session. Independently
   check scalar equations and recompute paired and aggregate costs at all three
   weights 0,0.1,1; verify k4 replication and the A/A records.
4. Report observed mismatches, rejected records, censored cells and unavailable
   data. Complete `VALIDATION.json` only when the required checks actually pass.
   A deterministic replay on this different host does not establish new timing.
5. New holdout performance measurements require their own frozen L2 protocol,
   five rounds, warmup and matched strongest-rho/A/A arms, through Isolab EKS.

## Hecke arbitrary targets

1. Import and replay the pinned class points, subgroup projections, rank rows,
   anchor and log certificates. Certify any coordinate change both ways and
   preserve signs, identity and subgroup action.
2. Freeze fresh public Q values after selecting the class/collector. Give the
   recovery algorithm only E,P,Q and reusable certified artifacts; keep secrets
   solely in the independent verifier. A Q manufactured from a known row is
   a positive control, not a natural-target result.
3. Compare direct known-log lookup, the bounded shift control, the proposed
   natural Hecke attachment, and same-target rho/BSGS with all setup charged.
   Include out-of-base targets, misses and a shuffled/corrupted-label refusal
   control. Freeze the sample count and resource caps before execution.
4. Score verified target recoveries, failures and total cold cost. Source rank,
   source count or component probes alone do not determine the result.
5. A Lean proof that attachment plus a row certificate implies recovery already
   exists here; the remaining mathematical work is constructing that attachment
   on actual curve points and bounding its complete cost.
