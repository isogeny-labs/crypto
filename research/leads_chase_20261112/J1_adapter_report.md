# LEAD-J step 1: can N4v2's D family run on a generic plane quartic? (read-only study)

Paths: V2 = research/quartic_n4v2_20261020, N4 = research/quartic_n4_20261019,
PIC = src/cryptanalysis/plane_quartic_pic.rs. All line numbers verified by reading the files.

## Verdict
Yes, with a small adapter. Relations, graph/cycle rows, linear algebra, descent and log recovery use only
the plane quartic F (15 coefficients), rational points, and PIC class arithmetic. The GHS data enters in
exactly three places: (a) the builder `Inst::search`, (b) the source of the three descent classes
(`transfer_cubics`, an E0-to-Jac map), (c) the final check `ec.mul(g,k)==target`. Three hidden
restrictions remain (q = 3 mod 4, q < 2^31, ell < 2^64) and one hard prerequisite: you must supply #J and ell.

## (1) Instance fields the solver reads
`solver2::run_family(inst:&Inst, cfg, vars, k_true, seed, verify_rel)` (V2/solver2.rs:96):
* `inst.q` (:102), `inst.ell` (:103, :453), `inst.f:[u32;15]` (:105, :118). The curve is the 15
  coefficients in the order of `pic::mons_of_degree(4)`; `ic::Curve::new(q,&f)` (N4/ic.rs:187) copies them.
* `inst.ec` and `inst.g` (E0 Weierstrass curve over K=F_{q^3} and its order-ell generator): used only for
  `target = ec.mul(g,k_true)` (:108), descent start points r0=rG, r1=r0+G, r2=r0+T (:543-545), and the
  verifier `ec.mul(g,kk)==target` (:605-606).
* `inst.transfer_cubics(&p)` (:548; defined N4/ghs.rs:478): needs `om`, `qa..qd`, `u`, `u1`, `u2`, `kf`
  (E0-to-quartic cover map ec_to_quartic/transfer_w, ghs.rs:400-474). This is the GHS cover map.
* Not read by the solver: `order`, `tries`, `u*`, `qa..qd`, `om` (except through transfer_cubics).
* Base point: not an Inst field. run_family takes the first factor-base point for which
  `PlaneQuartic::new(q,f,pt)` succeeds as P0 and swaps it to fb[0] (:115-127). P0 is therefore random per
  (seed, attempt) and Pic^0 canonical forms depend on it. Inputs P,Q must be base-point independent
  (point-list divisors) or P0 must be forced (see (5)).
* Nothing reads "4 | #E0" or the cofactor in the solver. The cofactor 4 appears only in the builder
  (ghs.rs:308 `n%4`, :311 `ell=n/4`, :325 `g=4*rand`).
* The D arm uses `Fam::D`, `Cfg{c=c_D=2.0 default, rows_mult 1.2, gamma 4, block 4, max_desc 1e5}`
  (v2.rs:257-294); nfb = ceil(c*sqrt(q)) (solver2.rs:113).

## (2) `preamble(q, seed, arms, true)` (V2/v2.rs:193-218)
1. `Inst::search(q,seed,200000)` (:195; N4/ghs.rs:266-397), the GHS-specific core: random u=(c0,b0) in K,
   u1,u2 Frobenius conjugates, f0 = u*u2 (quartic over K), E0: w^2=V^3+qb V^2+a4 V+a6 (ghs.rs:297-303);
   `ec.order_bsgs` = Hasse-interval BSGS over K (ghs.rs:182-224, :304); require n%4==0 and ell=n/4 prime
   and < 2^64 (:308-313); generator g=4*random (:322-331); build the plane quartic F over F_q from
   om^-1 and R^-1 (:334-367) and normalise; `pic::smoothness_certificate` (:374) gate.
2. Seeds an Rng; measures E0 add cost `gae_mul/gae_inv` over 100 counted additions (:198-208). Only used
   for GAE pricing of rho/unpriced work, and E0-specific.
3. If `print`, calls `ratios(&inst)` (v2.rs:37-73) which uses only q and ell (timing of F_q mul, inv,
   mod-ell mul) and prints the `ratios` record (:211-214). `calib_cmd` (:116) does the same robustly and
   also builds an Inst only to get q and ell.
