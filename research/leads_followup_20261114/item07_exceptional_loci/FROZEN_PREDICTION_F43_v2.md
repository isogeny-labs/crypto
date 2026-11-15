# Frozen prediction v2 for F_43 (supersedes P3 of v1; P1, P2, P4, P5, P6 of v1 stay; v1 sha256 6553911c64d8772c8b2b8e0b846c160e83890718e3a4bd9681c7be163f48029d)
Written 2026-11-14 before any F_43 closure was computed (a first F_43 launch was stopped after 275 cells, its files deleted unread, when the integer lift of the cubic was found).

Lifted cubic (found from p = 19, 23, 29, 37, 41, 47 fits; verified without refitting on p = 53 and 59 samples, 0 false positives/negatives): [pcc:unverified]
 Q(alpha,beta) = 5377 alpha^3 + 10384 alpha^2 beta + 6548 alpha beta^2 + 1352 beta^3 + 27709 alpha^2 + 36028 alpha beta + 11364 beta^2 + 47435 alpha + 31524 beta + 26615,
 equivalently with A = alpha+3, B = beta-2:  10384 A^2 B + 5377 A^3 + 6548 A B^2 + 1352 B^3 - 84 (2B-A)(A+B), a nodal cubic with node at (alpha,beta) = (-3, 2).
 Secant slope m = B/A from the node: m = -2 gives dim 599, m = -5/4 gives dim 607, other Q points dim 602; Q and alpha = -3 together dim 600; alpha = -3 alone dim 603.
P3' (exact). For every valid (alpha,beta) in F_43^2 (nonsingular, nine S_3 coefficients nonzero, f(1) != 0), t = 1 plane, k = 6, D = 4:
 dim6 is in {599,600,602,603,607} iff alpha = 40 or Q(alpha,beta) = 0 mod 43, and then equals the value in data/pred43.json (76 valid points: 603 x38, 602 x35, 607 x1, 599 x1, 600 x1),
 whether or not the point is unit at k = 7. pred43.json sha256 e7257618f8c4f44ad8695abe0fe292a5549adbb1e1dc0002bb061d3af7546393. Scoring: number of valid points where observed class membership/dims differ from pred43.json (prediction: 0).
P4' (statistical, from v1): valid points with dim >= 786 (class C): 28..46; unit-valid departing fraction in [0.12, 0.30].
