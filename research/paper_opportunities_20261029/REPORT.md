---
title: "Paper opportunities in index calculus and fall degrees: which paper to submit"
date: 2026-10-29
---

# Paper opportunities in index calculus and fall degrees

**What this is.** A search across `crypto`, `crypto-autoresearcher` and
`cryptanalysis` for the best paper the programme could submit. Scope, as the
owner set it: index calculus and first/last fall degrees (ffd, lfd). The
ECC2K-130 GPU paper is out of scope because it is already public as IACR ePrint
2026/2469 (14 October 2026).

This is a read-only assessment, plus one re-verification (§6). It ran no
experiment, changes no research state, and promotes nothing. Candidate results
are quoted at the tier their sources give.

## 1. Recommendation

**Submit one paper on the binary Semaev descent: "The first fall degree is
constant, the last fall degree is not: proofs and checkable certificates for
summation-polynomial systems."** Post it to IACR ePrint, then submit to the
*Journal of Mathematical Cryptology* (where Kousidis–Wiemers 2019 appeared) or
to *Designs, Codes and Cryptography*.

Its core already exists in
`crypto-autoresearcher:analysis/semaev-m2-last-fall-degree/` (721-line theorem
dossier, two independent implementations, a certificate checker). Fold in the
explicit-membership "guarded" chain theorems from
`crypto:research/guarded_fall_degree_20261028/`.

Why this one:

1. **It answers a stated open question.** Huang–Kosters–Yeo (CRYPTO 2015) and
   Kousidis–Wiemers (JMC 2019) both name rigorous bounds on the last fall
   degree of summation-polynomial Weil descent as open. The only post-2020
   lower bounds I found are for other systems (iterated MiMC-type systems,
   arXiv 2310.03637). Huang 2103.07282 and 2311.02804 give upper bounds only.
2. **It replaces solver observations with intrinsic, independently checkable
   statements.** Kosters–Yeo (2015) and Kousidis–Wiemers (2019) already saw
   Magma/F4 *step degrees* reach 5. Those are properties of a run. The
   dossier's statements are properties of the system: $1\notin W_4(F)$, hence
   $d_{\rm LFD}\ge5$ and *every* F4 run reaches degree $\ge5$. A short checker
   that shares no code with the producer verifies each one.
3. **It has proofs, not only data.** All proved for every $n$ and $V$:
   - an exact reformulation of $S_3$;
   - $d_{\rm ff}=2$ for the whole chained system and every $t$, where Semaev's
     ePrint 2015/310 states 4;
   - $d_{\rm LFD}\le k+2$, and $(t-1)k+t$ for chains;
   - $d_{\rm LFD}\ge3$ under checkable conditions.

   The guarded study adds $(d_{\rm ff},d_{\rm LFD})=(4,5)$ *exactly* for every
   $p>3$ at $B=3$, the binary analogue (exact except the even-degree cube
   orbit), and $\max(5,B+1)\le d_{\rm LFD}\le3B-4$.
4. **It is honest about scope.** Its §5 already says it proves no growth to
   infinity and makes no cost claim.

Figure 1 shows the central separation: the first fall degree stays flat while
the last fall degree climbs.

