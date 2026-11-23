# Two-choice adding walks: an ensemble counterexample to an unqualified bound

For each prime `p`, sample an ordered distinct pair of increments uniformly in
`ZMod p`, then independently attach a fair binary label to every vertex. Starting
at zero, iterate `x ↦ x + increment[label(x)]`. Let `E(p)` be the expected index
of the first repeated state, averaged over both actual finite distributions.
Lean proves

\[
  p\ge 4m^2(4m+1),\quad m>0\quad\Longrightarrow\quad E(p)\ge m^2/2.
\]

For every natural `C`, some prime satisfies `E(p)² > C p`. Consequently no
nonnegative real constant bounds `E(p)` by `K√p` for all primes in this model.
The increments are distinct; zero is allowed as in the ordered-embedding
sampling law. This is a two-choice negative control for an unqualified extension
of the collision heuristic. [pcc:unverified]

**The qualified larger-partition claims remain open.** The source explicitly
states an `nS ≥ 8` result subject to a conjecture and Teske's `nS ≥ 20`
recommendation. The theorem concerns the least repeat, without asserting Floyd
operation counts, cycle length alone, scalar recovery time or practical hash
performance. It averages supports, unlike the credited fixed20 Sidon family.
Established maximal bounds, finite counting, deferred decisions and Bertrand's
theorem are credited; no novelty is claimed. [pcc:unverified]

The proof derives the probability and path bridges:

1. First-label decomposition of the actual iid-word product law proves the
   squared-boundary hitting bound `W² · hitMass(W,n,s) ≤ s²+n`. At `n=m²`,
   `W=2m`, `s=0`, at least three quarters of words stay inside the boundary.
2. Actual prefix states equal twice the number of label-one steps minus time.
   Actual modular states equal `j·a + count1(j)·δ`. A balanced collision yields
   one of at most `m²(4m+1)` explicit short relations.
3. An explicit bijection identifies ordered distinct supports with a first
   increment and a nonzero difference. For `0<d<p`, each relation
   `d·a+k·δ=0` has exactly one first increment for each difference, giving
   actual support mass `1/p`. No independence of the two distinct increments
   is assumed.
4. The support union bound and word bound give survival at least one half.
   The credited static-hash/iid identity applies to selfavoidance before a
   repeat. The actual first-repeat tail gives the mean bound; Bertrand supplies
   primes at the required scale. [pcc:unverified]

Controls include actual one-step boundary-hit masses `1` and `0`, normalization
and positivity of both label weights, `Σpᵢ²=1/2`, and the degenerate `d=0,k=0`
relation having mass `1` rather than `1/p`. The credited prime-Z3 postcollision
word has static mass zero and iid mass `1/8`, so unrestricted replacement of
static observations by fresh labels remains false. [pcc:unverified]

The [closed certificate](lean/TwoChoiceCertificate.lean),
[proof map](proof-map.json), [source plan](sources.json),
[source review](evidence/source-review.json), [protocol](PROTOCOL.md),
[preflight](preflight.json) and [one-page report](reports/report.pdf) retain the
precise scope. Fresh primary fulltext and visual coverage is Galbraith corrected
ch14 PDF4,5,12,13 of38 and Bernstein–Lange2013 PDF11,12 of28. The original Teske
paper was not read in this stage. Four documented KB calls and Conductor were
unavailable with exit127 before processing/code; bounded local, official and
primary fallback was used. Copyright primary files remain local. [pcc:unverified]

Seven new modules (six concrete and one import adapter) use ten frozen local
modules without copying or changing them. All36 predecessor source/artifact
pins are checked before and after every retained build and gate. Thirteen
numbered states retain seven failures and six successes. The first70-theorem
gate passed, but the separate native header check caught a435-character prefix
of the actual1134-character statement at an inline instance assignment. The
final header gives the instance argument explicitly; the mathematical statement
and frozen verifier are unchanged. Both header inputs and all failed states
remain in the lossless attempt archive. [pcc:unverified]

To restore the source/configuration/log history into a fresh directory:

```sh
python3 research/rho_twochoice_collision_audit_20261115/restore_attempts.py /tmp/twochoice-attempts
```

Python performs provenance and document handling only. No mathematical Python
computation, solver benchmark, paid compute or agent experiment upload was used.
Foundations are `propext`, `Classical.choice`, `Quot.sound`, with default limits,
no holes, custom axioms, native proof evaluation, global flags or waivers.
The native written-header check is procedural; it does not prove compiler,
kernel-parser or independent physical-host refinement. Signed standing is
unverified. Larger-partition collision constants and the remaining obligations
across all14 items remain open. [pcc:unverified]

Final gate: **70 theorems, zero failures and zero trusted-native exemptions**. The complete1090-character written statement matches SHA256`ce864beb518e06e523e89e965cc96ef0d00e05836cae76e1e4d229659cb0d339`; native1complete/0prefix/0unresolved/empty suffix. The lossless archive retains245files/91objects in29268bytes. The final report is one page/48131bytes/all4973textcharacters read and all-page visual/inbounds checked. Seven exact valid-first-draft source/PDF/layout/log files are retained in48140bytes; the regenerable raster stays local after an initial oversize archive. All32formal+19artifact pins match. [pcc:unverified]

Exact local **PCC-20261115-db512a** replay PASS at 2026-11-15T16:05:42Z; signed standing unverified. See [verification receipt](verification.json). [pcc:unverified]

Publication: existing ready [PR3743](https://github.com/isogeny-labs/crypto/pull/3743), remote proof `205935f711c74fd2b835e73aa33bd6615a640e3e` and local receipt `5171fa85fbaf1ccf5b8934089d195f6cd5eff38d` share tree `effbd5c32d9f84cdff1e6c549bd5875ac475a375`. Anonymous fetch completed; fresh PR head/body/open/ready matched. Publication metadata is outside the frozen pinset. [pcc:unverified]
