from pathlib import Path
import json,hashlib,struct
D=Path(__file__).parent;P=Path('/var/tmp/pencil-musl-build-final-3c592af-20261031-v1');R=Path('/var/tmp/pencil-final-source-refresh-execution-20261031-v1');S=Path('/var/tmp/pencil-final-sparse-proposal-20261031-v1');sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest();e=json.loads((D/'evidence.json').read_text());r=json.loads((P/'receipt.json').read_text());m=json.loads((R/'new-materialized.json').read_text())
assert m['selected_sha256']==sha(S/'selected.json');assert not any(m[k] for k in ['errors','missing','unexpected']);assert len(m['files'])==5909
for n in ['head-before','head-after']:assert (P/(n+'.stdout')).read_text().strip()==r['source_commit_requested']
assert (P/'tree.stdout').read_text().strip()==r['source_tree'];assert all(c['exit_code']==0 for c in r['checks']);assert r['status']=='built_static_candidate' and r['build_exit_code']==0
for n in ['untracked-before','untracked-after','tracked-clean-before','tracked-clean-after']:assert not (P/(n+'.stdout')).read_bytes()
b=(P/'ecbench').read_bytes();assert b[:6]==b'\x7fELF\x02\x01';typ,machine=struct.unpack_from('<HH',b,16);phoff=struct.unpack_from('<Q',b,32)[0];ents,num=struct.unpack_from('<HH',b,54);headers=[struct.unpack_from('<IIQQQQQQ',b,phoff+i*ents) for i in range(num)];tags=[]
for h in headers:
 if h[0]==2:
  for i in range(h[2],h[2]+h[5],16):
   tag,val=struct.unpack_from('<qQ',b,i);tags.append(tag)
   if tag==0:break
assert machine==62 and typ==3 and 3 not in [h[0] for h in headers] and 1 not in tags
stderr=(P/'build.stderr').read_text();fresh=[x.strip() for x in stderr.splitlines() if any('Fresh '+n+' ' in x for n in ['ring','libsqlite3-sys','blake3'])];assert len(fresh)==3
extra={'schema':'software.provenance-review/v1','verdict':'validated_scoped','source_commit':r['source_commit_requested'],'binary_sha256':r['binary_sha256'],'map_sha256':r['link_map_sha256'],'source_materialization_files':len(m['files']),'source_materialization_bytes':sum(x['bytes'] for x in m['files']),'ELF_independent_bytes':{'machine':machine,'type':typ,'interpreter':False,'DT_NEEDED':False,'PT_DYNAMIC':2 in [h[0] for h in headers]},'reused_native_dependencies':fresh,'execution_during_review':False,'image_qualified':False,'bundled_musl_upstream_revision':'unestablished','prior_involvement':'Reviewer produced private compiler prerequisite and sparse selector; did not execute final build.'};(D/'review.json').write_text(json.dumps(extra,indent=2)+'\n')
paths=list(P.glob('*'))+list(R.glob('*.json'))+[S/'manifest.json',S/'selected.json',Path('/var/tmp/pencil-musl-native-provenance-review-20261031-v1/manifest.json')];(D/'sources.json').write_text(json.dumps({str(p):sha(p) for p in paths if p.is_file()},indent=2)+'\n')
text=f'''# Final corrected-source musl build provenance review

Validated at retained native build and link-provenance scope. The completed build has exit 0 and binary SHA256 {r['binary_sha256']}. Source commit {r['source_commit_requested']}, tree {r['source_tree']}. No binary, CLI, container, compiler or scientific workload was executed during this review.

The corrected-source refresh receipt reports refreshed_verified_no_build. Its selected-file digest matches the reviewed immutable sparse selection. Materialization records 5,909 files and 289,945,217 bytes with no missing, unexpected or erroneous entries. Final build receipts bind the same commit before and after, detached source and clean tracked/untracked state; all recorded checks exit zero. These are retained materialization/source-check receipts, not a second full live filesystem traversal or a hermetic compiler proof.

Cargo used Rust 1.99.1, locked/offline release, x86_64-unknown-linux-musl, no default features, x86-64 CPU baseline and jobs2. Build metadata sets the corrected commit and dirty0. Cargo rebuilt crypto after the commit environment changed; ring, libsqlite3-sys and blake3 were Fresh reused dependencies. Thus the new log contains no fresh C compiler declarations; their prior build lineage remains separately retained.

Independent binary parsing confirms ELF64 little-endian x86-64 static PIE (ET_DYN), no PT_INTERP and no DT_NEEDED; a dynamic section is present. The binary and link-map hashes match the native receipt. The final printed linker arguments and map both bind five Rust 1.99.1 self-contained CRT objects. The map loads its self-contained libc.a with SHA256 e699c64b0c6b427d89ad5bf767bf78beb30c19930c5dedff635951636e8f9f70; 362 distinct archive member names were observed. The library is selected by linker search rather than an explicit full libc pathname in final arguments. All six CRT/libc asset hashes match the recorded candidates.

The private musl prefix has zero references in the final link map. Its compiler/specs/libc hashes still match the build receipt, but its hello-program libc version does not establish this Rust-linked executable's libc version. The bundled libc upstream musl revision is unestablished. Printed final driver cc currently resolves to /usr/bin/gcc with the recorded GCC digest; this is current resolution consistency, not historical process-exec tracing.

No discrepancy blocking the next assembly binding was found. The next assembly must bind this exact final binary and commit, replacing the historical b985 candidate. Image assembly and runtime qualification remain separate steps. Review independence disclosure: this reviewer produced the private compiler prerequisite and sparse selector proposal; root executed the final source refresh and build. The review uses fresh binary/map hashing and independent ELF parsing, while retaining that prior involvement.
''';(D/'REVIEW.md').write_text(text)
t='#set page(paper:"a4",margin:16mm)\n#set text(size:10pt)\n';t+='\n'.join('#text('+json.dumps(x.removeprefix('# '),ensure_ascii=False)+')\n#parbreak()' for x in text.split('\n\n'));(D/'report.typ').write_text(t)
(D/'diagram.svg').write_text('<svg xmlns="http://www.w3.org/2000/svg" width="920" height="280"><rect width="920" height="280" fill="#f1f5fa"/><g font-family="sans-serif" fill="#24394f"><text x="25" y="40" font-size="23">Final corrected-source build: observed provenance</text><text x="25" y="90" font-size="18">3c592af → 5,909 selected files → Rust 1.99.1 musl target</text><text x="25" y="135" font-size="18">Rust self-contained CRT + libc → static PIE d0e2d142…</text><text x="25" y="180" font-size="18">No interpreter / shared-library requirements; exact map bound</text><text x="25" y="230" font-size="17">Native dependencies reused; image qualification is a separate step.</text></g></svg>')
print(json.dumps(extra,indent=2))
