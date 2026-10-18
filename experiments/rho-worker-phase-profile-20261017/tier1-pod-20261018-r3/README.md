# Frozen longer Tier 1 worker panel, attempt 3

The first two 32-walker panels completed every exact-output pair but failed
their frozen A/A timing gates (three and one out-of-range controls). This
attempt changes the charged workload to **128 consecutive walkers per arm**
to test whether longer intervals pass the same timing gate. It uses the same
32 public Q points, four alternating repetitions, serial/batch comparison,
serial/serial and batch/batch A/A controls, source bundle, Linux executable,
CPU placement, and memory rule. The plan still has 96 cases, 384 pairs, and
768 arms. All failed, noisy, and successful arms remain in the ledger.

The complete [plan](attempt3-plan.json) was frozen before dispatch at SHA-256
`7b56decce137bf6dba398036a93133a3eb9ba3a1345ccca9ff2f593fb23fb8b6`.
The [allocation receipt](attempt3-allocation.json) was frozen at SHA-256
`b9b53f3f03d8f65f9736f4a5719095d5d35af848c305e32864a5729c71bda7b6`.
The executable and source bundle remain SHA-256
`1bbbef507589180facc928ef6689bc474adadce5b3389a45d5116828479ef2a3`
and `ccc4c83128510e145bed6e8c04b26eb9a63f617bb2d3665826856ccd91902904`.

The independent auditor will require every exact-output pair, every Tier 1
host receipt, all A/A ratios in `[0.90, 1.11]`, extra peak RSS within both
10% and 64 MiB, and a Q-cluster bootstrap 95% lower charged-time ratio above
1.05 for default worker routing. Any failed gate leaves the aggregate ratio
and interval unknown. The longer workload is a new measured condition; it
does not revise or erase the prior two failed attempts.
