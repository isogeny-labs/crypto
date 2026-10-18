#set page(width: 21cm, height: auto, margin: 1.5cm)
= n = 31, m = 3: the curve effect does not replicate (2026-10-18)

The pre-registered arm (32 curves × 4 geometric V, seeds 1–4) gave H1 on
work_per_call: ICC 0.228 [0.032, 0.403]. A post-hoc diagnostic traced it to |F|.

We ran two replications.

- *Fresh seeds 5–8, same arm.* work_per_call ICC is 0.000 [0, 0.043], and |F| ICC is
  0.000 [0, 0.033]. Yield is 0.075 [0, 0.243], µs/call 0.017 [0, 0.170], and projected
  solve 0.094 [0, 0.253].
- *32 fresh V, |F| only.* ICC(|F|) is 0.000 [0, 0.007] at n = 31 and 0.000 [0, 0.009] at
  n = 19.

The grouping by depth and by Tr(a₆) was uninformative: only K_a sits at depth 0, and
every curve has Tr(a₆) = 1.

*Verdict:* inconclusive, not H1. The original estimate most likely reflected a
curve × V interaction across only four subspaces. Future arms need more V per curve,
or replication on fresh seeds before H1 is declared.

#image("icc_replication.svg", width: 100%)
