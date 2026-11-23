# Corrected bound and fixed-anchor saturation

V1 source3d5268c7819b/head015b005a5db9/checkpoint224218 ran once, exit101,
empty stdout, failing the per-Weil Bdegree<=3assert BEFORE scalar section,
roots or decoding. Retain RUN.txt/RUN.stderr/EXIT.txt and fiber-v1.rs. No
calibration pair was selected. This is a mathematical bound correction,
not a target/anchor/seed change. All original controls and inputs remain.

N has Adegree<=4 and Bdegree0. For hAdegree<=4/hBdegree<=1, N³h after reducing
y²=f has Adegree<=16, Bdegree<=13. Exact division by cubic uT³ yields
Adegree<=7, Bdegree<=4. Squaring/multiplication then yields section numerator
Adegree<=15, Bdegree<=11; norm A²-fB² has degree<=30. The missing Bdegree4
term is the reason V1's bound was false. These inequalities are derived
before examining a norm polynomial or any roots.

For P approaching Q or R on the ORIGINAL anchor branch, the divided-difference
N vanishes to order>=1. Each Weil numerator N³h/uT³ vanishes to order>=3,
so every section numerator vanishes to order>=6. The norm therefore contains
(x-q)^6*(x-r)^6, degree12. Divide EXACTLY by this known factor (the original
restricted denominator den=d^6) and require the quotient degree<=18.
This is fixed-anchor saturation, not removal of a chosen target root.
Candidate charts already exclude x=q,r. Independently verify complete rootset
and all regular direct values exactly as before; no anchor/curve/limit tuning.
Record raw and saturated norm degrees; all decoded candidates and exclusions.

V2 scientific output files RUN-V2.txt/RUN-V2.stderr/EXIT-V2.txt are write-once.
Freeze corrected source/binary and checkpoint/gate before one execution.
