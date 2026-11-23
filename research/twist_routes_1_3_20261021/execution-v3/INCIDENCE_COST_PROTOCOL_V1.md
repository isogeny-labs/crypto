# Anchored box solver-cost diagnostic, version 1

Freeze source, plan and binaries before measuring. This first cost grid uses
all ten original public curves, H=K in {8,16}, and every distinct lambda in
{1,2,3,H}: eighty configurations, ten target pairs, target label 0. Selection
uses the complete named grid without filtering degree reductions, relation
counts or prior solver outcomes. The other 520 frontier cases and arity-three
full-cost comparison remain open. No final E28 labels are opened.

Seven processes per configuration (560 requested): direct-product A/A and
direct-product, full-mask, reduced-U A/A and reduced-U, group-MITM A/A and
group-MITM. All reconstruct their actual inputs from public curve, target,
H,K,lambda. Expected frontier answers and coefficients stay in the parent;
workers cannot read them through task fields. Preserve every failure/cap.

Direct-product constructs the distinct curve-rational base and both product
membership polynomials plus F3(x1,x2,Rx). Full-mask constructs the Cartesian
bounds, Newton-interpolated curve-and-twist mask and substituted F3. Reduced-U
constructs only r bounds, twist-selected U bounds and substituted F3. For this
anchored arity-two finite problem, F3 with nonzero rational target y guarantees
the source lifts, so reduced-U avoids constructing the redundant source mask
and full base. The same supplied U describes an identical ordinary encoding;
this does not establish an intrinsic twist advantage. Group-MITM constructs
the same explicit base and point lookup, then finds all signed pairs.

The generic native F4/substitution/root solver uses grevlex, maximum degree
128 for every polynomial arm, GM=1 and a shared 2,500 ms cooperative budget.
Each process has four-second timeout, one-second kill grace, 4 GiB address
space, 32 MiB/file and one thread. Cap the batch at 2,100 seconds and 1 GiB,
checkpointing every cell; outer transport has a 2,200-second deadline.

Charge startup, parsing, actual base/mask/equation construction, all solving,
finite roots, classical explicit sign lifting, point/sum verification,
canonicalization and serialization in simulated guest instructions Ir.
Do not charge construction avoided by an algorithm or treat an empty ideal
as an unattempted zero-cost success. No admitted wall time, OS/kernel cost,
independent replay, relation matrix, full DLP or matched-rho claim.

Require exact canonical pair and explicit signed-y lists against the frozen
600-case frontier, whose separate independent native audit and strict PARI
replay passed. Every retained finite root must satisfy its box/base bounds
and source sum. Deduplicated root counts must equal the complete independent
finite census, with box-fiber multiplicity retained; this certifies Fp answers,
not all algebraic-closure components. Replay accepted rows and wrong-sign
controls in PARI, validate hashes/clean errors and independently audit raw Ir.
Paired comparisons require <=5 percent A/A spread for both compared algorithms
(full-mask has no separate A/A, so uses the direct gate). Do not erase valid
group-control comparisons when a polynomial reference fails to complete.

Report every requested cell and each missing pair. Compare all polynomial
arms with both the product baseline and ordinary group MITM; report losses.
No new external full-root reference, fastest claim, novelty claim or admitted
benchmark row follows from this diagnostic. Lower degree alone is not a speedup.
