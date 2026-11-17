#set page(margin: 2cm)
#set text(size: 10pt)
= L3: Precomputation and multi-target attacks on fixed curves

Writeup, derived from published bounds and atlas cards `KR-RHO-ea34b8` and `KR-RHO-46c2c6`. No measurement.
Units are log2 of group operations (or of stored entries). Not a break.

== Implied exponents

#table(
  columns: 8,
  [Curve], [rho / target], [BL table], [BL precomp], [BL online / target], [BL online, k=2^10], [KS total, k=2^10], [KS / target],
  [P-256 (log2 l = 256.00)], [128.00], [85.33], [170.67], [85.33], [95.33], [133.50], [123.50],
  [Curve25519 (252.00)], [126.00], [84.00], [168.00], [84.00], [94.00], [131.50], [121.50],
  [ECCp-131 (130.19)], [65.09], [43.40], [86.79], [43.40], [53.40], [70.60], [60.60],
)

BL = Bernstein–Lange; KS = Kuhn–Struik total √(2kl). The task's "table of about l^(2/3)" is split here:
stored table about l^(1/3) entries, precomputation about l^(2/3) operations.

== Tradeoff

#image("L3_tradeoff.svg", width: 90%)

Derived from the Corrigan-Gibbs–Kogan line S·T² = l. Rho and Bernstein–Lange are the endpoints; intermediate points are not attained algorithms.

== Reading

The sub-rho online cost is paid by a precomputation far above rho (about 2^42.7 for P-256). It is not an attack on any of the three curves.
