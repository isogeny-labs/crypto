# Actual torsion constructor branch repair, version 2

All five version-1 constructors reached the 128-trial cap without emitting
a basis. Preserve every GP error, native rejection and empty certificate.
The cause was the initial-point conditional: its `next` was the else branch,
so every later point skipped the pairing test. Move `next` into the first-point
initialization branch. Retain the original seeds, selected curves, extension
degrees, trial count, time/memory caps and all independent checks. Repeat into
fresh v2 inputs and outputs; the failed search is not evidence against actual
torsion or gluing on those curves.
