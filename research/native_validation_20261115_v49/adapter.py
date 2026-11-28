import ctypes

def verify(lib,base,cols,records):
 words=[]
 for H in [base]+list(cols):
  assert len(H)==6
  for v in H:words.extend([v&((1<<64)-1),v>>64])
 checks=[];basis=[]
 for r in records:
  value=int(r['particular'],16);offset=len(basis)//2
  checks.extend([r['x'],r['rank'],int(r['consistent']),len(r['basis']),value&((1<<64)-1),value>>64,offset,0])
  for t in r['basis']:
   v=int(t,16);basis.extend([v&((1<<64)-1),v>>64])
 array=lambda v:(ctypes.c_uint64*len(v))(*v)
 lib.verify83.argtypes=[ctypes.POINTER(ctypes.c_uint64),ctypes.c_size_t,ctypes.POINTER(ctypes.c_uint64),ctypes.c_size_t,ctypes.POINTER(ctypes.c_uint64),ctypes.c_size_t];lib.verify83.restype=ctypes.c_int
 return lib.verify83(array(words),len(cols),array(checks),len(records),array(basis),len(basis)//2)
