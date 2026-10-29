# Post-v8 target-identifiability diagnostic

Filed after v8 native/CAS output, before any v9 data. V8 reports full-system
rank9 recovery for the ordinary base and rank8/no full solution for the twist
base. Rank deficiency alone does not prove that the target log is ambiguous:
an unused factor-base variable could be free while the target is fixed.

Use the unchanged v8 RESULTS.json (hash in the new freeze). Reconstruct every
one of the18 mod823 systems, including the G anchor and all repeated witness
rows. Compute RREF, a particular solution and a full nullspace basis, verifying
all rows/directions. The target is identifiable iff every nullspace direction
has zero target coordinate. Verify identifiable target scalars on the public
curve. For an ambiguous target retain a direction and two distinct target
values consistent with the linear rows. Do not feed the known fixture scalar
or factor-base point logs into elimination. Do not add queries or change bases.

Independent GP uses matker/matinverseimage, checks all ranks and uniqueness
flags, and verifies each identifiable scalar with ellmul. Two separate control
matrices cover rank deficiency with identifiable and ambiguous target logs.
Local primary PARI help for matker/matinverseimage is checked before execution.

One Rust and one GP execution, each <=300s,CPU0,one thread,4GiB,L0; no performance
claim. Freeze source/binary/input before data and push Setec checkpoint gate.
Success distinguishes incomplete full-base logs from actual target ambiguity.
It changes no frozen v8 bytes or original campaign requirement.
