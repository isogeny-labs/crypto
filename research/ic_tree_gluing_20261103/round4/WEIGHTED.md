# Weighted vs standard grading, Singular 4.3.2 (m=4, t=4, p=786433, k0)

Question: did G lose to F only because standard grading counts q (product of two roots) as degree 1?

## Results

| arm | order | time (s) | exit | \|GB\| | max weighted deg of GB | vdim | max protocol degree |
|---|---|---|---|---|---|---|---|
| F | dp | 105.1 | 0 | 12 | 4 (std deg 1) | 1 | 15 |
| F | wp(1,2,3,4,1^8) | 97.3 | 0 | 12 | 4 (std deg 4) | 1 | 34 (weighted) |
| G | dp | >1800 (timeout) | 124 | n/a | n/a | n/a | >= 23 at stop |
| G | wp(1,2,1^6,1,2,1^6) | >1800 (timeout) | 124 | n/a | n/a | n/a | >= 37 at stop (weighted) |

Timeouts are lower bounds. F is not the unit ideal (vdim 1, dim 0, unit_ideal 0), so the planted solution is implicit.
No vdim/size for G: std() did not finish, so "G has vdim > 1" is not checked here.

## Weight convention
F: e1..e4 weights 1,2,3,4. G: s0,s1 weight 1; q0,q1 weight 2. Chain variables (fr*, h0r*, h1r*) weight 1 in both arms.
Ordering `wp(weights)` in the variable order of the .ms file; "max weighted deg" is `deg(f, w)` with these weights for both orders.

## Commands
`/home/user/crypto/research/ic_tree_gluing_20261103/round4/run1.sh ARM ORDER` runs
`ulimit -v 3145728; MS=<file.ms> ARM=<F|G> ORDER=<dp|wp> taskset -c <cpu> timeout 1800 Singular -q weighted.sing > logs/weighted/ARM_ORDER.log`
with wall time from bash `date` appended as `RC <exit> WALL <s>`. .ms inputs:
F `.../scratchpad/r2/m4_t4_k0_F.ms`, G `.../scratchpad/r3/m4_t4_k0_G.ms`.
Run pairs: F dp (cpu0) with G dp (cpu1); then F wp (cpu0) with G wp (cpu2). At most 2 Singular processes at a time, 3 GiB address-space cap each.
Script: `weighted.sing` (reads the .ms, builds `ring R=786433,(vars),(dp|wp(w))`, `option(redSB,prot)`, `std`).
Logs: `logs/weighted/{F,G}_{dp,wp}.log`.

## Protocol degree parsing
With option(prot) std prints a stream of `s`, `-`, `(pairs)` and bare integers. A bare integer immediately followed by `(` and not itself in parentheses is the new current degree
(e.g. `s10(611)`). Extracted with `grep -oP '(?<![0-9(])[0-9]+(?=\()' LOG | sort -n | tail -1`. Under wp it is the weighted degree. For timed-out runs it is the last degree reached.

## Reading
Weights do not rescue G in this test. F is unchanged by weighting (97 s vs 105 s, same 12-element GB), while G did not finish in 1800 s under either order, so the G/F time ratio is >18 under wp and >17 under dp: no sign of G becoming comparable. At the cutoff G had reached degree 23 under dp and 37 weighted under wp, against F finishing at 15 (dp) and 34 (wp), so the weighted degree reached by G was already above F's whole run. Natural weights alone do not remove the G handicap; the hypothesis is not supported at this budget (but a longer cap could in principle still finish; the result is a lower bound). Singular std here is ~10x slower than msolve on F (10.7 s), so absolute times are not msolve-comparable.
Caveat: an unrelated msolve job ran on another core during the G runs (possible minor contention).
