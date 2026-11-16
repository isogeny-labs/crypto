from pathlib import Path
import ctypes,hashlib,json,subprocess,sys,tempfile
P=Path(__file__).resolve().parent;R=P.parents[1]
for name,digest in json.loads((P/'EVIDENCE_SHA256.json').read_text()).items():
 data=(R/name).read_bytes();assert hashlib.sha256(data).hexdigest()==digest;x=json.loads(data);assert x['error']is None
out=x['out'];assert out['cases']==329 and out['supported']==38 and out['rejected']==291;assert out['row_verification']==dict(actual_systems=138,synthetic_valid_systems=2,rejected_corruptions=3,portable_field_arithmetic=True)
old=json.loads((R/'research/native_quintic_support_20261115_v48/raw.json').read_text());assert out['flags']==old['out']['flags']
if '--build'in sys.argv:
 from rootfinder.binaryfield import BinaryField
 from research.native_validation_20261115_v49.worker import check_rows
 with tempfile.TemporaryDirectory(dir='/dev/shm')as folder:
  libpath=Path(folder)/'verify.so';subprocess.run(['g++','-O3','-std=c++17','-shared','-fPIC',str(R/'research/native_validation_20261115_v49/validation.cpp'),'-o',str(libpath)],check=True,capture_output=True,timeout=12);lib=ctypes.CDLL(str(libpath));values=(R/'research/root_conditioned_native_20261115_v45/matrix.txt').read_text().split();v=[int(t,16)for t in values[1:]];base=v[:6];cols=[v[i:i+6]for i in range(6,len(v),6)];result=check_rows(lib,BinaryField(83,0x800000000200000000007),base,cols);assert result==out['row_verification']
print('Verified native-validation receipt and329 support flags; --build independently rebuilds and repeats140valid/3corrupted row checks.')