4. Draws the planted k = rng.below(ell-1)+1 (:216). GHS-specific parts: step 1 entirely, step 2.
   Generic parts: step 3 and the record format.
   `ic_cmd` (:220-300) passes `inst` and k only into `solver2::run_family`; D arm is :257-294.

## (3) Weak-family dependence beyond the quartic model and Jac arithmetic
Checked solver2.rs (all), ic2.rs (find_relations_2lp :400+, build_graph, cycle_rows, propagate_logs), la2.rs,
N4/ic.rs (Curve, roots, split3). Result: none, except the descent source and hidden field restrictions.
* Factor base: `sample_points2` (ic2.rs:66) draws random rational points via random lines; generic.
* Relations: all pairs of FB points, residual quadratic from gradient/half-Hessian local data
  (ic2.rs:346-399, 400-590); relation = line section Sum = kappa (hyperplane class) with eta column =
  kappa; no 4-rational-point or cofactor hypothesis. Pure plane geometry.
* Graph/cycles, SGE, Lanczos/block Lanczos (la2.rs): linear algebra mod ell on integer relation rows. Any
  ell works (u64). Rows use to_mod(-x,ell); lm2 uses u128 (ic2.rs:43-47).
* Verification of relations (`verify_d` solver2.rs:649-704) uses only `cv.point_class/add/mul_scalar`.
* Descent (solver2.rs:524-593): start classes come from E0 via transfer (the GHS cover). The loop itself
  (random FB-class addition until `PicClass::Typ` splits into 3 rational points via `split3`
  N4/ic.rs:559, then log lookup) is generic. Result k = (y2-y0)/(y1-y0) mod ell (:599-604) needs only that
  y_i = log(class_i) with class_1-class_0 = P and class_2-class_0 = Q, so any start class R works.
* Mathematical preconditions the generic Jacobian must meet (true for GHS by construction, unchecked by
  code): ell | #J(F_q) (else the mod-ell kernel does not exist, LA fails); ell || #J or at least a cyclic
  ell-Sylow containing P (else kernel dimension > 1, reported as "kernel not unique" / LA failure, :387-389);
  P has order ell and Q = xP. Logs are the hom J -> J/(prime-to-ell part) = Z/ell, so cofactors in #J are fine
  and the GHS cofactor 4 is not special.
* Hidden restrictions (coincide with GHS choice q = 7 mod 12, but are not weak-family properties):
  - q = 3 mod 4: `e_sqrt=(q+1)/4` and `r*r==disc` test (ic2.rs:410,459) treat a non-square as "residual
    irreducible" silently; `Fp::sqrt` is "q = 3 mod 4 only" (common.rs:227-243, used in ic.rs:138). For q = 1 mod 4
    relations would be silently dropped (a quiet failure, not a crash). Fix = Tonelli-Shanks in Fp::sqrt
    and ic2.rs:459.
  - q > 3 (inv 2, 6). `Kf::new` asserts q = 7 mod 12 (common.rs:262) but only the GHS builder needs it.

## (4) Point counting / L-polynomial / Jacobian order
* PIC has none in the library. Only in `#[cfg(test)] mod tests`: `jac_order(q,n1,n2,n3)` (PIC:1862),
  `enumerate_points` (:1897) and `ext_points` (:1803), brute force over P^2(F_{q^k}) with a double loop over
  (y,z), i.e. O(q^{2k}); N3 costs q^6, usable only for q of about 10-20. Not compiled outside tests.
* `src/cryptanalysis/curve_construction/lpoly.rs`: `char_poly_from_counts(&[N1,N2,N3], q)` gives the monic P(T)
  with P(1) = #J (Newton identities + functional equation); self-contained (no `crate::` imports). It needs
  N1..N3, so it only helps if N1..N3 are available (small q, or a faster counter written for the purpose).
* No Kedlaya/Harvey/Schoof-Pila genus-3 counter was found (grep of src/ research/ scripts/ tools/; no sage/gp/magma
  on the host). `bielliptic_quartic.rs` is a q <= 257 diagnostic (MAX_PRIME 257), not a counter.
