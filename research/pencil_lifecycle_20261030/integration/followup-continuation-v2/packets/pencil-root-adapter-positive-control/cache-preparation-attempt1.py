from pathlib import Path
import os,json,hashlib,subprocess,time,resource,shutil,stat
P=Path('/home/ec2-user/crypto-pencil-signed-graph-20261029/research/pencil_lifecycle_20261030/integration/followup-continuation-v2/packets/pencil-root-adapter-positive-control');x=json.loads((P/'source-materialization.json').read_text());w=Path(x['workspace']);start=time.monotonic();resource.setrlimit(resource.RLIMIT_AS,(2*1024**3,2*1024**3));receipt={'status':'failed','elapsed_seconds':None,'source_preparation_overhead_before_this_attempt':'not cumulatively metered','limits':{'total_seconds':180,'cache_bytes':3*1024**3,'vendor_bytes':2*1024**3},'compiler_invoked':False};rows={}
def guard():
 if time.monotonic()-start>160:raise RuntimeError('preparation work deadline')
 if shutil.disk_usage(w).free<6*1024**3:raise RuntimeError('free disk below6GiB')
def H(f):
 guard();h=hashlib.sha256()
 with f.open('rb') as stream:
  for b in iter(lambda:stream.read(4*1024**2),b''):h.update(b)
 return h.hexdigest()
def regulars(root):
 files=[]
 for f in root.rglob('*'):
  guard();mode=f.lstat().st_mode
  if stat.S_ISLNK(mode):raise RuntimeError('source symlink: '+str(f))
  if stat.S_ISREG(mode):files.append(f)
  elif not stat.S_ISDIR(mode):raise RuntimeError('source special file: '+str(f))
 return sorted(files)
try:
 receipt['initial_free_bytes']=shutil.disk_usage(w).free
 assert receipt['initial_free_bytes']>=12*1024**3,'initial12GiB disk requirement'
 cache_source=Path('/home/ec2-user/crypto-pencil-signed-graph-20261029/target/release');cache_files=[]
 for name in ['deps','build','.fingerprint']:cache_files+=regulars(cache_source/name)
 cache_bytes=sum(f.stat().st_size for f in cache_files);assert cache_bytes<=3*1024**3
 registry=json.loads((P/'registry-preflight.json').read_text());assert not registry['missing'];vendor_items=[];vendor_bytes=0
 for package in registry['found']:
  src=Path(package['paths'][0]);checksum=json.loads((src/'.cargo-checksum.json').read_text());assert checksum['package']==package['checksum'];files=regulars(src);vendor_bytes+=sum(f.stat().st_size for f in files);vendor_items.append((src,files,checksum))
 assert vendor_bytes<=2*1024**3
 receipt.update(cache_source_bytes=cache_bytes,vendor_source_bytes=vendor_bytes,cache_file_count=len(cache_files),vendor_package_count=len(vendor_items))
 target=w/'target/release';target.mkdir(parents=True,exist_ok=False)
 for name in ['deps','build','.fingerprint']:
  guard();subprocess.run(['cp','-a','--reflink=auto',str(cache_source/name),str(target/name)],check=True,timeout=max(1,160-(time.monotonic()-start)))
 for src in cache_files:
  dst=target/src.relative_to(cache_source);before=src.stat();a=H(src);b=H(dst);after=src.stat();assert (before.st_size,before.st_mtime_ns)==(after.st_size,after.st_mtime_ns) and a==b
  assert src.stat().st_ino!=dst.stat().st_ino or src.stat().st_dev!=dst.stat().st_dev,'writable hardlink'
  rows[str(dst)]={'source':str(src),'sha256':a,'bytes':after.st_size}
 vendor=w/'vendor';vendor.mkdir()
 for src,files,checksum in vendor_items:
  guard();dstroot=vendor/src.name;subprocess.run(['cp','-a','--reflink=auto',str(src),str(dstroot)],check=True,timeout=max(1,160-(time.monotonic()-start)))
  for rel,digest in checksum['files'].items():assert H(dstroot/rel)==digest,'registry file checksum mismatch'
  for f in files:
   dst=dstroot/f.relative_to(src);a=H(f);assert H(dst)==a;assert (f.stat().st_dev,f.stat().st_ino)!=(dst.stat().st_dev,dst.stat().st_ino);rows[str(dst)]={'source':str(f),'sha256':a,'bytes':dst.stat().st_size}
 home=w/'cargo-home';home.mkdir();(home/'config.toml').write_text('[source.crates-io]\nreplace-with="vendored-sources"\n[source.vendored-sources]\ndirectory='+json.dumps(str(vendor))+'\n[net]\noffline=true\n');(w/'tmp').mkdir()
 guard();receipt['status']='passed'
except BaseException as e:receipt['error']=repr(e)
finally:
 receipt['elapsed_seconds']=time.monotonic()-start;receipt['final_free_bytes']=shutil.disk_usage(w).free;receipt['verified_file_count']=len(rows);(P/'cache-preparation-inputs.json').write_text(json.dumps(rows,indent=2)+'\n');(P/'cache-preparation-receipt.json').write_text(json.dumps(receipt,indent=2)+'\n');print(json.dumps(receipt))
raise SystemExit(0 if receipt['status']=='passed' else 1)
