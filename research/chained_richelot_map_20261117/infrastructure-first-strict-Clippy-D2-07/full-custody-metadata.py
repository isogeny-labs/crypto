"""Metadata, byte custody, and disposable render-cache layout only."""
from pathlib import Path
from datetime import datetime, timezone
import hashlib, json, os, shutil, tarfile

r=Path('/workspace/crypto-chained-balanced-main')
s=r/'research/chained_richelot_map_20261117'
o=s/'infrastructure-first-strict-Clippy-D2-07';o.mkdir(exist_ok=False)
h=Path('/dev/shm/ct745-first-map14-D2')
def sha(p): return hashlib.sha256(p.read_bytes()).hexdigest()
def meta(p):return {'path':str(p),'bytes':p.stat().st_size,'sha256':sha(p),'mode':p.stat().st_mode&0o7777}
phase=json.loads((h/'all-phases.json').read_text())
assert phase['first_failure']=='strict-direct-clippy'
assert next(x for x in phase['phases'] if x['name']=='compile-only')['exit']==0
assert not any(x['name']=='first-frozen-arithmetic' for x in phase['phases'])
assert 'items after a test module' in (h/'14-strict-direct-clippy.stderr').read_text()

# Only derived page-raster caches. Original report sources/PDFs and raw runs stay.
groups=[
 ('ct60-',Path('/workspace/crypto/research/chained_jacobian_transfers_20261110/reports/quartic-gluing/report.pdf')),
 ('ct61-',Path('/workspace/crypto/research/chained_jacobian_transfers_20261110/reports/independent-kernel/replay-report.pdf')),
 ('ct604-',Path('/workspace/crypto-chained-main-validated/research/chained_jacobian_transfers_20261110/native-richelot-chart-boundary-10/REPORT.pdf')),
 ('ct605-',Path('/workspace/crypto-chained-main-validated/research/chained_jacobian_transfers_20261110/native-richelot-chart-boundary-10/REPORT.pdf'))]
cache=[]
for prefix,pdf in groups:
 assert pdf.read_bytes().startswith(b'%PDF-')
 for p in sorted(Path('/workspace/scratch').glob(prefix+'*.png')):
  cache.append(dict(meta(p),retained_original_pdf=meta(pdf)))
receipt={'time':datetime.now(timezone.utc).isoformat(),'role':'disposable PDF page raster caches only; no original PDF/source/raw experiment deletion','files':cache}
(o/'DERIVED_PAGE_RASTER_CACHE_RECLAMATION.json').write_text(json.dumps(receipt,indent=2)+'\n')
for row in cache:
 p=Path(row['path']);assert sha(p)==row['sha256'];p.unlink()

files={ 'hold/'+p.relative_to(h).as_posix():p for p in sorted(h.rglob('*')) if p.is_file() }
for name in ['ct745-first-map14-D2-caller.stdout','ct745-first-map14-D2-caller.stderr','ct745-first-map14-D2-caller.exit','ct740-actual-source-D-commit.json','ct740-source-D-push.stdout','ct740-source-D-push.stderr','ct740-source-D-push.exit','ct740-source-D-exact-readback.txt']:
 p=Path('/workspace/scratch')/name;assert p.is_file();files['caller-and-publication/'+name]=p
archive=Path('/run/lock/ct746-complete-first-D2.tar.xz');assert not archive.exists()
with tarfile.open(archive,'w:xz',preset=9) as t:
 for name,p in files.items():t.add(p,arcname=name,recursive=False)
inventory=[]
with tarfile.open(archive,'r:xz') as t:
 assert len(t.getmembers())==len(files)
 for name,p in files.items():
  m=t.getmember(name);assert m.isfile() and m.mode==p.stat().st_mode&0o7777
  b=t.extractfile(m).read();assert b==p.read_bytes()
  inventory.append({'member':name,'bytes':len(b),'sha256':hashlib.sha256(b).hexdigest(),'mode':m.mode})
dest=o/'complete-first-strict-Clippy-D2-source-full-rlibs-compiled-ELF-and-logs.tar.xz'
shutil.copy2(archive,dest);assert sha(archive)==sha(dest);archive.unlink()
(o/'WHOLE_ARCHIVE_MEMBERS.json').write_text(json.dumps(inventory,indent=2)+'\n')
data={'time':datetime.now(timezone.utc).isoformat(),'source_commit':phase['source_commit'],'first_failure':'strict-direct-clippy: items_after_test_module in shifted.rs; new bridge follows retained original test module','phases':len(phase['phases']),'main_compile_exit':0,'strict_main_Clippy_exit':1,'scientific_arithmetic_calls':0,'source_and_dependency_guards_after':0,'full_archive':dict(meta(dest),members=len(inventory),all_full_bytes_and_modes_compared=True),'prospectively_excluded_compiler_temp_inventory':json.loads((h/'prospectively-excluded-compiler-temp-inventory.json').read_text()),'next_E':'Only relocate entire unchanged new visibility adapter before retained original test module; no allowance, arithmetic, fixture, population, sign, timeout or resource-floor change.'}
(o/'ACTUAL_FIRST_STRICT_CLIPPY_D2.json').write_text(json.dumps(data,indent=2)+'\n')
# Compact only loose copies after exact full archive comparison. Keep all logs.
removed=[]
for p in sorted(h.rglob('*')):
 if p.is_file() and (p.is_relative_to(h/'source') or p.is_relative_to(h/'deps') or p.name=='compiler-output'):
  name='hold/'+p.relative_to(h).as_posix();row=next(x for x in inventory if x['member']==name)
  assert p.stat().st_size==row['bytes'] and sha(p)==row['sha256'];removed.append(row);p.unlink()
for p in sorted(h.rglob('*'),key=lambda x:len(x.parts),reverse=True):
 if p.is_dir() and not any(p.iterdir()):p.rmdir()
(o/'ARCHIVE_EQUAL_LOOSE_COPIES_COMPACTION.json').write_text(json.dumps({'time':datetime.now(timezone.utc).isoformat(),'files':removed,'full_primary_archive':meta(dest)},indent=2)+'\n')
print(json.dumps({'archive_bytes':dest.stat().st_size,'members':len(inventory),'archive_sha256':sha(dest),'disk_free':shutil.disk_usage(r).free,'shm_free':shutil.disk_usage(h).free,'scratch_free':shutil.disk_usage('/run/lock').free}))
