# R126 handoff

Source and protocol were pushed at `a56e7b28ca9ededc8d7e62be2e4955b29a929445` before the accepted calculation. Exact N51 orders on the four tame-seven normalized x=0 places are -141, -78, 42, and 49 (base-place order y0/e1, y0/e3, y1/e1, y1/e2). Together with R125's simple-place orders, the full x=0 local degree-weighted N51 divisor contribution is -358. Result SHA-256: `40cb7e162ee63d0ff61c3774e2a1c1e0893b7a61e7b18d4a4fd32e3437318672`.

Independent F4-table/row translation and coefficientwise Hensel replay, changed-source rejection, byte replay, and Lean finite checks passed. The Rust release library suite could not reach tests because sparse tracked include fixtures are absent. Next use these local orders in the full normalized N51 divisor, while resolving other supports and infinity.
