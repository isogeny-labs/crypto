# Resume: crypto @ research/h114-multiradix-carry-cohomology-freeze

This is the H114-specific checkpoint note. Copy it to `.resume/RESUME.md` only
on the exact H114 branch immediately before the ordinary checkpoint push/gate.

## Goal

Determine whether one of the 11,520 frozen shared-channel multiradix carry
predicates gives a uniform scalar digit modulo 30 on the seven-curve H114 toy
cohort, with an independently verified end-to-end decoder. This is a bounded
falsification experiment for a possible non-generic ECDLP digit mechanism; it
is not a claim of a generic or subexponential algorithm.

## Lead / hypothesis

The lead extends H113's two-multiplier winding to the three faces `(2,3)`,
`(2,5)`, and `(3,5)`. A single public generator-aware channel is applied to
all limbs, fixed residue permutations are combined by CRT, and all six orders
of `[2]`, `[3]`, `[5]` are constrained by a braid identity. One unchanged ID
that is exact on all 28 rows and decodes all 9,096 targets confirms the frozen
toy-cohort candidate. An empty 28-row intersection kills this exact family.

## Spec

Study root: `research/h114_multiradix_carry_cohomology_20261117/`.
Authoritative human protocol: `PROTOCOL.md`, SHA-256
`92a970c1392183ceb5c84d76082af7b977ebf52145c6b938704cfca24d708c3b`.
Machine protocol: `PROTOCOL.json`, SHA-256
`8d89092331a64a00d51ce7caa1602b035ff582835f8b6f7287912d07590ed51c`.
Curve cohort: `CURVES.json`, SHA-256
`5dff248152f4cfb3a8b5f9f39542110767d60779d4b917e4b817a935b6170385`.
Producer source revision is crypto `4ceb5c032917a5f72a15046eee5aa4aad327a93c`;
producer source SHA-256 is
`795a80d7cd527ac878e54087664aaa1ccf6c46322f7952c7dd213df0a8b90d57`.
Independent verifier is cryptanalysis PR #1736 revision
`f18430232e2814922d95f67e97abdda5d6ff44f6`, source SHA-256
`307589c02d437d7d13377c6fce194b441f37579d0b1879263fbe925591a26650`.
Controls are random global sections, matched labels, coordinate isomorphisms,
the prime-to-p torsion section, path/braid checks and three limb ablations.

## State

The protocol, seven-curve cohort, proof map, Lean proof layer, producer, and
independent verifier are published in two draft PRs. Lean build/proof gate
passed 46 declarations with zero failures and zero trusted-native use. Producer
self-test and structural-test passed all 2,274 nonidentity curve points, 6,822
cached carries, 6,822 pair windings, 13,644 lifted paths and 2,274 zero braid
residuals without scoring labels. The verifier's fixed synthetic fixture,
11,520 ID round trips and CRT tests pass. No candidate run, raw output, summary,
selected ID, decoder result, timing or speedup measurement exists. The next
blocked requirement is uploading this checkpoint and obtaining an immutable
EKS image digest.

## Next steps

1. On this crypto worktree, install a non-expired upload token and run
   `scripts/checkpoint.sh push`, followed by `scripts/checkpoint.sh gate`.
2. Build the immutable Isolab EKS image from crypto revision
   `4ceb5c032917a5f72a15046eee5aa4aad327a93c`, record its digest, and admit one
   Spot job with the frozen USD 1.00 / 1,800-second cap and zero scientific
   retries.
3. Inside that admitted job, compile with
   `rustc --edition=2021 -D warnings research/h114_multiradix_carry_cohomology_20261117/h114_multiradix_lab.rs -o /tmp/h114_multiradix_lab`
   and run
   `/tmp/h114_multiradix_lab --raw /results/h114-winding-raw.tsv --summary /results/h114-winding-summary.json`.
4. Restore the exact outputs into cryptanalysis revision
   `f18430232e2814922d95f67e97abdda5d6ff44f6`, compile its verifier warning-free,
   then run
   `./h114_multiradix_verify h114-winding-raw.tsv h114-winding-summary.json`
   and
   `./h114_multiradix_verify --tamper-test h114-winding-raw.tsv h114-winding-summary.json`.
5. Attach raw bytes, digests, runner receipt, verifier output, all five tamper
   rejections and a claim-labeled interpretation to both draft PRs.

## Runs in flight

None. No H114 scoring process or remote job has been started.

## Stop and success conditions

Hard budget: Isolab EKS Spot only, at most USD 1.00 and 1,800 seconds, with zero
scientific retries. Stop after the exhaustive frozen cohort and controls, or
immediately on any arithmetic/path/braid/digest/metadata mismatch. Success for
the frozen candidate requires the same public predicate ID exact on all 28
rows, all 9,096 targets independently decoded and checked by `[m]G=Q`, every
structural/control requirement passing, normal verifier acceptance and all five
tamper mutations rejected. Empty uniform intersection is a clean null result.
Any incomplete, over-budget or unverifiable run is invalid/censored, not a
scientific result.

## Open questions

The checkpoint uploader and AWS CLI are absent in the current runtime. An
authorized upload token or credentialed runner is required before the gate can
pass. The immutable image digest and EKS job receipt do not yet exist.
