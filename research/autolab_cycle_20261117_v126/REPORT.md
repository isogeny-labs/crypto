# C126: clipped hard diagnostic,13-field odd lifting and14 product trees

The changed conditional GF(2^83) k3 diagnostic cold-rebuilt and hashed the degree-clipped47,663-variable graph. It retained every theta/q, degree, numeric factor, quadratic/cubic multiplicity, raw five-root product, local residue and GF83 prefix check. It returned UNKNOWN(timeout), zero models and no exact original count. Cold compilation was0.143715seconds; native formulation reached3.333558seconds; the40-second soft total target ended at40.064525seconds. Supervisor wall was40.254367seconds, peak631,872KiB RSS. Smaller formula/setup cost has not resolved the search; these are development observations, not isolated speed or memory ratios. All other strata/source/U/outer families remain open.

A new odd-characteristic implementation clips out-of-envelope factor coefficients before subsequent Sylvester/Hensel steps. Coprimality of Z^2 and(Z-1)^3 makes the correction matrix invertible, including characteristic3. Induction preserves every genuine bounded pair; exact product equality certifies the converse. Independent tests passed208 planted pairs,208 comparisons with the previous full lift,48 exhaustive-small-factor reference cases,416 correction-map checks,208 field inverse checks,221 exact product checks (including13 explicit repeated-factor cases),and13 unsupported-leading-form dispatch checks. The13 coefficient fields are GF3,5,7,11,9,25,27,49 and NIST P-192,P-224,P-256,P-384,P-521. Source presets are preserved verbatim. This proves component behavior on the tests, not prime interval/curve/original-root coverage, hardness, or cross-field timing gains.

The fresh search hypothesis after UNKNOWN changes multiplication association in the final exact certificate. Polynomial associativity preserves every constraint/root value, including on invalid inputs. All14 fixed-leaf-order binary trees were frozen before costs were observed. They passed458,752 exhaustive d2 assignments,2,800 supported d3 assignments,1,792 random d3 assignments and224 actual-chart full observable-vector checks. The training rule selects the right-associated tree0:[0,[1,[2,[3,4]]]],47,422 variables/169,937 clauses,graph c3ce8ebaecb0114f3f1242388a7e3c5680ab75619cf76cc4631b88466feac2f8. This saves914 clauses(0.535%)against170,851. The algebraically attractive2+3grouping costs175,196 or175,394 clauses: its proposed size benefit is refuted. Every arm, cost and hash is retained; left-associated tree13 reproduces the prior graph exactly. This is exposed training calibration, not a qualified holdout comparison.

Process records:

{
  "degree-clipped-search-20261117-v126": {
    "status": "ok",
    "wall_seconds": 40.25436736800475,
    "peak_process_rss_kib": 631872
  },
  "odd-clipped-oracles-20261117-v126b": {
    "status": "ok",
    "wall_seconds": 2.8535471179930028,
    "peak_process_rss_kib": 653608
  },
  "product-tree-oracles-profile-20261117-v126c": {
    "status": "ok",
    "wall_seconds": 18.72644181898795,
    "peak_process_rss_kib": 645544
  }
}

Decision: do not repeat the same clipped timeout or presume balanced/clustered multiplication is cheaper. Prepared C127 instead uses the generator identity r2=r1+a: the pair product constant coefficient is r1^2+a*r1, and raw binary polynomial squaring is wiring. This prototype has static preflight only and must beat the selected right-associated tree under independent exact-value checks before a new hard search. The known Frobenius identity does not imply novelty or universal prime-field benefit.

All three workers were registered with frozen source/input hashes,90-second outer deadlines,4GiB and CPU0. Raw output and reporting-boundary guards are preserved. One registration helper initially referenced an obsolete plan filename and aborted before manifest mutation; the corrected helper used the existing uppercase plan. The unregistered resume, original files and corrected plan snapshots are retained. No scientific failed execution was retried unchanged.

The20-hard-instance all-roots matched-F4/F5/F6 release gate is unchanged and unpassed. Neither2x nor50x is established. The conditional norm support/factor components do not replace missing original coupled root recovery, full field/curve coverage or matching baselines. The program remains active. Portable replay verifies source/raw hashes and rebuilds all14 product graphs without rerunning native search.
