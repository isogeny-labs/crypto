from pathlib import Path
import json,hashlib
p=Path(__file__).parent
src=Path('/home/ec2-user/crypto-pencil-signed-graph-20261029/src/cryptanalysis/three_leads.rs');lines=src.read_text().splitlines()
spans=[('Divisor representation and Jacobian validity',938,984),('Scholten constructor',1094,1101),('tau output assertions',1174,1208)]
rows=[]
for name,a,b in spans:
 out='\n'.join(lines[a-1:b])+'\n';fn=f'source-{a}-{b}.rs.txt';(p/fn).write_text(out);rows.append({'name':name,'first_line':a,'last_line':b,'excerpt':fn,'sha256':hashlib.sha256(out.encode()).hexdigest()})
(p/'caller-source-map.json').write_text(json.dumps({'source':str(src),'source_sha256':hashlib.sha256(src.read_bytes()).hexdigest(),'spans':rows},indent=2)+'\n')
body='''Normalization validity from sliced input guards

T-1189; 2026-10-31. Producer proof and source trace; independent source-fidelity review pending.

A coefficient prefix through degree d whose coefficient at d is one has exact natural degree d and is monic. A prefix of length d has polynomial degree strictly less than d, including the empty prefix at d=0. Combined with d<=2 and residual divisibility, these facts establish all four Mumford validity conditions. The UInt64 composition derives the stored-output validity and preserved degree directly from the top-word-one guard; separate input degree metadata and a bundled input-validity premise are no longer required by that theorem.

Six main declarations and eight controls are exposed for separate axiom readback. Controls cover degrees zero, one and two, empty ordinate, missing top coefficient, inconsistent degree metadata, a nonmonic top coefficient, and the ordinate prefix bound. Canonical coefficient/parameter words, prime 1<p<2^31, nonzero scale and residual divisibility remain explicit assumptions of the stored-word theorem.

Source trace: Jacobian.valid checks degree<=2 and u[degree]==1, then checks an empty remainder of f-v^2 modulo the sliced u. Divisor.up includes coefficients through degree; Divisor.vp takes only coefficients below degree. This supplies the shape of the proved guards. The logical connection between the native remainder algorithm and polynomial divisibility is still unproved here. Jacobian.valid does not itself check canonical coefficient words or unused array tails.

For non-infinite tau outputs, the source asserts every stored u/v word is below p and checks Jacobian.valid. The infinity branch returns the fixed identity. The model gate checks source validity for its sampled d inputs, but the add-result passed to map_divisor has no immediate validity assertion at that call. Source-level construction/invariant proofs remain necessary to cover that path. Scholten.new asserts p>13, p mod4=3 and constructor primality; the inspected constructor does not assert p<2^31. Field.validate contains that bound, but this model-gate caller does not invoke it explicitly. The proof retains the bound; this trace does not certify the fixture data or all callers.

Source excerpts are exact-line/hash-bound to three_leads.rs, alongside the corrected named model-gate extraction. Prior sourcePoly_coefficient and stored_valid supply the composition. Pinned Mathlib degree lemmas provide coefficient/degree equivalences. No new compiled dependencies, native remainder proof, compiled-Rust equivalence, geometric class theorem or scientific execution is claimed.

All four native checks passed with unchanged input bindings. All 14 named axiom readbacks use only propext, Classical.choice and Quot.sound; results are in status.json. Existing compiled upstream dependencies retain their trust boundary. Any nonfatal deprecation warnings are kept in the receipts.
'''
(p/'REPORT.md').write_text('# '+body)
s=body.replace('_',r'\_').replace('^',r'\textasciicircum{}').replace('<=',r'$\leq$').replace('<',r'$<$')
(p/'report.tex').write_text(r'\documentclass[10pt]{article}\usepackage[margin=.7in]{geometry}\usepackage[T1]{fontenc}\setlength{\parindent}{0pt}\setlength{\parskip}{8pt}\sloppy\begin{document}'+s+r'\end{document}')
(p/'flow.svg').write_text('''<svg xmlns="http://www.w3.org/2000/svg" width="1100" height="350"><rect width="1100" height="350" fill="white"/><g font-family="sans-serif" fill="#17364a" text-anchor="middle"><text x="550" y="30" font-size="22">From sliced storage guards to normalization validity · T-1189</text><g stroke="#17364a" fill="#edf5f7"><rect x="30" y="70" width="300" height="100"/><rect x="400" y="70" width="300" height="100"/><rect x="770" y="70" width="300" height="100"/></g><text x="180" y="100">u[d]=1; u prefix length d+1</text><text x="180" y="130">v prefix length d; d ≤ 2</text><text x="550" y="100">Exact degree + monicity</text><text x="550" y="130">Ordinate degree bound</text><text x="920" y="100">Input and stored validity</text><text x="920" y="130">Residual divisibility supplied</text><path d="M330 120H400M700 120H770" stroke="#17364a"/><text x="550" y="225">Word composition retains canonical words, prime bound and nonzero scale</text><text x="550" y="270">Next native bridge: empty remainder ↔ polynomial divisibility</text><text x="550" y="320">Caller/fixture bounds and add-result invariants remain explicit obligations</text></g></svg>''')
