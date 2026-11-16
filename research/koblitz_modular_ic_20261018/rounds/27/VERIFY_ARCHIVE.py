#!/usr/bin/env python3
"""Validate a fresh object download and deterministic repack, not math replay."""
from pathlib import Path
import hashlib,json,subprocess,sys,tarfile
rid=int(sys.argv[1]);download=Path(sys.argv[2]).resolve();work=Path(sys.argv[3]).resolve();assert not work.exists();work.mkdir(parents=True)
here=Path(__file__).resolve().parent.parent/str(rid)
meta=json.loads((here/'raw-meta.json').read_text());digest=hashlib.sha256(download.read_bytes()).hexdigest();assert digest==meta['raw_sha256']
with tarfile.open(download)as tar:
 for m in tar.getmembers():
  p=Path(m.name);assert not p.is_absolute() and '..'not in p.parts
  assert not m.issym() and not m.islnk()
subprocess.run(['tar','--extract','--xz','--file',str(download),'--directory',str(work),'--no-same-owner','--same-permissions'],check=True)
expected={}
for line in(here/'runs.sha256').read_text().splitlines():
 h,name=line.split(maxsplit=1);expected[name]=h
actual={str(p.relative_to(work)):hashlib.sha256(p.read_bytes()).hexdigest()for p in work.rglob('*')if p.is_file()}
assert actual==expected
repack=work.parent/(work.name+'-repacked.tar.xz')
subprocess.run(['tar',*meta['tar_flags'],'-cJf',str(repack),'-C',str(work),*meta['archive_contents']],check=True)
repacked=hashlib.sha256(repack.read_bytes()).hexdigest();assert repacked==digest
out={'schema':'koblitz.artifact-verification/v1','round':rid,'bucket':'setec-astronomy-artifacts','download_sha256':digest,'expected_sha256':meta['raw_sha256'],'bytes':download.stat().st_size,'regular_files_verified':len(actual),'fresh_GetObject_download':True,'local_artifact_cache_used':False,'all_member_hashes_verified':True,'deterministic_repack_verified':True,'scientific_replay':False}
(here/'artifact-verification.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps(out))
