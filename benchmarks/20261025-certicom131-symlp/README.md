# Certicom 131-bit symmetry breaking + double large prime (2026-10-25)

Measured on this host (single thread, seed 20261025), one field
implementation per family. The index-calculus decomposition step and the
Pollard-rho adding-walk step share the field code, so step rates compare
directly.

- ECC2-131 (binary): symmetric e1 oracle 5.24 Msteps/s, ~28.7x the
  per-point baseline, ~1.20x the rho step (4.35 Msteps/s).
- ECCp-131 (prime): symmetric oracle 0.09 Msteps/s, ~0.6x its baseline
  (no subspace saving), ~0.02x the rho step (5.61 Msteps/s).
- Yield: direct (0-large-prime) relations ~5e-7 of decompositions; the
  double-large-prime graph rescues the 2-lp bulk; planted full-size
  pipeline recovers a logarithm on both real groups.
- Derived distance to rho at n=131 (m=2 floor 2^{2n/3} vs rho 2^{n/2}):
  ~2^22 steps, constant-factor unchanged by the accelerators.

Study: ../../research/certicom131_symlp_20261025 (REPORT.pdf, runs/,
figures/). Report: REPORT.pdf here.