* What GHS uses: it never counts #J. It counts #E0(K) = n by BSGS over F_{q^3} (ghs.rs:182-224), takes
  ell = n/4, and relies on J(C) ~ Res_{K/F_q} E0 (isogenous => #J(F_q) = n). Checked after the fact by
  `v2 homcheck` (v2.rs:124-181): hom triples and `#J * x == 0` on 10 classes (:167-177).
  Consequence for a generic C: #J and ell must be an input (from the lead's construction, e.g. J with known
  Weil polynomial, or brute-force counts at tiny q). A cheap necessary check is `mul_scalar(P, ell) == zero`.
  For generic quartics at q ~ 2^15+ no #J oracle exists in-repo, so step 1 should use curves whose J is known.

## (5) Smallest adapter (new subcommand `v2 icx <inst.json> <arms> [cD gamma block]`)
Inputs (flat JSON; summ2::parse_line, V2/summ2.rs:10-31, already parses numbers, strings and `[..]` arrays
without any crate): `q`, `f` (15 ints, mons_of_degree(4) order), `ell`, `p0` ([x,y,z], optional), `P`, `Q`.
P and Q given as divisor classes of rational points: `P_plus`/`P_minus` lists (equal length) meaning
Sum[p]-Sum[m], evaluated with `cv.point_class` and `cv.add/sub` once cv exists (base-point independent);
optionally Typ conics via `cv.class_from_conics` (PIC:1035) when P0 is forced. Optional `x` for self-check.
Edits (estimates, rustc-only build, no new deps):
1. solver2.rs: replace `inst:&Inst, k_true` in `run_family` (:96) and `downstream` (:439) by a small
   `Src` struct: `q, ell, f, p0:Option<[u32;3]>`, and two closures/trait methods
   `start_triple(&mut Rng, &PlaneQuartic)->Option<[PicClass;3]>` (replaces :542-552, GHS: E0 + transfer_cubics +
   reduce_deg6; generic: R = sum of 2 random FB point classes, then [R, R+P, R+Q]) and `check(k)->bool`
   (replaces :605: generic `cv.eq(cv.mul_scalar(P,k), Q)`). `target` (:108, :421) becomes Source-owned. ~35 lines.
2. solver2.rs :115-127: if `p0` is given, force fb[0]=p0 and build `PlaneQuartic::new` with it; fail if it does
   not construct. ~8 lines. (Needed only if P,Q are passed in canonical Pic^0 form.)
3. Keep the GHS path: add `impl From<(&Inst,u64)> for Src` wrapping the existing code, so `v2 ic` output stays
   bit-identical (replay check with `v2 replaycmp`). ~20 lines.
4. v2.rs: `icx_cmd` (~70 lines): parse JSON, build Src, loop the D arm exactly as :257-294 (c-escalation
   2.0, step 0.25, 6 attempts), print the same `ic` record; `k` printed as the supplied x or `null`. Pre-check
   `ell` prime (common::is_prime), `ell < 2^64`, `q%4==3`, and `mul_scalar(P,ell)==zero` (compute with cv from
   a found point) with clear errors. Ratios record: call `ratios2` (v2.rs:78) which only needs q and ell, so
   make it take `(q, ell)` instead of `&Inst` (~4 lines); `ratios`/`calib_cmd` likewise.
5. main(): one more match arm (:357). 2 lines.
Total about 140 lines changed/added; no change to ic2.rs, la2.rs, rho2.rs, summ2.rs, pic.
Not in scope of the adapter: rho. `rho2::rho_plain/rho_neg` run on E0 via `inst.ec` (rho2.rs:28-210), so the
matched-rho reference for a generic Jacobian needs a Jac-class rho (cv.add per step, count via
`cv.counter()`), about 60 lines more, or the 1/2-sqrt(pi*ell/2) model priced per-step (see (6)).
Validation test for the adapter: export a GHS instance (f, ell from results/instances.jsonl), derive P,Q from
G,T via transfer_cubics, and confirm the generic path recovers the same k as `v2 ic` (v2.rs:127-165 has the
needed transfer-to-class code in homcheck).

