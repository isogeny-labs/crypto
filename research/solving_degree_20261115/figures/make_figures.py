"""Lay out recorded values (no computation): sd^stat, d_lf, d_ff vs depth k.
Sources (ml-cryptanalysis, branch claude/lfd-proofs-20261108, experiments/ic-bounds-solving/):
  laneA_prime_m2/RESULTS.md table (k, d_ff, d_lf, sd), laneB_prime_m3/RESULTS.md table,
  laneC_binary/data/growth_table.md (proper ideals: medians). All values [pcc:unverified]."""
import matplotlib; matplotlib.use('Agg')
import matplotlib.pyplot as plt
# prime, two summands (lane A)
k2 = list(range(3, 14))
sd2 = [2*k+2 for k in k2]                      # measured on every unit instance, k = 3..13
dlf2_meas = {3:4, 4:4, 5:4, 6:5, 7:5, 8:5}      # measured (lfd_engine)
dlf2_rec = {9:5, 10:6, 11:6, 12:6, 13:6, 14:7}   # recorded exact (ic-bounds-exact10, ic-bounds-k14 unit p=65537)
dff2 = {k:4 for k in range(3, 10)}
# prime, three summands (lane B)
k3 = list(range(2, 9)); sd3 = [3*k+6 for k in k3]
dlf3 = {2:7, 3:7, 4:7, 5:7}                      # k=2,3 measured; 4,5 recorded
# binary, two summands, proper ideals (lane C, medians over exact rows)
kb = list(range(2, 9)); dfull_b = [3,4,5,6,7,8,9]; lfd_b = [2,3,3,3,3,4,4]; dreg_b = [2,3,3,4,4,5,5]; dff_b=[2]*7
fig, axes = plt.subplots(1, 3, figsize=(13, 4.2))
ax = axes[0]
ax.plot(k2, sd2, 'o-', label='sd$^{stat}$ (measured, every unit instance)')
ax.plot(sorted(dlf2_meas), [dlf2_meas[k] for k in sorted(dlf2_meas)], 's-', label='$d_{lf}$ measured')
ax.plot(sorted(dlf2_rec), [dlf2_rec[k] for k in sorted(dlf2_rec)], 's--', color='C1', label='$d_{lf}$ recorded exact (repository runs)')
ax.plot(k2+[14], [(k+14)//4 for k in k2+[14]], '-', lw=0.8, color='C1', alpha=.5, label='law $\\lfloor(k+14)/4\\rfloor$')
ax.plot(sorted(dff2), [dff2[k] for k in sorted(dff2)], '^-', label='$d_{ff}$ measured')
ax.plot(k2, [k+2 for k in k2], ':', color='gray', label='$k+2$ (U1 bound on $d_{lf}$)')
ax.set_title('prime, two summands ($m=2$)'); ax.set_xlabel('depth $k$'); ax.set_ylabel('degree'); ax.legend(fontsize=7); ax.grid(alpha=.3)
ax = axes[1]
ax.plot(k3, sd3, 'o-', label='sd$^{stat}=3k+6$ (measured)')
ax.plot(sorted(dlf3), [dlf3[k] for k in sorted(dlf3)], 's-', label='$d_{lf}=7$ (k=2,3 measured; 4,5 recorded)')
ax.plot(k3, [3*k+1 for k in k3], ':', color='gray', label='$mk+1$ (K1 bound on $d_{lf}$)')
ax.set_title('prime, three summands ($m=3$)'); ax.set_xlabel('depth $k$'); ax.legend(fontsize=7); ax.grid(alpha=.3)
ax = axes[2]
ax.plot(kb, dfull_b, 'o-', label='$d_{full}^{stat}$ median (exact rows)')
ax.plot(kb, lfd_b, 's-', label='LFD median'); ax.plot(kb, dff_b, '^-', label='$d_{ff}$ median')
ax.plot(kb, dreg_b, 'd--', label='$d_{reg}$ semi-regular')
ax.set_title('binary, two summands, proper ideals'); ax.set_xlabel('depth $k$'); ax.legend(fontsize=7); ax.grid(alpha=.3)
fig.suptitle('Static solving degree grows linearly in k; the last fall degree does not (study solving_degree_20261115, all values [pcc:unverified])', fontsize=9)
fig.tight_layout(); fig.savefig('figures/sd_growth.svg'); fig.savefig('figures/sd_growth.png', dpi=150)
print('ok')
