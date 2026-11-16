# JET35 frozen implementation, 2026-10-29
Exact computation; no timing/statistical/IC claim. Same caps4GiB virtual/1200s.
Read Khuri-Makdisi math/0409209v3 Representation B, and retained R34 source
jets and proper-local bridge before choosing this implementation. Same Q,
u=u(Q)+z. Arithmetic explicitly reduced modulo(z154,T2+T+1); qring does not
implicitly reduce products. Cached-power evaluator consumes binary exponent
metadata with explicit zero slots, never nonzero ideal size as slot count.
Newton precisions2,4,8,16,32,64,128,154; lex first constant invertible pair;
all5 source equations checked after every correction. All8 inverse witnesses,
discriminant unit,5 coefficient conjugates/closure, corrupted nu+z153 rejected.
All68 descended sections compared with both literal original evaluations.
Every old coordinate/unit/inverse/68section/196quadratic reproduced moduloz9.
Fixed old selected196quadratics and324cubics, no adaptive basis search.
Ranks68,196,324 required with original-column pivot recombinations; polynomial
basis F32 words, not old trace-coordinate words. Export154words per section.
Accept only exit0/exactmarker/noSingularerrors/allchecks; keep failures/caps.
Sample/randomization/AA not applicable: deterministic exact identity checks,
not statistical estimation. Positive controls are identities/recombinations;
negative control is corrupted top source coefficient. No global normalization,
proper factor map, target action, relation rank or speed follows from jets.
The overarching PROTOCOL predates ALG35; this file and sources freeze before
JET35. ALG35 accepted770/770 on existing run, not repeated here.
