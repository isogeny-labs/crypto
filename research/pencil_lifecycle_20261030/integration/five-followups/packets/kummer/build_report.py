from pathlib import Path
import hashlib,json,difflib
D=Path('/var/tmp/pencil-five-leads-kummer-20261030')
H=Path('/home/ec2-user/crypto-pr2304-persist/research/kummer_differential_collector_20261022')
s=H/'src/main_labelled.rs'; w=H/'src/walk_impl_uncached.rs'
old='''        let (kd, r0) = xadd_repaired(k, k, k0, t, acts, p);
        let (km, r1) = xadd_repaired(k, kp, kg, t, acts, p);
        let (kpp, r2) = xadd_repaired(kp, kp, k0, t, acts, p);
        let pair = if choice == 0 { kd.zip(km) } else { km.zip(kpp) };
        (pair, choice, [r0, r1, r2])'''
new='''        // Proposed arm: only the selected double is evaluated.
        // None in the unused repair slot means not evaluated, not repair failure.
        if choice == 0 {
            let (kd, r0) = xadd_repaired(k, k, k0, t, acts, p);
            let (km, r1) = xadd_repaired(k, kp, kg, t, acts, p);
            (kd.zip(km), choice, [r0, r1, None])
        } else {
            let (km, r1) = xadd_repaired(k, kp, kg, t, acts, p);
            let (kpp, r2) = xadd_repaired(kp, kp, k0, t, acts, p);
            (km.zip(kpp), choice, [None, r1, r2])
        }'''
