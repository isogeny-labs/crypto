# R125 handoff

R125 translates the actual N51 coefficient series along all eight x=0 local branches. Source/protocol were committed at `09ae11fc5` before data. Result SHA-256: `c9606bafa00f11756f7c4a84d0738033fb9337e8ea7028933e4ba7902fc3b795`.

The four simple normalized places have exact N51 orders `-30,-21,0,0` in base-place order y0/e1, y0/e3, y1/e1, y1/e2. The four tame-seven places each have an exact leading cancellation between correction powers 0 and 7, with no source-precision competitor. Next, lift the seven-edge root `W=1` over the local parameter `t` with `s=t^7`, then evaluate N51's first noncancelling term. This can be done place by place while preserving F4 conjugacy.

Independent binomial/F4-table expansion, support hashes, changed-source rejection, replay, Lean finite claims and visual PDF inspection passed. The first negative-control fixture setup was incomplete; its isolated rerun passed.
