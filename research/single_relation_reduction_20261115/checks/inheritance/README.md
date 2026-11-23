# Retrospective assembly probe

The probe was written and run after the original proof builds. This ordering
does not satisfy the skill's requested pre-edit gate; do not label it as such.
It applies existing generic laws and the two actual semantic endpoints without
copying their proofs. The output explicitly prints the standard transitive
axioms of the three central coverage results and all seven secant laws.

Source: crypto-lean867527c752375f21d47c000044d8215d07e432ba, with the same
Lean4.35.0/Mathlib cfc55fc pins as the line-laws canonical receipt. Command
from that built shared checkout:

    lake env lean /workspace/scratch/single-relation-inheritance001.lean

The preserved probe source is inheritance001.lean.txt; copy it to a .lean path
for replay. Exit0. No numerical case, proof limit or trusted native evaluation.
