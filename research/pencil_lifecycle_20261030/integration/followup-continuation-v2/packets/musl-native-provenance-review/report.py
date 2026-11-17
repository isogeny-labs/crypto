from pathlib import Path
import json,hashlib,datetime,subprocess
D=Path(__file__).parent
C=Path('/var/tmp/pencil-b985-static-cli-check-20261031')
h=lambda b:hashlib.sha256(b).hexdigest()
r=json.loads((C/'positive.receipt.json').read_text()); request=Path(r['command'][r['command'].index('--request')+1]); data=request.read_bytes(); j=json.loads(data)
refs=[x[k] for x in j['routes'] for k in ('plan','spec')]+[x['file'] for x in j['artifacts']]
rows=[]
for x in refs:
 b=Path(x['path']).read_bytes(); rows.append(dict(path=x['path'],bytes=len(b),sha256=h(b),matches=h(b)==x['sha256']))
cli={'receipt_directory':str(C),'files':{p.name:h(p.read_bytes()) for p in C.iterdir() if p.is_file()},'request_sha256':h(data),'request_matches_receipt':h(data)==r['command'][r['command'].index('--request-sha256')+1],'references':rows,'recomputed_loaded_evidence_bytes':len(data)+sum(x['bytes'] for x in rows),'all_reference_hashes_match':all(x['matches'] for x in rows),'positive_exit':r['exit_code'],'negative_exit':json.loads((C/'wrong-hash.receipt.json').read_text())['exit_code'],'observed_report':json.loads((C/'positive.stdout').read_text()),'replayed':False}
(D/'cli-readback.json').write_text(json.dumps(cli,indent=2)+'\n')
assert cli['all_reference_hashes_match'] and cli['request_matches_receipt']
assert cli['recomputed_loaded_evidence_bytes']==5251028
status={'schema':'software.provenance-review/v1','reviewed_at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'status':'validated_scoped','binary_sha256':'16b6eecd16e4cdc3c56bd733090ae80bb9b558f3f8b5dc449d5b8d3344d880d1','source_commit':'b985ddb30b46abbd9a050e5753c7e301f40b07f6','source_predates_lock_correction':True,'observations':['Five startup/termination objects occur explicitly in final linker arguments and map LOAD records.','Rust self-contained libc.a appears in map LOAD and 362 archive-member references; final arguments select libc by library search.','No private musl prefix reference occurs in linker map.','Native dependency logs select private musl-gcc for ring, sqlite3 and blake3.','ELF has no interpreter or NEEDED entry; a dynamic section remains for static PIE.','Existing CLI receipts: positive exit0, wrong-request-hash exit2; 22 referenced files independently hash-match: 5244979bytes plus6049 request bytes equals5251028bytes.'],'binding_limits':['Rust bundled CRT/libc exact file hashes and installed toolchain path are bound; upstream bundled musl source revision/build recipe is not established by these receipts.','Printed final driver is cc; current resolution is /usr/bin/gcc and matches recorded hash, not an independent historical exec trace.','CC selection declarations do not record complete compiler subprocess/header provenance.','Static diagnostic CLI verifies inventory only, with owner authentication and scientific admission false.','Candidate source predates lock correction; no current-head or image qualification inference.'],'rebuild_required_solely_due_to_bundled_libc':False,'scientific_runs':0,'cost_ratio_to_rho':None,'reviewer_executed_candidate':False}
(D/'review.json').write_text(json.dumps(status,indent=2)+'\n')
text='''# Native link provenance review

The retained linker invocation and map identify the actual static runtime: the five CRT objects and libc.a come from Rust 1.99.1's x86_64-unknown-linux-musl self-contained directory. All six current hashes match their recorded candidates. The private musl libc archive has a different hash and no private-prefix map reference; its qualification does not transfer to the bundled archive.

The final invocation names the five CRT objects, uses -static-pie, -nostartfiles and -nodefaultlibs, and selects libc through its library search. The map records the bundled libc LOAD and 362 archive-member references. The executable hash is 16b6eecd16e4cdc3c56bd733090ae80bb9b558f3f8b5dc449d5b8d3344d880d1. Its ELF records show no interpreter or NEEDED dependency; a dynamic section exists for static PIE.

Native dependency output declares the private musl-gcc selection for ring, libsqlite3-sys and blake3. Corresponding native object members appear in retained linked archives. These declarations do not provide a complete subprocess/header trace. The final driver is printed as cc. Its present /usr/bin/cc resolution is /usr/bin/gcc and matches the recorded GCC hash; this is a current consistency observation, not independent historical process tracing.

The exact installed toolchain and bundled asset hashes are known. The upstream source revision and build recipe for those bundled musl assets are not established here. Bundled-versus-private selection alone is not a reason to rebuild when the recipe permits Rust's self-contained target runtime.

The source is commit b985ddb30b46abbd9a050e5753c7e301f40b07f6, tree895383ebc1fdc304045da79f3edc0d1e8a9979d8, predating the lock correction. This review does not relabel it as current-head or image-qualified.

The existing static CLI receipts were read without rerunning the candidate: positive exit0 and wrong-request-hash exit2. All 22 referenced files hash-match and sum to 5,244,979 bytes; adding the 6,049-byte request gives 5,251,028 loaded bytes. The report inventories1,260 workloads,22,680 cohort slots and18 separate gate slots; these are inventory counts. Stage remains static_inventory_only, owner authentication false, software bindings incomplete and scientific execution not admitted.

Evidence: evidence.json binds build receipts, binary, linker map and logs; link-arguments.json records parsed arguments without environment values; cli-readback.json binds the separate CLI receipts and all referenced inputs. No executable, image build or cloud action was run by this review. Cost ratios remain null.
'''
(D/'REVIEW.md').write_text(text)
svg='''<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="330"><rect width="1000" height="330" fill="white"/><g font-family="sans-serif" font-size="18" fill="#172438"><text x="30" y="35">Static candidate: actual link provenance</text><rect x="30" y="60" width="430" height="80" fill="#e9f1ff"/><text x="45" y="90">Private musl-gcc: native dependency selection</text><text x="45" y="120">ring / sqlite3 / blake3 compiler declarations</text><rect x="520" y="60" width="450" height="80" fill="#e5f5ef"/><text x="535" y="90">Rust self-contained CRT + libc</text><text x="535" y="120">6 exact hashes; map + final invocation</text><path d="M240 145 L450 190 M740 145 L550 190" stroke="#34495e" fill="none"/><rect x="230" y="190" width="550" height="65" fill="#eef0f5"/><text x="245" y="217">b985 static PIE candidate / retained CLI checks</text><text x="245" y="242">inventory only; source predates lock correction</text><text x="30" y="300">Bounded review: no candidate rerun, image build, or scientific execution.</text></g></svg>'''
(D/'provenance.svg').write_text(svg)
escape=lambda s:s.replace('\\','\\textbackslash{}').replace('_','\\_').replace('#','\\#').replace('%','\\%').replace('&','\\&')
paras=text.split('\n\n')[1:]
tex='\\documentclass[10pt]{article}\n\\usepackage[margin=0.65in]{geometry}\n\\setlength{\\parskip}{6pt}\n\\begin{document}\n\\section*{Native link provenance review}\n'+ '\n\n'.join(escape(x) for x in paras)+'\n\\end{document}\n'
(D/'report.tex').write_text(tex)
p=subprocess.run(['pdflatex','-interaction=nonstopmode','-halt-on-error','report.tex'],cwd=D,capture_output=True,text=True,timeout=30)
(D/'render.stdout').write_text(p.stdout);(D/'render.stderr').write_text(p.stderr)
assert p.returncode==0
print(json.dumps({'loaded_bytes':cli['recomputed_loaded_evidence_bytes'],'references':len(rows),'render_exit':p.returncode}))
