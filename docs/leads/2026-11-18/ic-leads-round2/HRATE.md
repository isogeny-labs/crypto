# L2-HRATE: hit rate of the k = 5 sum table against the generic model (closed)

[pcc:unverified] I read the `ksum5` arm records (non-warm-up) in `research/ic_ksum_sibling_20261118/sessions/P{20,22,24}/records.jsonl`. H is the measured relations per trial divided by 2E/(r − 1), where E = `table_insert_attempts`. The error is √relations / trials / expected.

| curve | r | E | trials | relations | H |
|---|---|---|---|---|---|
| P20 | 1,046,999 | 2,016 | 255,033 | 952 | 0.969 ± 0.031 |
| P22 | 3,291,679 | 4,032 | 433,780 | 1,050 | 0.988 ± 0.030 |
| P24 | 10,933,753 | 7,392 | 823,668 | 1,136 | 1.020 ± 0.030 |

H falls inside the preregistered band [0.94, 1.06] at all three sizes, so the lead is closed. The table's x-classes behave generically, with no excess hit rate from the small prime abscissae. Counting accepted relations rather than raw hits slightly understates H; the risk the proposal names. The result confirms the hit-rate model that L2-INNER and L2-FREE rely on.
