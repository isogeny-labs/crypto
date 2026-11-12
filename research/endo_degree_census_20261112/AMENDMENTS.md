# Amendments to PROTOCOL.md (dated; PROTOCOL.md itself is frozen by hash)

## 2026-11-12, before the run

The first smoke run (tiny sizes, output not kept, not part of any result) showed
that population (a) contains lab-constructed CM curves in bulk, so rule 2 of the
decision rule, read literally over (a) union (b), can fire on lab-built curves
alone. The frozen rule is unchanged and is applied as written. Two reporting
additions, which change no test, no population and no threshold:

1. Rows in (b) carry their group (`b:named_standard`, `b:pairing_friendly`,
   `b:other_named`) and the rule-2 candidate list is printed twice: over
   (a) union (b) as frozen, and restricted to (b).
2. A sub-population `a:registry_prime:distinct(p,t)` (first registry row per
   `(p, t)`) is added, because many registry rows are different models of the
   same `(p, t)`.
