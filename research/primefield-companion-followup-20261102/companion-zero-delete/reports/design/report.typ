#set page(paper:"a4",margin:17mm)
#set text(size:10pt)
= Zero-prefix coordinate deletion
2026-11-02 · Derived identities / static integration · Numerical trials: zero

*Invariant.* With nonzero diagonal D and DA=AᵀD, checked zero columns are
also zero rows. A proper zero prefix of length j gives A=diag(0,T), where
T has dimension q=n−j and inherits the trailing metric.

#image("pipeline.svg",width:100%)

*Displacement formula.* Write S for the q-dimensional nilpotent shift,
e for its first unit vector and f for its last. The trailing block gives
ST−TS=GₜKₜᵀ. Restoring distinct companion wraps yields

G′ = [Gₜ, a e, −b T e],     K′ = [Kₜ, Tᵀ f, f].

Width grows from w to w+2 before recompression. The selector retains its first
nonzero column to supply T e, including secondary-diagonal selection. One
transpose source query supplies Tᵀ f. The identity also holds for q=1.

#table(columns:(2fr,2fr),inset:5pt,
 [Prefix RHS],[Derived branch],
 [Has nonzero entry bₜ],[Witness eₜ/bₜ; zero new source queries],
 [All entries zero],[One transpose query; reduce n to n−j],
 [No zero prefix],[Unchanged paired pivot route])

*Exact reinsertion.* Both solution and dual witness tails lift by zero
padding. Prior Schur frames then unwind normally, and the original root
checks remain. Early witnesses leave the trailing rank unknown: report a
null full rank and only the established pivot-count lower bound.

*Charged work.* Reduction uses one original-width query, q field products
and one subtraction outside it, plus generator copies and allocation.
The next iteration repeats exact compression and selection on the smaller
operator. Retaining the first column adds at most n field entries in the
secondary-diagonal branch. ZeroFrame retains only dimensions, meter and a
consumed flag; padding allocates j zeros and copies q tail entries.
No net runtime or heap improvement is measured. Prefix frequency is unmeasured.

*Checks and next.* Three declared-substitution AST audits pass; six Python
sources parse. A frame AST check excludes source/field-arithmetic calls.
Numerical, corruption, heap and performance validation is unexecuted under
the existing cost gate. Required controls include scalar tails, repeated
deletions, alternating Schur/permutation frames, both RHS outcomes, independent
displacement/lift checks and rank semantics. Next reuse the selected pivot
columns on the reduced operator to avoid repeating search.

*Sources.* DERIVATION.md, deletion.py, selection\_delete.py, driver\_delete.py,
integration.json. Standard cyclic-shift displacement conventions:
Bostan et al.2017, SIAM J. Matrix Anal. Appl.38(3), equation (2).
