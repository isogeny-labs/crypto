# Genuine F3 curve census A — frozen before the first call

This is native correctness/fixture qualification, not a timing experiment or a
solver comparison. All statements and observations remain unsigned.
[pcc:unverified]

## Population and deterministic selection

Enumerate every monic polynomial of degrees3 and5 over F3 in ascending
coefficient-vector lexicographic order. Trial division by every monic polynomial
of degree at most half the degree decides irreducibility completely. There must
be8 irreducible cubics and48 irreducible quintics. Form all28 unordered distinct
cubic pairs and their degree6 products, and retain all48 quintics. These are76
actual smooth genus-two models y^2=f(x), with Frobenius root types3+3 and5+1.
No quadratic twists, parameter searches, target scalars or random seeds occur.
This complete fixed universe is not a census of F3-isomorphism classes.
[pcc:unverified]

Count every affine (x,y) over F3 and F9=F3[i]/(i^2+1), adding2 rational points
at infinity for the monic sextics and1 for the quintics. Retain every coefficient
and every per-x fibre count. Exhaustively check the field laws, inverses and
x^9=x for all9 elements, including all729 triples before the curve counts.
The representation is a+3b for a+bi. [pcc:unverified]

Set t=4-N1 and b=(t^2-10+N2)/2. P(T)=T^4-tT^3+bT^2-3tT+9.
Check parity/integrality, the Weil interval conditions, and the predicted mod2
polynomial: Phi3^2 for3+3; Phi5 for5+1. Change N2 to N2+2 in a separate copied
pair for each model, require the mod2 check to reject it, and restore pristine
inputs. A failed assertion is a failed first outcome, never a reason to drop a
curve. [pcc:unverified]

Compute all #J(F3^n) for n=1..12 in3+3 and n=1..20 in5+1 using Newton traces
and the exterior-square formula. Independently compare all1296 orders with
fraction-free exact determinants det(I-F^n) of the integral companion matrix.
Check positive u64 range and exact divisions, using i128 intermediates with
overflow checks enabled in the compiler. Retain complete arrays and all
2-adic valuations. Their valuations must be even in3+3, and divisible by4 in
5+1; the horizon orders must be divisible by4096. [pcc:unverified]

Record elliptic factors exactly when t^2-4(b-6) is a square and the two integral
traces are in the F3 elliptic Weil interval. Compare every extension order with
the product of the two elliptic orders in that case. Record ordinary status
(3 does not divide b). Select the first lexicographic3+3 coefficient vector
having horizon v2=12 separately for (i) split elliptic factors and (ii) ordinary
non-split factors. Missing selections remain negative results and require a
newly frozen successor population. [pcc:unverified]

The known-answer curve x^6+x^4+x^2+2 has N1=2,N2=20,t=2,b=7,
P=(T^2-T+3)^2 and #J(F3^12)=283875840000=4096*69305625. Require it to occur
and satisfy these values. It is a control, not a newly discovered transfer.
[pcc:unverified]

## Native execution boundary

Run only the published, fully read-back census_a.rs through run_first_A.sh.
Rust1.99.1 standard library only; no Cargo dependencies, Python mathematics,
network calls, paid compute or hidden backend. Versions, rustfmt --check,
release rustc -Dwarnings -Copt-level=3 -Coverflow-checks=yes, strict
clippy-driver, then one native census. Each phase has120 seconds,2GiB address
space, one caller/worker, core dumps disabled. No scientific timing is claimed.
Cargo metadata is unnecessary for this standalone qualification; future
production integration remains separate. [pcc:unverified]

Publish full source/protocol/caller/metadata/lock before the first compiler or
native call and compare every input with full GitHub readback. The metadata
script performs only file/Git/hash/JSON custody. Distinct write-once BUILD,
PRE_CALL and POST_CALL manifests bind copied source, pinned compiler tools and
installed runtime files. Run a separately copied mode0555 ELF, with a pre-call
hash and post-call guards. Use fresh absolute RAM output/temp directories;
retain all argv, exits, stdout and stderr. Stop on the first nonzero phase.
Do not overwrite A if a format/lint/correctness failure needs a successor.
[pcc:unverified]

All curve rows are mandatory; the curve-row output must be below60000 bytes.
Preserve the entire output, not samples. Whole ELFs/large traces remain pending
remote custody until an actual approved store/CI upload succeeds; private RAM
paths are temporary. Existing artifact policy, ledger, keyring, verifiers and
CI rules are unchanged. [pcc:unverified]

## Interpretation and next unit

This qualifies actual curves, point counts and order constraints. It does not
compute genuine J[8] coordinates, a Frobenius matrix on them, the mod8 Weil
pairing, a Richelot/divisor map, prime-order DLP transport, a Lean proof or an
ECDLP speedup. The next coordinate unit requires a separate frozen protocol.
No repeated CT332 abstract lifts, native genus3 A/C tables or old cost16 run.
[pcc:unverified]
