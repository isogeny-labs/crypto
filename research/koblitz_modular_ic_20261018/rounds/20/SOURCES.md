# Round20 sources and interpretation boundaries

Scientific input is frozen Round19 at11b2f23f97195b5e3eda5f5764d46a1dd7818f92:
actual G25 CM rows/weights/point tuples,G21 refinement,G17 torsion bases,
G16 diagonal data,G15 class data,CM17/22 incidences and the Q32 source DAG.
The operational parentef4f01 only merges main;study files are unchanged.
All inputs are copied into the raw archive and hashed. No external database
record or unverified genus/modularity claim is introduced.

Primary mathematical references read for the written bridges:

- Moonen,ChapterVII,Duality,Corollaries7.22–7.23:Pic0 is the polarization
  kernel and inversion acts by inverse on Pic0 classes.
  https://www.math.ru.nl/~bmoonen/BookAV/DualAV2.pdf
- Moonen,ChapterXI,Polarizations and Weil pairings,11.1–11.4:the line-bundle
  class/polarization distinction. The actual Pic2 correction is checked,
  rather than inferred solely from equal Hermitian matrices.
  https://www.math.ru.nl/~bmoonen/BookAV/PolWp.pdf
- Earlier frozen descent/Riemann–Roch references are retained with their
  exact scope:Moonen8.10–8.12 and9.11–9.14.
  https://www.math.ru.nl/~bmoonen/BookAV/ThetaGr.pdf
  https://www.math.ru.nl/personal/bmoonen/BookAV/CohomLB.pdf

The integral recovery,projector equations,degree tables,full elliptic
factor lists,finite section values and source elimination are new native
calculations. Their self-contained written proofs are in THETA.md and
SATURATION.md. The explicit curve-chart recovery argument uses the actual
pole orders2 and3 and the ratio x/y at infinity. The proper image is a
closed equalizer,identified functorially by the computed retraction.

Lean formalizes actual finite CM/bit/gate/basis certificates and universal
algebraic implications,not full divisor,group-scheme,Galois descent or
saturation semantics. PROOF_MAP.md names the missing bridges. Native search
exhaustion,field implementation and evaluation rank are not relabelled as
kernel-checked theorems. No independent-host or external-CAS replay is
claimed. The user retains the IC efficiency assessment.
