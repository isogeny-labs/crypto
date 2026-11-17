from pathlib import Path
import hashlib,json,subprocess,textwrap,html
p=Path(__file__).parent;h=lambda x:hashlib.sha256(x.read_bytes()).hexdigest()
paths=[Path('/home/ec2-user/crypto-pr2304-persist/research/projective_law_followup_20261022/src/bin/projective_law_gate.rs'),Path('/home/ec2-user/crypto-pr2304-persist/src/cryptanalysis/three_leads.rs'),Path('/var/tmp/pencil-mixed-generic-20261031-v1/MixedGeneric.lean'),Path('/var/tmp/pencil-mixed-generic-20261031-v1/manifest.json'),Path('/var/tmp/pencil-mixed-generic-independent-review-20261031/manifest.json'),Path('/var/tmp/pencil-mixed-cancellation-20261031-v1/WordMixed.lean'),Path('/var/tmp/pencil-mixed-cancellation-20261031-v1/MixedCancellation.lean'),Path('/var/tmp/pencil-add11-word-composition-20261031-v1/WordAdd11.lean'),Path('/var/tmp/pencil-add11-word-validity-20261031-v1/WordAdd11Validity.lean')]
(p/'sources.json').write_text(json.dumps({str(x):h(x)for x in paths},indent=2)+'\n')
text='''Generic mixed-degree addition: literal word correspondence

Five theorems compiled for the retained generic degree-one plus degree-two branch. The proof preserves the source multiplication nesting, including the different outer-dispatch and generic-body expressions for k. No exponentiation loop occurs in this branch: every square is an explicit multiplication.

1. generic_word_decode: all six output words and k are canonical residues; decoded values equal MixedGeneric.output and genericK.
2. generic_guard_decode: the actual word k is zero exactly when the decoded field k is zero.
3. generic_dispatch_decode: mapping the word Option result through decoding gives the field Option dispatch, covering both None and Some.
4. generic_outer_guard: the outer complete-dispatch k word equals the generic-body k word, despite their different multiplication associations.
5. generic_word_preserves: with valid normalized inputs, four nonzero weights and nonzero k, dispatch returns a canonical degree-two payload with nonzero output weights and normalized Mumford divisibility.

Value assumptions: UInt64 modulus p, prime with 5 <= p < 2^31; all ten coordinate words and curve[3] are canonical. The literal output w=1 is included in canonicality and decoding. The validity theorem takes the remaining three curve coefficients directly as field values; they are not read by this machine branch. Its curve is the monic quintic with no X^4 term. Input validity and nonzero weights are premises, not output-validity assumptions.

Source binding: projective_law_gate.rs lines 232–292 for generic addition and 828–851 for outer dispatch; Field arithmetic is the retained n=1, tables=None path with sufficient normal counter headroom. FieldExpr establishes bounded arithmetic value correspondence. These theorems do not establish compiler behavior, Rust execution traces, operation/counter equivalence, divisor-class addition, or a whole collector walk. The pure genericWords definition also defines unused payload values when k=0; Option correspondence does not assert that those computations run in the early-return Rust branch.

Evidence: main attempt3, Controls controls4 and separate Readback readback1 all exited zero under the pinned Lean toolchain. Five axiom readbacks contain only propext, Classical.choice and Quot.sound. Attempt1 failed on literal-one normalization; controls1–3 retain their canonical-predicate and finite-field normalization failures. All failures remain in this packet.

Controls: a nonunit-weight F7 fixture invokes the full validity theorem, using a=(0,1,2,2), b=(0,4,0,0,3,2), and f=X^5+X^2-X+1. Separate controls check canonical payload, nonzero generic k, k=0 returning None, rejected canonicality at y=p, and zero input z yielding k=0. These are polynomial/value checks; no nonsingularity or cryptographic fixture claim is made.

The field dependency MixedGeneric was independently reviewed in the separate mixed-generic-independent-review packet. Its source hash and review manifest are bound in sources.json. This newly composed word packet still requires its own independent literal-source review. No scientific execution or performance measurement occurred; cost ratio is null.

Reproduction: copy this packet's source and checker into a new private directory, preserving dependency paths from check.py; invoke check.py WordGeneric UNIQUE_TAG, then Controls and Readback with fresh tags. Each invocation has a 120-second cap and writes its own source snapshot, native stdout/stderr and receipt. Source dependency hashes must match sources.json. Existing dependency oleans are reused; this is not a claim that all transitive dependencies were rebuilt here.
'''
(p/'REPORT.md').write_text(text)
s={'schema':'research.proof-component/v1','scope':'literal generic mixed add12 UInt64 payload and guard correspondence','theorems':['generic_word_decode','generic_guard_decode','generic_dispatch_decode','generic_outer_guard','generic_word_preserves'],'native_receipts':{'module':'attempt3/receipt.json','controls':'controls4/receipt.json','readback':'readback1/receipt.json'},'native_exit_codes':[0,0,0],'prime_bound':'5 <= p < 2^31','canonical_inputs':'ten coordinate words and curve[3]','word_packet_independently_reviewed':False,'field_dependency_review':'/var/tmp/pencil-mixed-generic-independent-review-20261031','value_scope_only':True,'counter_correspondence':False,'compiled_execution_correspondence':False,'class_sum_proof':False,'scientific_execution':False,'cost_ratio':None,'failures_retained':['attempt1','controls1','controls2','controls3']}
(p/'status.json').write_text(json.dumps(s,indent=2)+'\n')
lines=[]
for para in text.splitlines():lines.extend(textwrap.wrap(para,106)or[''])
height=90+18*len(lines);svg=f'<svg xmlns="http://www.w3.org/2000/svg" width="980" height="{height}"><rect width="100%" height="100%" fill="white"/><g font-family="DejaVu Sans" font-size="14" fill="#193047">'
for i,l in enumerate(lines):svg+=f'<text x="35" y="{35+i*18}">{html.escape(l)}</text>'
svg+='</g></svg>';(p/'overview.svg').write_text(svg);subprocess.run(['rsvg-convert','-f','pdf','-o',str(p/'report.pdf'),str(p/'overview.svg')],check=True)