![Figure 1. Left: binary Semaev $m=2$ descent. First fall degree 2 (proved),
exact last fall degrees per instance, and certified lower bounds
$d_{\rm LFD}\ge5$ (open triangles) at $n=35,37,39$ (random $V$) and $n=45$
(Semaev's default polynomial $V$). Right: prime-field Kummer tower ($m=2$). The
session-reported $d_F$ series lies on PKM 2016 Table 4's corrected degree of
regularity. The growth is prior art; the proved constant ffd $=4$ and the
$2t+2$ certificate are ours. Values and sources:
`figures/data.json`.](figures/fall-degrees.pdf)

## 2. What must be true before submission (gating)

| # | item | why | effort |
|---|---|---|---|
| G1 | **Publish the certificates.** Only their SHA-256 hashes are committed (`results/certificates.txt`); the files are not in any repository. Regenerate them (§6 shows this works) and deposit them in an archive (P45a is about 0.3 GB). | A referee must be able to run `cert_check.py`. | 1–2 days of compute |
| G2 | **More instances per $n$.** Only 2 instances per $n$ at $n\ge33$; 1 polynomial-$V$ instance at $n\ge41$. Certify about 10 per $n$ for $n=33\ldots45$, and push to $n=47\ldots51$ to see whether 5 becomes 6. | Makes the step $4\to5$ a population statement rather than a handful of instances. | 1–3 weeks on a large-memory host (P45a closure took about 6,000 s) |
| G3 | **Expert proof review** of Thm 2.1, Cor 2.2/2.7, Prop 2.6, Thm 2.5, Prop 3.7, and the guarded B1–B4 proofs. | All are written proofs; no Lean beyond small identities. | external |
| G4 | **One fall-degree convention.** Use Caminata–Gorla closure $V_{F,d}$ for $d_{\rm LFD}$ and CG Def. 1.3 for $d_{\rm ff}$. State how the Petit–Quisquater reading differs; the 2026-10-22 replay counterexample shows the reading matters. | Reviewers in this area check definitions first. | days |
| G5 | **Literature section, read in full.** Read: Semaev 2015/310 (latest version), Kosters–Yeo 1503.08001, HKY 2015/573, HKYY 1505.02532, Kousidis–Wiemers (JMC 2019), Huang–Petit–Shinohara–Takagi 2015/358, Caminata–Gorla, Gorla–Müller–Petit, Huang 2103.07282 and 2311.02804. State plainly that "Assumption 1 fails" was already observed empirically (KY 2015 at $n=45$; KW 2019). The new part is the intrinsic certificate and the proofs. | Avoids a novelty overclaim. | 1 week |
| G6 | **Cite B5/B6 correctly or drop them.** The Boolean upper bounds $3\ell+1$ and $3\ell+2$ are weaker than the dossier's own $2k+3$ and are standard brute-force PC refutations. | Keep the paper's claims tight. | hours |

**Stretch, which would move the venue up.** A *proved* lower bound on
$d_{\rm LFD}$ that grows with $n$ (or $k$) for a summation-polynomial system
would resolve the open question outright, and would merit CRYPTO or EUROCRYPT.
Semaev's own abstract concedes his asymptotics survive if the regularity
degree is $o(\sqrt{n\ln n})$. So only a growing lower bound, not the
certificates, bears on the subexponential claim. The dossier's §4 heuristic and
the regular "$4n$ new cubics per round, then stall" closure pattern are the
starting points. Treat this as research, not as a submission requirement.

## 3. Second paper: prime-field Kummer towers (PKM)

`crypto:research/prime_fall_degree_20261027/`, written proofs D1–D8:

- **ffd $=4$ for every tower height $t\ge2$** (D3, via $\det M=-2g^6(3x_R^2+a)\neq0$);
- certificate degree $2+\mathrm{popdeg}(f^{-1})\le2t+2$ (D4);
- **presentation dependence**: the naive univariate encoding has $d_F\ge2^t$ (D5);
- a single summand has $d_F=2$ for every $t$ (D8).

Two facts limit it today:

1. **The measured growth is prior art.** PKM 2016's own Table 4 (prime case,
   $p-1$ subgroups) reports corrected $D_{\rm reg}$ 4,4,4,4,4,5,5,5,5,6,6 for
   $n_1=1..11$. Our session-reported $d_F$ for $t=1..10$ is 3,4,4,4,4,5,5,5,5,6
   (Figure 1, right).
2. **The raw data is lost.** `runs/`, `TABLE.md`, `RESULTS.md` and four
   `derived/*.md` files were never committed (`RESTART.md`). Stage A at
   $t\ge11$, stage C and the registered prediction $d_F=4+\lfloor(t-2)/4\rfloor$
   are untested.

Best use: regenerate stage A for $t=1..14$ with closure certificates (as in
G1), score the registered prediction, and publish either as a short JMC/DCC
paper or as one section of the main paper. Its contribution is the *proved*
constant ffd beside a growing $d_F$ in the setting PKM introduced, plus the
presentation-dependence theorem.

## 4. Short notes worth an ePrint each

- **Nagao Lemma 4** (`crypto-autoresearcher:papers/nagao-lemma4-note/`).
  - Under the equality reading, $d_F\le d'_F$ fails: $\{X^2Y,XY+X\}$ gives
    $d'_F=2<d_F=3$.
  - 94 of 400 random multilinear pairs violate it.
  - Owed: typeset it, have a human check it, and contact the author.
