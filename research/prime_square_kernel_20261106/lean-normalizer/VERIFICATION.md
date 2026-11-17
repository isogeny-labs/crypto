# Executed checks for the actual kernel

Final mathematical source commit77a4f24e9c9907b9eec19ca1f612433ecc954dd8.
The source project has24 named theorems across CauchyNormalizer and KernelNodes.
These are producer evidence; claim standing remains unverified until a trusted
signed verdict exists. [pcc:unverified]

With the pinned Lean4.35.0 bin directory on PATH, from this Lake directory:

```
lake build
lake env leanchecker --fresh --verbose KernelNodes
```

Both completed exit0. The clean full build is nodes-build-005.log.
kernel-nodes-fresh-002.log names the successful final import-inclusive replay.
It covers KernelNodes, the final exposed normalizer definitions and their
imports. The earlier normalizer-only fresh replay also completed exit0;
its pre-exposure source is retained in successful-007.lean.txt. The earlier
missing-olean leaf replay is explicitly failed evidence. [pcc:unverified]

From the actual repository root, with actual sources staged:

```
bash tools/lean/proof-gate.sh --out /workspace/audit/square-normalizer-full-gate-002 \
  research/prime_square_kernel_20261106/lean-normalizer
```

Exit0. Audit26 declarations:24 named theorems, one generated instance proof,
one generated equation theorem.0 failing,0 native-trusted. Only propext,
Classical.choice and Quot.sound occur. Actual receipts are in gate/;
gate-initial/ and gate-before-nodes/ preserve the earlier scoped pass and
failed expanded build. No trust-list or proof-gate change was made.
[pcc:unverified]

The explicit-PATH producer PCC check of7288ef returns pass and binds the
correct declaration. Its unsigned result is pcc-binding-final.json, at the
mathematical source commit above. The initial missing-PATH not_covered result
is preserved separately. Neither result creates a signature or independent
execution verification. [pcc:unverified]

No native instrument, frozen protocol or Rust source changed during this
proof work. No native panel, benchmark or cloud job was executed. The full
panel is still awaiting the explicit backend exception requested earlier.
Remaining claims are coefficientwise chart content, uniform mode counts,
degree/recovery costs, recursion and ECDLP correctness/complexity; they are
not established by these pointwise algebraic theorems. [pcc:unverified]
