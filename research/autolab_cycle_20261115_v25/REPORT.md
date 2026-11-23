# Parameter-dependent duals recover a missed parity

The first hypothesis was refuted: iterating constant shared-dual restrictions stalls at dimension4 rather than the predicted3. Both actual low6prefix families retain16directions versus8feasible. Every feasible direction survived4096tiny pencils and4608affine domains. Worker0.280365470s,peak process RSS18660KiB; raw SHA25636889137a71fb5c5e1d6b88e45eca2b31c19b9715fbee2a4a3902351dc44a024.

Changed hypothesis: affine parameter-dependent left multipliers can cancel the inner matrix identically and expose guards invisible to constant duals. This succeeded. In both pre-Gold73 andGold69 families the225unknown multiplier system has rank213 and nullity12. Its nonzero guard image has dimension2. The first guard is L=t0+t1+t3; the other is t0L and adds no zero-set restriction. On u=33t0+2t1+20t2+8t3, L is exactly parity mask42, the missing u1+u3+u5 constraint. Together with prior masks20and33 it gives precisely {0,10,20,30,35,41,55,61}.

The sparse certificate uses only equation rows41and44(zero-based): lambda(t)=(1+t0)e41+(t0+t1+t3)e44. Recorded identities verify lambda^T M(t)=0 and lambda^T b(t)=L coefficient by coefficient in the Boolean ring. This is a proof on the fixed finite pencil. All4352tiny affine domains matched their complete feasible sets. Worker0.223956293s,RSS20212KiB; raw SHA256c52cb373487ad042f4bf6e7f9824d7204e53215721b1d7d2b7cb13d331ca32ef.

The third experiment covered all512exposed low9directions with8disjoint cells, fixing high3bits and leaving6free. Both families gave identical counts:

| high3 | constant candidates | variable-dual candidates | feasible |
| --- | ---: | ---: | ---: |
| 0 | 16 | 8 | 8 |
| 1 | 32 | 24 | 24 |
| 2 | 64 | 48 | 47 |
| 3 | 64 | 48 | 47 |
| 4 | 64 | 64 | 62 |
| 5 | 64 | 64 | 61 |
| 6 | 64 | 64 | 63 |
| 7 | 64 | 64 | 60 |
| total | 432 | 384 | 372 |

All372feasible directions survive;12extras remain unresolved by the guards. The cover controls exhausted4096tiny pencils/8192affine cells and matched full feasible sets. Worker0.836072933s,RSS21792KiB; raw SHA256b5823d6fefe80e13f12d4eae4c24649ad1f83b7aa5f18f11a972820c40c2a26f. Worker times include diagnostic oracles, not comparative production benchmarks. RSS is peak process memory, not aggregate cgroup memory.

A nonzero degree-r Boolean polynomial on6bits has weight>=2^(6-r). With2,3,1,4infeasible directions in the last four cells, any nonzero necessary guard there needs degree at least5,5,6,4. Thus quadratic guards cannot remove them. The induction proof is in the cover proof; this is the established Reed-Muller minimum-weight principle, not a novelty claim.

Portable REPLAY_RECORDED.py uses only the standard library. It independently reconstructs all512direction systems per family, checks every reported affine restriction, verifies multiplier identities and guard evaluations, checks complete cover accounting, and confirms every density obstruction. It passed. This is independent algorithmic verification, not independent external source admission. All3experiments were frozen before execution and ran through lab.runner resume/run-ready with90second/4GiB/onecore limits. Previous algorithms, inputs and evidence were preserved.

Next: PROPOSED_NEXT.md proves a common-constant-column reduction to a smaller residual matrix. Preregister the prediction that at least35of45rows can be removed on a shared block; test tiny complete inner solution sets, actual ranks and all paid costs. Failure must change the hypothesis. No fourth experiment ran this session.

Primary-source review: Courtois,Klimov,Patarin,Shamir, Efficient Algorithms for Solving Overdefined Systems of Multivariate Polynomial Equations, section3, https://www.nicolascourtois.com/papers/xlfull.pdf, explicitly uses bounded monomial multiplication and linearization. Our restricted outer-variable syzygy extraction is a structured instance of that established approach, not a proven novel general alternative to Groebner methods. The IACR-hosted PDF returned403; the author-hosted primary text was read. Reed1954 DOI10.1109/TIT.1954.1057465 was identified but the publisher returned a robot check; no full-text review is claimed. Specific sparse certificates and paid reuse merit further comparison, without novelty clearance.

The requested8x end-to-end target and unchanged genuine-F4/F5/F6 release gate remain open. No full339-variable original solve, source/ghost/chart closure, new odd-extension/NIST timing coverage or qualified speed ratio follows. These are binary component advances toward that goal.

To reproduce recorded evidence: restore exact v23raw and the3new raw files if repository artifact maintenance relocated them, then run python research/autolab_cycle_20261115_v25/REPLAY_RECORDED.py. Re-running workers requires fresh output paths and a new frozen plan; do not overwrite raw evidence. Internal experiment-management provenance remains separate from this public scientific checkpoint.