- **Fall-degree definitions matter**
  (`crypto-autoresearcher:docs/reports/2026-10-22-semaev-first-fall-replay/`).
  - The finite instance $n=n'=4$ has operational first fall 3, where the
    literal Petit–Quisquater reading gives 2.
  - It contradicts no published theorem: Kosters–Yeo say "usually 2". It
    belongs as a remark in the main paper's definitions section, not as a
    paper of its own.

## 5. Considered and not recommended (index-calculus scope)

| candidate | why not now |
|---|---|
| Plane-quartic (Diem) IC on GHS-weak $E_0/\mathbb F_{q^3}$ (`research/quartic_n4v2_20261020`) | Exponent $\approx0.98$ against rho's $\approx1.5$, but this is known: Diem 2003/2006 and Diem–Thomé 2008 ("faster than rho even for small group sizes"). Joux–Vitse 2012 solved 149-bit $\mathbb F_{p^6}$ curves. Also L0 host, B0 records do not replay, 3 units per size. Publishable only at record scale. |
| $k=4$ $D_4$ Frobenius-reduced form, char 2 (`gaudry_quartic_d4_char2_20261015`) | The red team (`gaudry_k4_char2_redteam_20261026`) rated the crossover "NOT SURVIVED (unsupported)": 1–2 targets per rung. Invariant-ring work in char 2 is Galbraith–Gebregiyorgis 2014. The cheapest headline experiment (about 30 h, frozen plan) is worth running, but the paper waits on it. |
| Genus-3 hyperelliptic GHS with double large primes | Gaudry–Thomé–Thériault–Diem 2007 already beat rho in genus 3; the slope against rho is not robust (`ghs3_vs_rho_20261023`). |
| Root-finding decomposition (`papers/rootfinding-decomposition`) | Prop. 1 is the definition of $S_3$ (McGuire–Mueller; Amadori–Pintore–Sala). Engineering only. |
| Semaev monodromy / conservation (`crypto-autoresearcher:papers/semaev-conservation-specialization`) | The Galois group $(\mathbb Z/2)^{m-2}$ is plausibly folklore (sign choices of $y_i$); the paper itself says novelty is not adjudicated. |
| IC improvement ledger / programme record / audit methodology | Records, not venue papers; 0 of 28 IC leaderboard rows are validated. |

## 6. Re-verification run for this report

To test gating item G1, I regenerated one certificate from the committed code
and checked it with the independent checker.

- Instance T33a: $n=33$, random $V\subset\ker\mathrm{Tr}$.
- Producer: `code/lfdclose2.c`, compiled against M4RI 20200125.
- Checker: `verify/cert_check.py`.
- Host and isolation: this cloud container (4 vCPU, 15 GB), L0. Only the
  verdict matters here, not the timing.

**Result.** The certificate reproduces **byte for byte**: SHA-256
`cf9dcae81bf1843fa55aeb5f5d3745b69a50790818b7eb48b161930c343a22cf`, the value
recorded in `results/certificates.txt`. The checker reports
`VALID: 1 is not in W_4(F), and codim W_4(F) >= 4888`.

| quantity | value |
|---|---|
| closure dimensions $\dim W_4\cap B_{\le d}$ | (0, 0, 64, 2968, 48068), the recorded values |
| closure rounds | 3 |
| certificate size | 33,044,576 bytes |
| closure time | 4 min 40 s |
| check time | 38.8 s |

So gating item G1 is a matter of compute and hosting, not of lost code. The
other certificates should regenerate the same way, at proportionally larger
cost (P45a about 0.3 GB).

## 7. Method and limits

- **Inventories.** Four parallel read-only inventories: existing drafts; crypto
  research 2026-10-20…29; the crypto-autoresearcher lead atlas, Lean and
  ledger; and the cryptanalysis repository. A fifth, deeper read covered every
  fall-degree result.
- **Literature checks.** Done on the web; the ePrint PDF host refused direct
  download. Read in full: Kosters–Yeo (arXiv 1503.08001) and PKM 2016. Read
  from abstracts or secondary summaries only: the rest, including HKY 2015,
  Kousidis–Wiemers 2019 and FGHR. A submission needs G5.
- **Repository state.** The `cryptanalysis` checkout is at 2026-10-22: a fetch
  failed while the disk was full. Nothing in its fall-degree material is
  distinct from crypto's.
- **Not covered.** No reading of the 43 ECDLP ledger records that have no lead
  in the lead atlas.
