# H4 n29 instance-zero subgroup eligibility

The frozen `n=29`, instance-0 curve has order **536,895,358 = 2 × 1,889 × 142,111**. Its largest prime factor, 142,111, is below the protocol's `2^20` subgroup gate. The runner stopped at that gate before base construction, pair-index construction, or target lookups. The recorded `error` status is an eligibility rejection caused by the explicit assertion, not a lookup failure or a measured zero-yield arm.

[`protocol.json`](protocol.json) and [`run.py`](run.py) were committed as `dd9a5136db9` before execution. [`result.json`](result.json) retains the original process outcome and 242,417,664-byte peak RSS. [`audit.py`](audit.py) independently reconstructs the exact curve and factorization through the checked Sage launcher; [`audit.json`](audit.json) records the gate decision. The checked Sage runtime receipt is [`sage-runtime-info.json`](sage-runtime-info.json).

The next frozen H4 instance, `n=29`, index 1, has order `536,895,614 = 2 × 268,447,807` in a separate read-only screen. It passes the prime-subgroup size gate and warrants its own precommitted base and lookup packet. This instance-zero rejection remains in the evidence set.
