# Public factor interface correction to map019

139 fails because inherited factor implementation is opaque at this public boundary. Do not import extra private bodies or change transparency. Existing OrientationDiagonal.factor_eval gives exact rational value at any point. Use Polynomial.funext over infinite Q and eval_mul/eval_C, then apply factor_eval AS STATED twice; scalar field identity closes the per-anchor actual instance. All rest of map019 chain and hypotheses unchanged.140 is a new readonly qualifier, not old source mutation or unchanged rerun. [pcc:unverified]
