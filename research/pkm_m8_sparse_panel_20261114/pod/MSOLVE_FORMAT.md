# msolve 0.6.5 output grammar over F_p (empirical)  [pcc:unverified]

Binary: `/usr/bin/msolve`, sha256 `c2722288d22c3a1eab00c0b818d0204d4a98104d059b2fd4ecb7343e8ef759fa`,
Ubuntu 24.04 package `0.6.5-1build2`. All examples below are reproduced by the files in `fmt/`
(inputs `a.ms b.ms c.ms d.ms nosol.ms posdim.ms cyc{6,7,8,10}.ms`, outputs and logs next to them).
Date 2026-11-14, this container (glibc 2.39).

## 0. Findings the protocol must act on (read first)

1. **`-g 2` and `-P 1` are NOT combinable.** `msolve -g 2 -P 1` prints only the reduced Groebner basis
   (grevlex) to `-o`, never the parametrization, and stops after the first F4 pass (single trace table,
   no genericity retry). The brief's command line `-t T -v 2 -g 2 -P 1` therefore yields a GB only.
   To get a parametrization run `-P 1` WITHOUT `-g` (a second solve). `job.sh` takes `msolve_mode` per
   plan row: `gb` (`-g 2`), `param` (`-P 1`), or `both` (two separate runs, GB first).
2. **`-P 1` alone may run several F4 passes** (see 3.4): the first pass in the given variable order, and
   if the staircase is not generic it reruns with reversed variable order, then with an extra variable
   `A` carrying a random linear form (`-c 2` default). The `-v 2` log then holds up to three F4 tables;
   the work proxy must sum all tables (the parser does and labels each pass).
3. **Exit code is 0 in every logical outcome** (solutions, no solution, positive dimension, non-squarefree
   elimination polynomial). Outcome is only in the output file. See section 5 for failures.
4. **Input prime**: tiny inputs (e.g. `x^2-2, y-x` at p=65537 or p=131101; cyclic-6/7/8 at p=65537) make
   msolve SEGFAULT (rc 139) in 0.6.5 although p=65521, 10007, 32771, 262147 .. 2^30 and the real panel
   prime 16801793 are fine. Treat rc 139 as censored/failed, never as data. Do not use p in 2^16..2^17 for
   dry runs.
5. **`prlimit --as=` needs >= about 1.3 GB even for a 5 MB problem**: msolve reserves ~1.2 GB of virtual
   address space at start (VmSize 1.2 GB, VmRSS 72 MB on cyclic-10, 2 threads). cyclic-8 (RSS 31 MB):
   `--as` 1024 MB segfaults at once, 1250 MB works with -t 1/4 but segfaults with -t 8, 1400 MB works with
   -t 8. Choose `mem_cap_kb` >= 2 GB + real need; the "2 GB cap" dry run is only just enough for 2 threads.
6. **Real memory exhaustion or the rlimit = SIGSEGV** (no OOM message): cyclic-10, `--as=2 GiB`, -t 2:
   died at 78 s with `Command terminated by signal 11`, max RSS 953 MB; the last `-v 2` row printed is the
   last COMPLETED round (the failing round prints nothing, the line may be cut mid-row).
7. **`/usr/bin/time` (GNU time) is not in the base image**: `apt-get install -y time` (it is installed here
   and on the Ubuntu 24.04 pod after `apt-get update`). `time -v` reports `Exit status: 0` for a child
   killed by a signal; the signal is the separate line `Command terminated by signal N`, and the shell sees
   128+N. With `time -v timeout T prlimit ... msolve`, `time` reports `Exit status: 124` on timeout (msolve
   gets SIGTERM; wall ~ T, user time up to threads*T).

## 1. Input (`.ms`) used

```
x,y,z                       <- variables, comma separated
10007                       <- characteristic
x^2+y^2+z^2-3,              <- one polynomial per line, comma after every polynomial except the last
x*y*z-1,
x+y-2*z
```

## 2. `-P 1`: output file grammar (EBNF)

```
file     := noSol | posDim | zeroDim
noSol    := "[-1]:"
posDim   := "[1, " nvars ", -1, []]:"                    (e.g. "[1, 2, -1, []]:")
zeroDim  := "[0, [" p ", " N ", " Q ", [" names "], [" lin "], [1, [[" ... "]]]]]:"
```

Real, whitespace-tolerant form of zeroDim (newlines/spaces are insignificant; a trailing `:` ends the file):

```
[0, [P, NV, Q, ['v1', 'v2', ..., ('A')], [l1, l2, ..., ln], [1,
 [[DF, [f0, f1, ..., fDF]],
  [DW, [w0, ..., wDW]],
  [ [[D1, [g1_0, ..., g1_D1]]], [[D2, [g2_0, ...]]], ... ]
 ]]]]:
```

