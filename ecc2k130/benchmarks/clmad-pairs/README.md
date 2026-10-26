# The RTX PRO 6000's carry-less unit: which issue forms reach which rate

Question: every ECC2K-130 walk on the RTX PRO 6000 is bound by the
carry-less unit. The tree had two rates for it, 1.69 lane-CLMADs per SM-clock
for a stream of `CLMAD.lo` and 1.99 for lo+hi pairs, both from
[`clmad-price/probe.cu`](../clmad-price/probe.cu), and TWO-CHAINS.md section 1
notes the floor would move from 22.3 to 27.4 B/s if the kernel ran at the
pair rate. Which feature of the pair stream earns the higher rate, and can a
product use it?

[`probe.cu`](probe.cu) runs 23 streams, 16 independent chains per thread,
256-thread blocks, at 8 and 2 blocks per SM ([`gpujob.sh`](gpujob.sh)). Each
counts only lane-CLMADs. One RTX PRO 6000 Blackwell Server Edition (Modal,
driver 610.57.04, nvcc 13.3.73), rates per SM-clock at the clock sampled after
each pass, two passes each: [`results/rates-per-sm-clock.txt`](results/rates-per-sm-clock.txt).

| stream | lane-CLMADs / SM-clock |
|---|---:|
| any single opcode, any addend (`lo`, `hi`, `lo` accumulate, `hi` accumulate, addend another register) | 1.665-1.674 |
| `lo` and `hi` mixed with zero addends (adjacent, swapped operands, grouped 16+16, 3+3, Karatsuba order, 4 LOP3 between) | 1.665-1.674 |
| mixed, addend a zero held in a uniform register (`UR`, from a kernel argument) | 1.665-1.673 |
| mixed, fresh destination, one multiplicand a per-lane zero register | 1.666-1.672 |
| mixed, `lo` zero addend and `hi` addend another register | 1.807-1.814 |
| **mixed, both in the in-place accumulate form `clmad r, r, y, r`** (the clmad-price pair, forced adjacent, grouped, `hi` reading the old operand, operands kept nonzero every round) | **1.978-1.987** |
| **two-way Karatsuba product, every CLMAD in place on a copy, XOR-corrected** | **1.980-1.999** |

Findings, in the order the runs established them:

1. **The 1.99 figure is real, not an artifact.** The clmad-price pair stream's
   operands do collapse: `a` reaches zero within about 80 rounds and stays
   there for over 95% of the run. But the same stream with operands forced
   nonzero every round still runs at 1.98, and a zero multiplicand gives no
   speedup in any other form.
2. **It needs both opcodes and a per-lane register addend on both.** Every
   single-opcode stream stays at 1.67 in every form, so this is not a faster
   `lo` or `hi`. Mixed streams with `RZ` addends or a uniform-register zero
   stay at 1.67, and a register addend on `hi` alone gives 1.81.
3. **A product can use it.** `clmad r, r, y, r` on a copy `r = x` computes
   lo/hi(x*y) XOR x, and one `LOP3` cancels the `x`. A complete two-way
   Karatsuba 128x128 product in this form runs at 1.98-2.00, **1.19x** the
   same product with zero addends, for six XORs on the logic pipe, which this
   part has to spare. The SASS (`results/probe-sass-counts.txt`, and the
   `CLMAD.LO Rd, Ra, Rb, Rb` lines in a disassembly) confirms ptxas keeps the
   form.

Not established here: why the hardware behaves this way. The probe measures
issue rates of fixed patterns and nothing about the unit's internals.

Before this probe, the 20 B/s table-walk build issued 268 CLMADs, none of them
adjacent and 238 of them `.LO` with zero addends
([`results/20b-clmad-pattern.txt`](results/20b-clmad-pattern.txt)). Whether
moving its products to the accumulate form raises a whole walk is measured
separately, against the campaign's sigma-table build, in
[`../clmad-self/`](../clmad-self/) (knob `PACKED_CLMAD_SELF`).
