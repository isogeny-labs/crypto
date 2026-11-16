from pathlib import Path
import typst,fitz
D=Path(__file__).resolve().parent
labels=['Unchanged numerical output, reference and ledger checks', 'Existing exact trace: setup, columns, packed final value', 'Typed canonical binary chunks; integer magnitudes, no decimal conversion', 'Stream chunks into SHA-256; retain version, digest and encoded bytes', 'Full cohort, seeds, controls and A/A schedule remain required']
svg='<svg xmlns="http://www.w3.org/2000/svg" width="800" height="300"><rect width="800" height="300" fill="white"/>'
for i,label in enumerate(labels):
 y=8+58*i;svg+=f'<rect x="15" y="{y}" width="770" height="40" rx="5" fill="#edf3f6" stroke="#286e89"/><text x="400" y="{y+25}" text-anchor="middle" font-family="sans-serif" font-size="15">{label}</text>'
 if i<4:svg+=f'<path d="M400 {y+40} v18 m-4 -5 l4 5 l4 -5" fill="none" stroke="#286e89"/>'
svg+='</svg>';(D/'pipeline.svg').write_text(svg)
t='''#set page(paper:"a4",margin:18mm)
#set text(size:10.5pt)
= Exact streaming trace fingerprints
2026-11-01 · Tested software component; no kernel trial or timing benchmark

*Inspected path.* The frozen checker serializes each complete packed-integer
trace to sorted JSON solely for a SHA-256 fingerprint. The string is then
discarded. This is confirmed from source; its contribution to the original
cap has not been measured. The original invocation remains terminal and
unchanged: candidate complete, A/A partial, native exits 124.

#image("pipeline.svg",width:100%)

*Implemented alternative.* trace_codec.py emits a domain/version header,
distinct type tags, canonical lengths and binary integer magnitudes. Strict
UTF-8 strings, lists and sorted string-key dictionaries are supported.
None, booleans and integers remain distinct. A separately implemented decoder
rejects malformed or noncanonical encodings, truncation and trailing bytes.
The fingerprint includes schema, SHA-256 and encoded byte count. The full
trace object remains in memory; no whole JSON trace string is constructed.

*Correctness argument.* Type tags and self-delimiting canonical payloads
identify each value and container boundary uniquely. Sorted distinct keys
fix dictionary order. This establishes encoding injectivity over supported
values, not cryptographic collision freedom. SHA-256 retains its usual
fingerprint assumption. The new fingerprint is not the old JSON hash.

*Software checks completed.* Eight deterministic unittest cases pass, covering
100,001-bit integers, signs/zero, Unicode, empty/nested containers, handwritten
golden bytes, canonical ordering, type/boundary distinctions, every truncation
of a structured fixture, malformed encodings and depth guards. Streamed hashes
match hashes of materialized encodings. No numerical kernel was imported.
The reported test duration is not used as performance evidence.

*Integration boundary.* integration.patch is unapplied. Whole proposed source
passes syntax checks. AST checks establish unchanged main schedule, stage
enumeration and verify statements before its final returned record. It retains
the declared 1,698 rows, 34,248 products, both arms, all inputs, references and
controls. Only imports and the trace-fingerprint representation change.
Original/proposed source hashes are in integration.json.

*Next.* Add version/hash/byte-field validation to a fresh analyzer and freeze
a complete separate instrument contract, locks, seed table and finite budget
before any execution. Preserve original capped records and every missing A/A
case. No kernel arithmetic or measured speed improvement is claimed by this
codec change. DESIGN.md records the exact encoding and rejection rules.
'''
(D/'report.typ').write_text(t);typst.compile(str(D/'report.typ'),output=str(D/'report.pdf'))
doc=fitz.open(D/'report.pdf')
for i,p in enumerate(doc):p.get_pixmap(matrix=fitz.Matrix(1.3,1.3)).save(str(D/f'page-{i+1}.png'))
print('pages',len(doc))