* `P` characteristic. `NV` = number of names listed (`len(names)`; includes `A` when present).
* `Q` = dimension of the quotient algebra F_p[x]/I **with multiplicity** (verified: `b.ms` Q=6, 6 solutions
  counted with multiplicity). `DF` = degree of the eliminating polynomial = number of DISTINCT solutions
  when the form is separating. `DF < Q` iff multiple roots (b.ms: Q=6, DF=5; msolve prints
  "Elimination polynomial is not squarefree." on stderr at `-v 2`, the param still describes the radical).
* `names`: Python-style quoted list. If msolve had to add the random-form variable it appears LAST as `'A'`.
* `lin`: integer list, same length as `names`. Meaning (verified numerically on c.ms):
  - with `A` present: `l1*v1 + ... + ln-1*v(n-1) + ln*A = 0`, i.e. `A = -(sum l_i v_i)/ln`. In c.ms
    (`[-11,66,83]`): `-11x+66y+83A=0`; reducing `-11x+66y` modulo the elimination polynomial gives `-83*t`.
  - without `A` (`a.ms`, `d.ms`: `[0, 1]`): the separating form is the variable with coefficient 1
    (here `t = y`), coefficients are a unit vector.
* `f` = eliminating polynomial in `t` (t = `A` or the chosen variable), coefficients LOW degree first,
  mod P, monic (`fDF = 1`). `[DF, [..]]`: DF is the degree, list length DF+1.
* second block `[DW, [w..]]` is the denominator. In every run seen it is `[0,[1]]` (constant 1); then
  `v_i = - g_i(t) mod f(t)`. If a DW > 0 ever appears use `v_i = -g_i(t) / w(t) mod f(t)`
  (the 0.6.5 `-P 1` convention). Verified: c.ms with `w=1` gives `x^2-1 = y^2-1 = 0 mod f` exactly; using
  `w=f'` does not.
* third block: one `[[Di, [coeffs low first]]]` per variable in `names` order, EXCLUDING the variable that
  is `t` itself (and excluding `A`). So `NV-1` entries. Degrees Di <= DF-1 (coefficient list length Di+1).
* All integers are mod P in `[0, P)`; `-1` (as `10006` mod 10007) is written as a positive residue.

### Examples (verbatim)

`a.ms` = `x^2-2, y-x` over F_10007 (`[0,1]`, no `A`; x = -g(t) = t, y = t):
```
[0, [10007, 
2, 
2, 
['x', 'y'],
[0, 1],
[1,
[[2,
[10005, 0, 1]],
[0,
[1]],
[
[[1,
[0, 10006]]]
]]]]]:
```
`c.ms` = `x^2-1, y^2-1` (4 solutions, form added: `A`, `[-11,66,83]`):
```
[0, [10007, 
3, 
4, 
['x', 'y', 'A'],
[-11,66,83],
[1,
[[4,
[8003, 0, 5016, 0, 1]],
[0,
[1]],
[
[[3,
[0, 4020, 0, 8000]]],
[[3,
[0, 5978, 0, 4669]]]
]]]]]:
```
`b.ms` (3 vars, Q=6 but DF=5, repeated root): header `4, 6`, names `['x','y','z','A']`, `[-11,66,83,-95]`,
`[[5,[6 coeffs]], [0,[1]], [ [[4,[5 coeffs]]], [[4,..]], [[4,..]] ]]`. No solution: `[-1]:`.
Positive dimension (`x*y`): `[1, 2, -1, []]:`. (Files: `fmt/aP1.out bP1.out cP1.out dP1.out nosolP1.out posdimP1.out`.)

Parsing recipe for `verify`: strip the trailing `:`; replace `'name'` by `"name"`; the rest is valid JSON
(`[0, [10007, 2, 2, ["x","y"], [0,1], [1, [[2,[..]],[0,[1]],[ [[1,[..]]] ]]]]]`). Check `P`, `NV==len(names)`,
`len(lin)==NV`, `DF==len(f)-1`, `f[-1]==1`, entry count `NV-1`. Distinguish outcomes by the first token
(`[-1]`, `[1,`, `[0,`). An F_p-rational solution = a root t0 of f in F_p; recover v_i by Horner of `-g_i(t0)/w(t0)`;
`t` itself is `A=-(sum l_i v_i)/ln` or the unit-vector variable.

## 3. `-g 2`: reduced Groebner basis (grevlex) file

```
#Reduced Groebner basis data
#---
#field characteristic: 10007
#variable order:       x, y, z
#monomial order:       graded reverse lexicographical
#length of basis:      3 elements sorted by increasing leading monomials
#---
[1*x^1+1*y^1+10005*z^1,
1*y^2+10005*y^1*z^1+5006*z^2+5002,
1*z^3+8005*z^1+2001]:
```
* Lines starting `#` are a header. `#length of basis:  N elements ...` (`1 element` singular for N=1).
* The body is `[` poly `,` newline poly ... `]:`. Terms `c*x^e*y^e...` joined by `+`, coefficient first,
  EVERY exponent written, including `^1` (`1*x^1`) but a constant has no variables (`+10005`); a term with
  coefficient 1 still prints `1*`. Coefficients are residues in `[0,P)`, no minus signs.
