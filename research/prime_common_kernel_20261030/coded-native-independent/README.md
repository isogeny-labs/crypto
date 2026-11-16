# Independent fixed a3 syndrome-kernel development unit

This standalone C++20/GMP unit tests the finite specialization a=3,L=8,
binary syndrome columns1,2,3,t=1,T=17. It is a bounded correctness check,
not a timing experiment, general theorem or improved ECDLP algorithm.
The prospective `PROTOCOL.md` was hashed before compilation. No previous
native implementation or native output was used to write this program.

The first strict-warning compilation and first execution passed all 33
checks. Raw exact results are in `run-001/`:

- 16 code slots, with each first orientation occurring twice;
- 1024 entries of four actual projected A tensor A matrices equal the
  paired-flip diagonal formula;
- 256 active-node entries equal U0;
- 128 crossing entries across two 8-by-8 sides have B/256 modulo 2 equal to I8;
- independent binary elimination gives rank 8 for both normalized blocks;
  exact rational Gaussian determinants both have valuation 64.

At T=17 the computed selectors (f*,f0,f1,f2) are

    (-12373218000,30990437521,-15348459520,-3268760000).

Their observed 2-adic valuations are (4,0,11,6). Every selector was computed
from the actual 16-node rational Lagrange products; no valuation was inserted
as a substitute for computing a selector. B/256 is integral with invertible
reduction, which certifies eight Smith orders 8 on each side. This is a
finite matrix certificate, not a general Smith-normal-form implementation.

The two negative-control types discriminate: replacing U0 with I causes 32
entry mismatches; omitting the f*²I contribution gives normalized rank 0
on both sides. No build or execution failed in this unit.

Reproduce with `bash replay.sh`. It recompiles the pinned source, executes
the same fixed cases and compares all seven frozen output files byte for
byte. Source and protocol checks are also required. The binary hash and
actual compiler/GMP versions are retained; neither binary size nor execution
duration is used as a performance result.

Source-provenance detail: the candidate markdown was being edited by a
parallel mathematical analysis. `candidate-source.sha256` records the
version read before compilation. The complete `candidate-frozen.md` is a
later post-run snapshot, separately hashed in `candidate-frozen.sha256`.
It must not be described as a pre-run source snapshot. The immutable
prospective protocol fully specifies every numerical input to this unit.
This concurrent document update caused no test failure or input change.

The general projection identity has a separate Lean project. This native
unit does not formalize the valuation argument, prove density bounds or
construct a sub-exponential contraction/ECDLP algorithm.