## (6) Reporting and cost counters
Record (v2.rs:284-287), one JSON line per arm and attempt: `rec:"ic"`, `arm` (D/DL1/DL), `q`, `seed`,
`attempt`, `c`, `ell`, `k` (planted), `k_found` (u64 or null), `ok` (bool), `fail` (string), then per phase
(`fb, relations, matrix, linear_algebra, log_recovery, descent, verify`, summ2.rs:40) the flat counters
`<phase>.{mul,inv,add,lmul,ladd,hash,cmp}` plus `.wall` and `.cpu`, and `s.<stat>` fields (rows, cycle rows,
holdout_ok/bad, descent_trials, ...).
* `ok` is true iff `ec.mul(g,kk)==target` (solver2.rs:605-606); for generic use the class check above.
  `fail` values: "no base point", "no full relation", "too few relations", "linear algebra failed",
  "kernel not unique (hold-out relations violated)", "no known factor-base logs", "no valid transfer triple",
  "descent exhausted", "degenerate denominator", "recovered k does not satisfy kG = Q".
* Counters (common.rs:9-16): global statics MUL, INV, ADD (F_q ops; `Fp::mul/inv/add` bump them, including
  inside Kf, so GHS E0 work in descent is charged), LMUL, LADD (mod-ell), HASH, CMP (hash and partition ops,
  unpriced). `pc()` (solver2.rs:46) adds PicClass-op counts from `cv.counter()` into the globals. `Meter`
  (:59-89) takes per-phase deltas; uncharged control blocks (relation verification) are excluded via
  pause_start/pause_end (:76-88). Counters are per-unit replayable (v2 replaycmp).
* M-equivalents (V2/summ2.rs:177, 208): meq = mul + r_inv*inv + r_lmul*lmul, summed over phases; failed
  attempts are charged (all attempts of a unit are summed). Unpriced native work `unp = hash + cmp`, priced at
  w in {0, 0.1, 1} GAE = `meq + w*unp` (summ2.rs:88-89, 103-104) with GAE per operation = gae_mul + r_inv*gae_inv
  from the E0 add measurement (summ2.rs:157), i.e. E0-specific; a generic arm must re-measure one unit of
  its own native op (one Jac add) for the GAE unit, per AGENTS.md section 5a.
* Calibration rule (PARAMS.md "Measurement deviation"): r_inv and r_lmul are NOT N4's tight-loop `ratios()`
  (v2.rs:37, unusable under contention) but the median over three `v2 calib q seed` records per size
  (`ratios2`, 600 interleaved triples of 20000 mul / 2000 inv / 20000 mod-ell mul, ratio formed per triple, median;
  v2.rs:78-114). `lmul` ratio rises when ell passes about 2^50 (u128 remainder slower; README). Sensitivity
  runs use fixed N4 constants `N4V2_FIXED=15,1.1` (summ2.rs:143-154). For rho the frozen calibration check is
  steps/sqrt(ell) in [1.0, 1.5] (README controls).
* Since gamma/block/cD are CLI positions 7-9 of `v2 ic` (v2.rs:358-366), the D family defaults c_D=2.0, gamma 4,
  block 4 apply unchanged.

## 128-bit and size limits (summary)
* q < 2^31 (PIC:368 `is_odd_prime_below_2_31`; `pack2` 2x31 bit packing ic2.rs:11-21; Fp u64 mul safe). `ic::pack`
  (N4/ic.rs:169) is 21-bit but only B0/S paths use it.
* ell < 2^64 (u64 everywhere; la2/lm2/ls2 are overflow-safe with u128 intermediates, ic2.rs:24-49);
  `mul_scalar` takes u128 (PIC:1138). GHS builder rejects ell > u64::MAX (ghs.rs:312). v2 README: N4's
  own `check` overflowed at k=22 (ell ~ 2^64) and `homcheck` fixed it with 128-bit sums (v2.rs:136).
  Prime-order J (#J ~ q^3) therefore fits only for q < about 2^21; for larger q a cofactor is needed so ell < 2^64.
* Elliptic-side u128 (`Ec::mul(k:u128)`, n0=q^3+1 in order_bsgs) is irrelevant to the generic path.
* Runtime: D family cost about q^(1.0) M-eq (README); sample points nfb = 2 sqrt(q), all-pairs search O(nfb^2).