a=s.read_text();assert a.count(old)==1
(D/'branch-directed-proposal.patch').write_text(''.join(difflib.unified_diff(a.splitlines(True),a.replace(old,new).splitlines(True),fromfile='a/src/main_labelled.rs',tofile='b/src/main_labelled.rs')))
report='''# Kummer transition closure and a branch-directed candidate

Status: source derivation and private proposal; no new cohort, compilation, arithmetic trial or timing run. The research-visuals skill was used for this report. Date: 2026-10-30. Parent integration must review this candidate before implementation.

## Sources checked
Historical N19 src/main_labelled.rs (step 41-56, lift 138-171, fallback 184-212, advance 215-280, collector 390-540), src/walk_impl_uncached.rs (counters 5-41, repaired arithmetic 63-93, branch 166-177), and n19-labelled-fallback/VALIDATION.md. This report reads historical source directly; it does not claim these files equal the current main build.
Primary literature: Chung, Costello and Smith, Fast, uniform, and compact scalar multiplication for elliptic curves and genus 2 Jacobians, sections 2, 3 and 6.4, https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/recovery.pdf . The paper specifies differential addition from a known difference, adjacent-point recovery, and exceptional-input restrictions. It motivates the abstract interfaces here, but does not certify this Rust implementation. No novelty claim is made.

## Derived closure and labels
Let J be an abelian group, kappa its quotient by sign, and R=aG+bQ. Retain (kappa(R),kappa(R+G)), fixed kappa(G), identity data, and labels (a,b). Assuming correct doubling and differential primitives, calculate D0=kappa(2R), M=kappa(2R+G), D1=kappa(2R+2G). For input-derived c in {0,1}, select (D0,M) if c=0 and (M,D1) if c=1. This is precisely the adjacent pair for R'=2R+cG. Its labels are a'=2a+c and b'=2b modulo r. The difference of the two signed entries stays G. This proves operation-set closure conditionally on the arithmetic interfaces, and matches the existing N19 algorithm rather than proposing a new random-addition walk.

Signed recovery uses the fixed signed anchor G and adjacent quotient pair. The source computes the opposite neighbour with repaired differential addition before conversion and recovery. Exceptional lifting is attempted on D_zero; unresolved conversion and recovery failures remain explicit. Recovering a signed divisor supplies the signs of y0,y1 used by graph.edge with the maintained a,b. A quotient point alone is insufficient: in Z/11, quotient classes of R=2 and -R=9 coincide, but with G=1 the successor classes are [2+1]=[3] and [-2+1]=[-1]=[1], which differ. This is an information obstruction to addition from quotient inputs alone, not an impossibility result with labels or extra state.

## Source-derived pruning candidate
branch(k,p) runs before all three calls and depends only on k and p. It normalizes one coordinate then takes its low bit. Existing step eagerly evaluates both doubles and the mixed term. The selected output uses only two of those three results. The private patch evaluates only those two, in their original relative order. The exact source call-count difference is three versus two repaired-arithmetic invocations per ordinary step; there is no timing or overall cost claim.

This changes observations intentionally: field counters omit the unused call, and the unused repair-root slot is None. The caller sums repair-root presence into a diagnostic count. No caller uses that count or root identity to choose a transition or fallback. Pair Option and selected coordinates are unchanged under the preconditions below. Fallback runs precisely when the selected pair is None; its signed reconstruction and labels remain unchanged. A skipped None never caused fallback in the eager version. The source has no RNG or other mutable data inside repaired arithmetic except thread-local operation counters.

Equivalence domain: all operands and action matrices canonical modulo p, prime 2<p<=2^32; nonzero t entries; sufficient u64 counter headroom for the eager evaluation; valid indexing; no external interruption. These ensure products fit u64, x+y and x+p-y fit, and inv's assertion holds. Without headroom, eager execution can panic when the candidate does not; under deadlines, removal of work may also change termination. Thus this is state/label/fallback equivalence on normal execution, not identical diagnostic counts, panic behavior on arbitrary inputs, or deadline outcomes. The proposal has not been compiled or executed and must receive a new method identity before any measurement.

## Symbolic full accounting
Charge initialization signed construction plus two projections and fixed model setup. Each ordinary step pays branch selection, selected double, mixed differential operation, repairs actually attempted, labels, conversion/filter, and bookkeeping. Each split candidate additionally pays opposite-neighbour construction, conversions, signed recovery or exceptional lifting, split checks and graph processing. Each restart pays two signed scalar multiplications, addition, neighbour addition, validity checks where executed, and two projections. Each failed differential step pays its failed attempts PLUS the signed fallback reconstruction, checks and projections. Final recovery and independent verification remain charged. Precomputing inverse constants is a separate possible change; it is not included in this proposal.

## Next concrete checks
Independent source review of conditional equivalence; identical-output paired transition checks including forced repair/None, nonselected repair, and fallback; explicit counter-delta accounting; runtime qualification and prospective protocol before any fresh 1,260-target cohort. Preserve all existing N19 data. Coordinate arithmetic and exceptional recovery still require their separate fidelity proof. No new cost ratio is reported.
'''
(D/'REPORT.md').write_text(report)
record={'schema':'research-construction-record/v1','lead':'kummer-operation-set-closure','status':'derived_unreviewed','runtime_executed':False,'new_cohort_targets':0,'claims':[{'id':'adjacent-pair-closure','kind':'conditional_algebraic_derivation','transition':'R -> 2R+cG','labels':['2a+c mod r','2b mod r']},{'id':'branch-directed-pruning','kind':'source_derived_proposal','eager_repaired_calls':3,'candidate_repaired_calls':2,'measured_cost_ratio':None,'identical_observations':False,'preserved':'selected pair, labels and fallback trigger under stated preconditions'}],'sources':[{'path':str(p),'sha256':hashlib.sha256(p.read_bytes()).hexdigest()} for p in [s,w,H/'n19-labelled-fallback/VALIDATION.md']],'literature':['https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/recovery.pdf']}
(D/'result.json').write_text(json.dumps(record,indent=2)+'\n')
svg='''<svg xmlns="http://www.w3.org/2000/svg" width="900" height="270" viewBox="0 0 900 270"><rect width="900" height="270" fill="white"/><g font-family="sans-serif" text-anchor="middle" font-size="18"><text x="450" y="28">Conditional adjacent-pair closure (derived, not measured)</text><rect x="30" y="70" width="250" height="80" fill="#e4edf8" stroke="#345"/><text x="155" y="102">[R], [R+G]</text><text x="155" y="130">labels (a,b)</text><path d="M280 110 H400" stroke="#345" stroke-width="2"/><text x="340" y="91">c=0 or 1</text><rect x="400" y="70" width="470" height="80" fill="#e4f1e8" stroke="#345"/><text x="635" y="102">[2R+cG], [2R+(c+1)G]</text><text x="635" y="130">labels (2a+c,2b) mod r</text><text x="450" y="191">Selected double + mixed differential addition</text><text x="450" y="222">Signed anchor G + adjacent pair → recovery when applicable</text><text x="450" y="251">Repairs, fallback, restarts and recovery remain charged</text></g></svg>'''
(D/'closure.svg').write_text(svg)
# Complete readable report source; Markdown converted to Typst text blocks.
typ='#set page(paper: "a4", margin: 18mm)\n#set text(size: 10pt)\n#image("closure.svg", width: 100%)\n'
for para in report.split('\n\n'):
 if para.startswith('# '): typ+='\n= '+para[2:]+'\n'
 elif para.startswith('## '):typ+='\n== '+para[3:]+'\n'
 else:typ+='\n#text('+json.dumps(para)+')\n\n'
(D/'report.typ').write_text(typ)
