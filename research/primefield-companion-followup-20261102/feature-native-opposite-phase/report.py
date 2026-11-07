from pathlib import Path
import json,hashlib,re
p=Path(__file__).resolve().parent
lines=(p/'validation02/check.stdout').read_text().splitlines()[:3]
receipt=json.loads((p/'validation02/receipt.json').read_text())
assert receipt['compile_exit']==receipt['check_exit']==0
rows=[dict(re.findall(r'(\w+)=(\d+)',s)) for s in lines]
(p/'RESULTS.md').write_text('''# Native correctness results — T-1878

Producer observations; independent signed verdict pending. [pcc:unverified]

validation01 passed on its first compiler/native attempt. The subsequent-update counter was wired to the old adapter counter; it is not evidence of zero subsequent work. Its three changed source files are retained. validation02 corrects instrumentation and adds 28 rejected-domain controls and 10 public fallback comparisons. Neither attempt failed compilation or native checks. Both include inherited tests.

## Separate deterministic panels

Seven (r,m) configurations: (41,5), (73,3), (97,3), (193,3), (241,3), (241,5), (281,5). For r <= 73, all c+/c- pairs and all four phases; for larger fields, diagonal pairs and actual scalar-polynomial values. Each support equals a direct gcd reference, is squarefree, and has degree <= n/2 for unequal observations. Complete candidate states compare initialization after two radii and updates at 2 and 3, including zero handling.

```
'''+ '\n'.join(lines) +'''
```

Both public panels recover all 281 targets. Cap 64 costs 241 additional calls (3122 versus 2881); cap 16 saves 2 calls (3376 versus 3378). These are exact logical source-call totals for these fixtures, not timing or total arithmetic measurements. The baseline uses the same opposite-radius schedule. Early endpoint exits are retained. Fallback on a nonopposite schedule agrees on result, source calls and route for k in {0,1,17,140,280} at both caps.

The first gcd domain degree is n/2 by construction. Initial folding remains degree-dependent, and lifting may reach n. No claim about peak memory or elapsed time is supported by these checks. Full source grids, initial observations, source calls and final point verification remain charged. Original 4563-case cohort is unchanged.

Replay: README.md and validate-recovery.py. Compiler: '''+receipt['compiler']+'''. validation02 records '''+str(len(receipt['input_hashes']))+''' native dependency hashes. No cloud jobs or timed workloads were launched. PCC-20261107-6848bb pins the derivation; its independent signed verdict is pending.
''')
# Bar chart of exact logical call totals, no sampling uncertainty.
svg='<svg xmlns="http://www.w3.org/2000/svg" width="960" height="235"><rect width="960" height="235" fill="white"/><g font-family="sans-serif" font-size="16" fill="#17324d"><text x="15" y="25">Source calls · 281 targets per cap · validation02 · exact totals</text>'
for j,(label,val,color) in enumerate([('cap 64: direct',2881,'#899aaf'),('cap 64: paired',3122,'#247ba0'),('cap 16: direct',3378,'#899aaf'),('cap 16: paired',3376,'#247ba0')]):
 y=43+43*j;w=val/5.5;svg+=f'<text x="15" y="{y+21}">{label}</text><rect x="190" y="{y}" width="{w}" height="28" fill="{color}"/><text x="{200+w}" y="{y+21}">{val}</text>'
svg+='</g></svg>'; (p/'calls.svg').write_text(svg)
(p/'report.typ').write_text('''#set page(paper: "a4", margin: 15mm)
#set text(size: 10pt)
= Paired-radius parity supports
T-1878 · Prime-field compressed recovery · 2026-11-07

The paired initializer separates the even and odd parts of the folded C0 polynomial. Its first gcd uses Y = V² and phase degree n/2; it then restores the allowed V orientations exactly.

#image("flow.svg", width: 100%)

*Checked construction.* Seven scalar-field/order configurations; 36,464 support equalities, including 4,984 equal-observation and 31,480 unequal-observation controls. There are 35,657 empty supports. All 3,501 candidate states agree with sequential direct updates. Validation02 also checks 28 rejected inputs and 10 public fallback comparisons.

#image("calls.svg", width: 100%)

*Public recovery.* E over F₂₅₇: y² = x³ + x + 7; P = (1,3); r = 281; m = 5. Radii 1, 280, 2, 3; extra offsets 0 through 16; call cap 128. Both adapters recover all targets at both degree caps. These totals include endpoint exits and source calls; they are not elapsed-time measurements. The second orbit costs 241 additional calls at cap 64 and saves 2 at cap 16.

*Costs and limits.* Four n-entry folds and n+1 coefficient deliveries remain. Norm filtering and orientation restoration add polynomial work; temporary lifting can reach degree n. Equal observations may retain both signs. Source setup, order checks, allocation/copy work and final verification stay charged.

*Evidence.* validation01 passed but omitted new-adapter later-update counts; changed sources are retained. validation02 corrects the counter and passes the expanded checks. 91 native dependency pins; no compiler/native failure or cloud/timing run. PCC-20261107-6848bb is an unsigned producer attestation; independent signed verdict pending.

*Sources and next.* Exact direct-phase predecessor and NIST DLMF 26.3 (dlmf.nist.gov/26.3); KB unavailable. Next use paired initialization only after a first-radius failure, to preserve early recoveries. DERIVATION.md and RESULTS.md give proof, replay and scope.
''')
