# Round06: modular kernel construction

Original goal active; no novel efficient arbitrary-curve map established. Literature already read: original LLL1982 sections1–2, FLINT3.3.1 polynomial GCD source and repository CT84. KB wrapper again exits127; no fresh KB/novelty claim.

Existing traces settle the denominator-filter hypothesis for the uncovered order61 input: all22 rows reach the modular screen and all22 reject at every original precision under both metrics. No denominator-filter variant is justified by those bins. This is a trace observation, not a universal obstruction.

Test a materially different remedy for the order127 setup timeouts: construct raw division-polynomial rational numerators without intermediate exact polynomial GCD normalization. Recover the degree-d common factor via two conjugate split-prime projections, CRT of Eisenstein coefficient pairs and rational reconstruction. Accept only after exact division of BOTH original numerators and a good-prime coprimality certificate for both quotient factors and every relevant denominator. Modular fit alone never accepts a model. This is classical modular-GCD arithmetic, not a new transfer map.

Fixed setup calls u3/u5/u7,60seconds each, at most64 good split primes <=65521; expected degree d=(r-1)/6. First require byte-identical u3/u5 models and u3 qualification output against frozen round04 before u7. Keep every failure/timeout. No old input or cap is changed. Native C++/GMP/OpenSSL only; no Python crypto or paid run.

Higher-precision public-point qualification is a separate declared amendment after setup qualification; do not silently raise old precision/caps. Full native/CM/global-field interface remains outside Lean. Preserve all109 old scientific pins, prior raw evidence and published1233-row ledger prefix. Source/executable qualifiers and pre/post manifests required before calls.

Preparation clarification before calls: exclude the torsion characteristic r from CRT primes; its kernel coefficients can have a local pole even when raw numerator coefficients reduce. Preserve first compiler source snapshot before this correction.
