# Schur interface prototype — unexecuted

`schur.py` implements the formulas derived in
../companion-blocks/SCHUR_NEXT.md. Only syntax has been checked. No Schur
numerical experiment, generator certificate or recursive solver is claimed.

Import with companion-blocks and structured-block-kernel on the module path.
The caller supplies public full generators and explicit leading-block forward
and transpose solve oracles. Schur.apply uses two full-source applications
and one leading solve. Schur.generators returns width s+2, with2s+6 full
source applications ands+2 leading solves in each direction. Counters retain
these nested calls alongside the complete full-source operator ledgers.

A leading oracle's ZeroDivisionError becomes LeadingBlockObstruction,
explicitly leaving the full matrix rank undetermined. The intended next
control is invertible [[0,1],[1,0]] with a zero leading scalar block.

The source invocation counts do not price the supplied inverse oracle as a
constant-time operation. The next frozen panel will use explicit dense E
references to validate correctness/accounting, including singular/zero Schur
outputs, altered corrections and invalid leading blocks. A recursive backend
must subsequently replace those references; no general success or speed
bound is inferred from this prototype.