* No solution (1 in the ideal): `[1]:` with `#length of basis:      1 element`. Positive dimension
  (`x*y`): `[1*x^1*y^1]:` (the GB is printed, the dimension is not stated: read it off the leading
  monomials). Variable order in the header is the input order (`-g` never adds `A`).
* The staircase of the leading terms gives Q (zero-dim iff each variable has a pure-power leading term).
  With `-v 2` stderr also prints `Dimension of quotient: Q` (seen with `-g 2` on b.ms).
* `-g 1` prints the leading ideal only (not tested further).

## 4. `-v 2` trace

Stdout carries the F4 trace; the INPUT DATA box (`#variables`, `#equations`, `field characteristic`,
`#threads`, ...) and the final line `msolve overall time X sec (elapsed) / Y sec (cpu)` plus
`Dimension of quotient: Q` go to stderr in 0.6.5 (verified `bv2g2P1.stdout/.stderr`). Capture both.
Per round, one row (columns, from the legend msolve itself prints):

```
deg     sel   pairs        mat          density            new data         time(rd) in sec (real|cpu)
  2       1       2       4 x 7          46.43%        1 new       0 zero         0.00 | 0.00
```
`deg` degree of the pairs selected; `sel` pairs selected this round; `pairs` total pairs in the list
(after selection, before update); `mat` rows x cols of the F4 matrix; `density` percent nonzero entries
of that matrix; `new` elements added to the basis; `zero` zero reductions; `time` real | cpu seconds of the round.
Then a line `reduce final basis   R x C   dens%   N new   Z zero   t | t` (the final-interreduction matrix),
`TIMINGS` (`overall(elapsed)`, `overall(cpu)`, select, symbolic prep., update, convert, linear algebra,
reduce gb, each `sec` and `%`), and `COMPUTATIONAL DATA` (`size of basis`, `#terms in basis`,
`#pairs reduced`, `#GM criterion`, `#redundant elements`, `#rows reduced`, `#zero reductions`,
`max. matrix data R x C (d%)`, hash table sizes). Single-F4 `-g 2` logs have one such table.
`-P 1` without `-g` may append more tables, each preceded by a banner and with the legend
text spelled "Legend for f information" (typo `f` for `f4`):

```
Changing variable order for possibly more generic staircase:
z, y, x
...                      (second table)
Adding a linear form with an extra variable (lowest wrt monomial order)
[coefficients of linear form are randomly chosen]
...                      (third table)
Elimination polynomial is not squarefree.        (stderr, when DF < Q)
Time spent to compute eliminating polynomial (elapsed): 0.00 sec
Time for rational param:          0.00 (elapsed) sec /  0.00 sec (cpu)
```
Time columns are `real | cpu` with 2 decimals (rounds below 5 ms print 0.00). With `-t N` cpu can exceed real.
Rounds can repeat the same `deg` (several rows with deg 4 in the committed L1 log): `deg` is the degree
of the selected pairs, not the round index. The row is printed only when the round finishes.

## 5. Exit codes and failure modes (measured)

| situation | shell rc | `time -v` | output file |
|:--|--:|:--|:--|
| solved, no solution, pos. dim., non-squarefree | 0 | `Exit status: 0` | content as above |
| `time -v timeout 3 msolve` (SIGTERM) | 124 | `Exit status: 124` | not written/partial; log cut mid-run |
| `timeout -s KILL` | 137 | `Command terminated by signal 9`, `Exit status: 0` | none |
| `prlimit --as` too small / real exhaustion / input crash | 139 | `Command terminated by signal 11`, `Exit status: 0` | none |
| unreadable input file | not tested | | |

Censoring rule for `runs.jsonl`: rc 124 -> `timeout`; rc 137 or 143 -> `signal:KILL/TERM`; rc 139 (or any
128+n) -> `signal:n` and, if `mem_cap_kb` was set, report it as `rlimit_or_crash` (indistinguishable from a
genuine msolve crash such as the p=65537 bug); anything else nonzero -> `error`.
The `-v 2` log of a censored run is kept: its last rows are the matrix-size evidence up to the cut.

## 6. Practical notes
* `-g 2` output is dense and large: cyclic-8 mod 65521 (a 4-ish thousand-term basis) is a 2 MB file; the
  panel's m = 8 GBs will be far larger. Keep `solve.out` out of Git (S3 by sha256), the sha256 is in `runs.jsonl`.
* `-P 1` runtime = GB (grevlex) + sparse-FGLM-style conversion + possible retries (section 4); `-g 2` alone
  skips the conversion, so the two modes are two separate timed solves, never one.
* `-t N` is honoured (cpu column > real column); `taskset` must grant at least N cpus or msolve oversubscribes.
* Both stdout and stderr are needed to reconstruct a run (trace on stdout, input box + timing on stderr).
