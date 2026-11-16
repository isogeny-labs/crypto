from pathlib import Path
import json,hashlib,tarfile,io,lzma,base64
C=Path(__file__).resolve().parent
for n,h in json.loads((C/'EVIDENCE_SHA256.json').read_text()).items():assert hashlib.sha256(Path(n).read_bytes()).hexdigest()==h,n
idx=json.loads((C/'CHILD_INDEX.json').read_text());raw=lzma.decompress(base64.b64decode((C/'CHILD_EVIDENCE.tar.xz.b64').read_text()))
with tarfile.open(fileobj=io.BytesIO(raw))as t:
 assert set(t.getnames())==set(idx)and len(idx)==36
 for n,h in idx.items():assert hashlib.sha256(t.extractfile(n).read()).hexdigest()==h,n
P=Path('research/rationality_trace_integration_20261116_v83');a=json.loads((P/'oracles_raw.json').read_text());b=json.loads((P/'cold_raw.json').read_text());g=json.loads(Path('research/norm_common_root_gate_20261116_v83b/raw.json').read_text());assert all(v['error']is None for v in [a,b,g]);assert(a['out']['small_cases'],a['out']['small_support_cases'],a['out']['actual_cases'],a['out']['new_main_rejections'])==(1976,1976,36,0);assert len(b['out']['runs'])==9 and b['out']['new_main_rejections']==0
v=g['out'];assert(v['cases'],v['primitive'],v['constant_b'],v['common_rational'],v['common_anti'],v['unresolved_b_zero'])==(480,120,120,157,83,12);assert len(v['field_cases'])==12
assert not json.loads((C/'LEDGER_VALIDATION.txt').read_text())['problems'];print('C83 hashes,36child files,integration negative outcome and common-root gate checks verified; no qualified solver ratio')
