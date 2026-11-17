# Two-pattern irreducibility theorem, before data
Compile the new FibreIrred52 module against the accepted Closure52.olean
(no recompilation of that producer),and pinned Mathlib.Polynomial.Monic.
The first theorem proves polynomial irreducibility from the two degree
restrictions for every monic divisor,using the formalized pattern intersection.
It does not assume irreducibility itself. The actual fiber factor lists,
DVR integral specialization/degree preservation,and conversion from those
lists to the divisor restrictions remain external. The second theorem proves
the common divisor of7and10 is1,for the base constant-field argument.
Two declarations audited,zero failing/trusted/native_decide. Same Lean4.34.1,
mathlibd13f23b,-j1,-M4096,1200s separately compile/audit,no virtual cap,
serialized math. Freeze all affected source/runner before data,retain failures.
