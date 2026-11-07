# Prior work and claim scope

Primary source: exact native recover_boundary_impl.rs, solve4 lines98-116, with modular multiplication and inv helper. Earlier published EliminationWord and EliminationField prove suffix subtraction; the present producer uses their Row/Vector/residual/word definitions and WordComparator arithmetic facts.

Primary mathematical documentation read 2026-11-03: https://leanprover-community.github.io/mathlib4_docs/Mathlib/Algebra/GroupWithZero/Basic.html . It describes groups with zero, inverse cancellation and the convention that the inverse of zero is zero. This supports the explicit nonzero-pivot boundary and frames scaling as standard field algebra, not a new linear algebra mechanism. Documentation is current background, not a version pin; actual proof dependencies are the local hash-bound cache inventory.

New source correspondence: native normalization captures unit once after pivot selection and swap, and multiplies row entries from col through RHS. Earlier entries are untouched. Prefix zeros make this equivalent to uniform row scaling. The mathematical field inverse is proved to give pivot1 and preserve the equation. The native inverse implementation (pow(x,p-2,p)) has not been proved here; canonical unit and equality of its decoding with the field inverse remain explicit inputs. No conclusion assumes the desired output equation.
