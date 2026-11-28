# Reproduction note (read-only arithmetic)

This diagnostic ran NO launcher, pod, relation/IC job, or Python-authored tool.
It only read evidence files and did order-of-magnitude arithmetic. The decisive
identity is the sweep's own first-moment ceiling:

    E[# m-decompositions of a uniform target] = N^m / m! / r

- N = distinct signed-orbit points in the factor base (K=64/256/600 -> 10,624 /
  42,496 / 99,600; koblitz_n83_factor_base_sweep_20261008/RESULT.md:261).
- r = subgroup order: a=0 primary 2417851639230796216685689 (log2 r = 81.000);
  a=1 diagnostic 8569786107849059 (log2 r = 52.928).

Checked against pilot-01/support-moments.json: e.g. a0 K600 m=4 stores
numerator N^4/4! and denominator r = 1.695987e-6; a0 K600 m=5 = 0.03378543;
a0 K600 m=6 vacuous. My awk re-derivation reproduced all of these.

Values re-derived (a0 primary, N=99,600, r=2^81.0):
  m=2 2.1e-15 | m=3 6.8e-11 | m=4 1.70e-6 | m=5 3.38e-2 | m=6 560.8
Values re-derived (a1 diagnostic, N=99,600, r=2^52.9):
  m=3 1.9e-2 | m=4 478.5 (plentiful)
Primary 2+2 MITM probes (K=64,256) were run to exhaustion and returned MISS;
with m=4 E = 2.2e-10 (K64) / 5.6e-8 (K256) a MISS is essentially certain.

Free-relation threshold (ic_free_relation_census_20261011/RESULTS.md:44-47):
  K >= (3 r / n^3)^(1/3), n=83  ->  ~3,556 (a1),  ~2.33e6 (a0 primary).

ecbench column scaling (ecbench_frobenius_orbit_vs_rho_20261014/README.md:70-79):
  working orbit-IC needs c = 1..90 for n=13..41 (covers 2^11..2^39); covering
  the primary's 2^81 is ~2^42x more columns than the n=41 base -> millions.

All arithmetic is order-of-magnitude. The E figures are first-moment UPPER
bounds under a uniform-target-with-repetition model; real signed-Frobenius,
fixed-target decomposition counts are <= these, possibly by 1+ orders.

To re-check the E table (awk, no Python):
  awk 'BEGIN{r=2417851639230796216685689.0;N=99600.0;
    f[2]=2;f[3]=6;f[4]=24;f[5]=120;f[6]=720;
    for(m=2;m<=6;m++){e=1;for(j=0;j<m;j++)e*=N;printf "m=%d E=%.4g\n",m,e/f[m]/r}}'
