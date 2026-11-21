# Typed resultant-class composition

The alternative projected geometry now accumulates main/tail resultants and the order-four square-class correction before one terminal projection. A modulus-bound ResultantClass keeps its field representative private, supports multiplicative operations only, and rejects mixed-field composition. Existing projected-resultant methods remain compatible wrappers.

Build plus ten selected native methods pass; each invocation filters 91 others. New checks cover 260165 field-element pairs across F5,F7,F13,F29,F509, product homomorphism and kernel-equivalent representatives, plus squaring/negation/noncanonical inputs/zero/context rejection. Test-only thread-local projection counting gives exactly one on every valid geometry case: 2500 synthetic cases,1349 valid,1151 rejected. Previous 1080 determinant (69 zero),245 root,245/980 accumulator, and paired-source checks remain passing. [pcc:unverified]

Generator replay reproduces tested bytes. Native build/test and input-lock checks had no failures. The compiler/binary hashes, transitive input hashes, exact commands and timeouts are in build-environment.json,source-lock.json,receipt.json and logs. Build cap90s; selected major tests60s,others30s; one local invocation. No timing or total-cost panel. Original4563-case cohort unchanged.

Use a disposable checkout to replay generate.py then run-record.py from this directory, invoking both from repository root. The recorder overwrites outputs. The report PDF was rendered and inspected. Sources and exact algebra/cost boundaries are in DERIVATION.md. Evidence is unsigned producer validation, not independent review or an execution proof.

PCC-20261107-7f88ba binds unsigned producer attestations.
