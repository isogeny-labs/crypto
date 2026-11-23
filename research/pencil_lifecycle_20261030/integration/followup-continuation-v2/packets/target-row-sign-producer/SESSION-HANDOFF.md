## What was done
Derived the legacy constant-sign conversion, implemented an offline adapter, checked468 one-row cases,6 fixed systems,8 invalid inputs, and948 native Rust diagnostic outputs. Source inventories remained unchanged.

## Decisions made
No runtime change, actual collector stream, scalar certification, timing or scientific trial. Explicit homomorphism premise separates scalar-coordinate interpretation from source geometry. Independent review pending.

## Open threads
Review and real retained-stream binding remain. The shared journal is deliberately untouched while the Copilot-first merge request6541497185 is pending; this scoped handoff records the finished component meanwhile.

## Errors
No failing adapter/native control invocation. Rank checks alone do not detect the explicit scalar-sign counterexample.
