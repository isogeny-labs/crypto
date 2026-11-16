# Independent classifier source-fidelity review plan

Written before inspecting the new candidate proof. Producer states: actual UInt64 negation/subtraction expressions and inverse-first/equal-second/zero-sub-third classifier agree with the existing field classifier for prime5<=p<2^31 and four canonical inputs. Compilation and separate readback are producer checks, not yet independently accepted.

Derive expected obligations independently: p-b%p cannot underflow; a+neg(b) <2p avoids word overflow; field coercion must respect canonical equality; inverse branch must precede equal branch so simultaneous conditions (zero inputs) choose inverse. Derive subtraction-zero iff equality from field axioms and the embedding under explicit range bounds. Compare every conditional and return constructor with current Rust source and RuntimeSemantics. Audit primality/characteristic hypotheses and unused stronger bounds. Exclude cost counters, compiled codegen and uncomposed cross-scaled expressions from claim.

Inspect source hashes and proof assumptions; compile/readback in an independent output directory if existing pinned dependencies allow,120seconds per command. Check bad-domain/overflow or changed-order controls where relevant. No scientific execution or full-law totality claim.
