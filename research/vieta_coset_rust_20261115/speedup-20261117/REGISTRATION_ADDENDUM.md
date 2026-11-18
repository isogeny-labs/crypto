# Field-selection construction wording correction

The immutable selection protocol's construction-backend factor includes a
stale sentence saying finite candidates build no unused point table. The frozen
worker actually builds the full point base for every arm, as visible in
native/src/main.rs worker(). Its construction_work and cold_ns charge it in
full. Lookup additionally constructs its HashSet. No cost was subtracted or
amortized. This wording mismatch is retained and disclosed rather than editing
the frozen protocol. New transform protocols describe the actual full-base
construction. It affects the interpretation of the earlier narrative, not the
unchanged raw timer boundaries or the exact-output comparisons.
