from pathlib import Path
import json,hashlib,datetime
D=Path(__file__).parent
h=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
paths=[Path('/home/ec2-user/crypto-pr2304-persist/research/projective_law_followup_20261022/src/bin/projective_law_gate.rs'),Path('/home/ec2-user/crypto-pr2304-persist/src/cryptanalysis/three_leads.rs'),Path('/var/tmp/pencil-add11-weighted-invariant-20261031-v1/WeightedAdd11.lean'),Path('/var/tmp/pencil-word-error-exclusion-review-20261031-v1/replay/CrossScaling.lean')]
(D/'sources.json').write_text(json.dumps({'files':{str(p):h(p) for p in paths},'rust_lines':{'add11':'511-532','Field_pow':'290-301'},'mapping':'Field.n=1, tables=None, odd-prime arithmetic, ordinary execution with counter headroom. Source model only; no compiler theorem.'},indent=2)+'\n')
r={'schema':'research.formal-component-result/v1','created_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'scope':'UInt64 exponent-five loop and literal weighted add11 distinct-branch payload correspondence','source_sha256':h(D/'WordAdd11.lean'),'hypotheses':['prime 5<=p<2^31','eight canonical UInt64 input coordinates (<p)','for distinct guard: two nonzero decoded z and unequal normalized x'],'conclusions':['exact exponent5 unroll including1*a and two squares','all six literal word payload outputs canonical','all six decoded coordinates equal WeightedAdd11.output','actual inverse-first/doubling-second/assert-equal guard reaches distinct under normalized distinct input x'],'external_source_conditions':['Field.n=1','tables=None','ordinary execution/counter headroom'],'axioms':['propext','Classical.choice','Quot.sound'],'independent_review':'pending','not_claimed':['counter or compiler semantics','inverse and doubling payload correspondence','Mumford validity composition for resulting word representation','divisor-class addition','whole-walk closure'],'cost_ratio':None,'scientific_execution':False,'receipts':{p.parent.name:json.loads(p.read_text())['exit_code'] for p in D.glob('*/receipt.json')}}
(D/'status.json').write_text(json.dumps(r,indent=2)+'\n')
report='''# Weighted add11 machine expression correspondence

The literal UInt64 weighted add11 distinct-branch payload now has a canonical-output and field-decoding theorem. The exponent-five helper follows the actual Field::pow loop: e=5,2,1,0; accumulator products include 1*a and the final multiplication by a^4; two conditional squares occur. The exact unroll is proved by reflexivity.

Under prime 5<=p<2^31 and eight canonical input words, a finite expression induction composes modular multiplication, addition, negation, subtraction and exponent-five. All six returned q,r,s,t,z,w words are canonical and decode to the existing WeightedAdd11.output field formula with the source parenthesization. Products stay below2^62 and add/sub intermediates below2^32; the bounded primitive lemmas establish value correspondence rather than assuming coefficient identities.

The word guard retains inverse first, doubling second, then equal-cross assertion failure. Nonzero decoded input z and distinct normalized x imply unequal actual cross words, hence the distinct branch. The distinct payload theorem is algebraic and also decodes the raw body outside that guard; source execution reaches that body only after guard dispatch.

Controls check exponent-five at0,3,6 mod7; nonunit-weight payload and distinct dispatch; zero-y inverse precedence; doubling; invalid same-x assertion failure; different raw x values with equal normalized x; noncanonical input and an overflowing UInt64 multiplication boundary. Controls and separate axiom readback exit0. Final axioms are propext, Classical.choice and Quot.sound. Failed attempts are retained: attempt2 omitted definitional word unfolding in four induction cases; attempt4 omitted prime instance on a field-valued cross lemma; controls1 needed explicit canonical-definition unfolding. Final proof has no sorryAx.

Source mapping requires Field.n=1, tables=None, odd-prime path and normal execution/counter headroom. The bounded loop is a source model, not a compiler equivalence theorem. Expression trees establish values, not operation counts or sharing. Counter behavior, inverse/doubling payloads, composition to normalized word Mumford validity, divisor-class addition and whole-walk closure are separate obligations. The field theorem packet remains immutable. No scientific trials or cost measurements were taken; cost_ratio is null. Independent review is pending.

Exact source hashes and mappings: sources.json. Native commands/exits and source snapshots: attempt*/ and controls*/readback1/. Frozen intent: PLAN.md. Prior field proof is WeightedAdd11.lean in the separately hashed producer packet; prior source-bound literature reviews and primitive no-overflow lemmas supply context, with no novelty claim.
'''
(D/'REPORT.md').write_text(report)
(D/'overview.svg').write_text('''<svg xmlns="http://www.w3.org/2000/svg" width="1100" height="340"><rect width="1100" height="340" fill="white"/><g font-family="sans-serif" fill="#17334d"><text x="25" y="38" font-size="24">UInt64 weighted add11: expression and distinct guard</text><text x="25" y="88" font-size="18">Canonical inputs + prime 5 ≤ p &lt; 2³¹ → bounded modular primitives</text><text x="25" y="138" font-size="18">Exact pow5 loop → expression induction → six canonical field-decoded outputs</text><text x="25" y="188" font-size="18">Nonzero z + unequal normalized x → unequal cross words → distinct guard</text><text x="25" y="238" font-size="17">Controls + separate axiom readback pass; failed attempts retained</text><text x="25" y="288" font-size="16">Next: word-validity composition. Counters/compiler/class-sum outside this claim.</text></g></svg>''')
(D/'report.tex').write_text(r'''\documentclass[11pt]{article}
\usepackage[margin=.8in]{geometry}\usepackage[T1]{fontenc}
\begin{document}\section*{UInt64 weighted add11 correspondence}
31 October 2026 UTC. Producer packet; independent review pending.

The literal distinct-branch payload now has canonical-output and field-decoding proofs. Under prime $5\le p<2^{31}$ and eight canonical input words, all six returned $q,r,s,t,z,w$ words decode to the weighted field formula with source parenthesization.

\textbf{Power loop.} The actual exponent-five path is $5\to2\to1\to0$: two accumulator products (including $1a$), two conditional squares, no final unused square. Its exact UInt64 unroll is proved by reflexivity. The expression induction composes modular multiply, add, negate, subtract and this power helper. Primitive products are below $2^{62}$ and additive intermediates below $2^{32}$.

\textbf{Branch dispatch.} The model preserves inverse first, doubling second, then an equal-cross assertion failure. Nonzero decoded input z and distinct normalized x imply unequal actual cross words, hence the distinct branch. The payload theorem decodes the raw body separately from its reachability condition.

\textbf{Validation.} Controls cover zero/nonzero powers mod7, nonunit weighted inputs, inverse precedence at zero y, doubling, invalid same-x assertion failure, unequal raw x encoding equal normalized x, a noncanonical input and an overflowing multiplication boundary. Final source, controls and separate axiom readback exit0. Only propext, Classical.choice and Quot.sound occur. Failed simplification, missing prime-instance and control-definition attempts remain with source snapshots and receipts.

\textbf{Source scope.} Field.n=1, tables=None, odd-prime arithmetic and ordinary execution/counter headroom are external mapping conditions. This is a source model, not compiler equivalence. Expression trees prove values, not costs or evaluation sharing. Inverse/doubling payloads, normalized word-validity composition, counters, divisor-class addition and whole-walk closure remain separate obligations.

No scientific trials or measurements; cost ratio is null. Frozen plan, source hashes, proof, controls, readback and failed attempts are retained. The prior field packet is unchanged.
\end{document}
''')
