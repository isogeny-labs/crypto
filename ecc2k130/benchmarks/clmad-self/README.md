# PACKED_CLMAD_SELF on the campaign sigma-table walk: do not promote

[`../clmad-pairs/`](../clmad-pairs/README.md) found that a two-way Karatsuba
product written in the in-place accumulate form, `clmad r, r, y, r` on a copy
`r = x` with the `x` cancelled by one XOR, issues at 1.98 lane-CLMADs per
SM-clock on the RTX PRO 6000 against 1.67 in the zero-addend form, 1.19x.
`PACKED_CLMAD_SELF=1` (`include/packed131.h`) routes the two-way product, the
flat-issue product and the paired product through that form. Squarings are
left in their zero-addend form, since single-opcode streams ran at 1.67 in
every form.

[`gpujob-headline.sh`](gpujob-headline.sh) is the sigma-table headline protocol
with the campaign's sigma-table build (`THREADS=512 MINBLOCKS=1
PACKED_SIGMA_TABLE=1`) as control and the same plus `PACKED_CLMAD_SELF=1` as
candidate. One RTX PRO 6000, driver 610.57.04, nvcc 13.3.73
([`results/`](results/)).

**Correctness: passed.** Both arms pass all seven device arithmetic suites
(18,194 products including all 17,161 basis pairs, paired products,
reductions, squares, normal products, Frobenius vectors), the client
integration, replay and checkpoint tests, 300/300 report re-walks with zero
drops, and the sorted 1,709,477-record v1 corpora are identical. The
candidate's walk binary has 60 of its 109 CLMADs in the accumulate form; the
other 49 are the squarings (`results/sass-clmad.txt`).

**Throughput: did not pay.**

| | control | candidate |
|---|---:|---:|
| A/B median, B complete updates/s | **15.936** | **15.852** |
| paired ratios | | 0.9925, 0.9900, 0.9966, 0.9948, 0.9947 |
| median ratio | | **0.9947** |
| A/A maximum drift | 0.14% | |

Every pair favours the control. The frozen summarizer's verdict is
`DO_NOT_PROMOTE`. The knob stays off by default.

**Interpretation, separate from the measurement.** A faster carry-less unit
can only help a walk that the unit binds, and two measurements on this build
say it does not. Moving the λ² squaring to the logic pipe (`PACKED_ALU_SQUARE=1`
in the [sigma-table sweep](../autosweep/rtx-pro-6000-sigma-table/), about 11%
fewer CLMADs per update) gained 0.19%. Here, raising the issue rate of 60 of
109 CLMADs cost 0.5%, about what the added copies and XORs would cost on a
walk bound elsewhere. The same sweep's winner was `PACKED_L2_PERSIST=1` at
+5.4%, and that arm ran at 2.42 GHz where the control sat near 2.30 GHz at
the 600 W cap. That points to memory traffic and the power limit rather than
the carry-less unit, though no counter here measures the power split
directly. The form remains available for a kernel the unit does bind.
