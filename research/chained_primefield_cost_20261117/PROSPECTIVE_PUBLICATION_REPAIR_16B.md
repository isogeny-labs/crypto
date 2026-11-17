# Prospective source16B: publication completeness and failure bytes

First sourceA8f3d314c stops before copying,compilation or arithmetic: Cargo.lock
is ignored and absent from the published tree, despite its exact physical hash
being in SOURCE_LOCK_16.json. Preserve the whole first-negative packet and all
sourceA inputs. Force-add that same unchanged Cargo.lock; do not edit an ignore
rule, dependency, arithmetic expression, oracle, population or resource limit.
[pcc:unverified]

Separate metadata helper B selects SOURCE_LOCK_16B.json and writes complete
failed Git stdout/stderr/argv/exit before raising. A lost the first such stderr;
its later diagnostic check remains explicitly post-call. Separate shell B only
selects the new helper. All native source and all original16 criteria stay
literal. Check every execution-lock path against the staged Git blob before
publication, then publish/read back B before its first scientific call.
[pcc:unverified]
