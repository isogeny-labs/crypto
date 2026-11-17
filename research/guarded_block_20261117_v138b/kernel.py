from research.local_split_circuit_20261116_v100.kernel import Circuit
from research.coprime_factor_checks_20261116_v119b.circuit import coefficient
from research.degree_clipped_checks_20261117_v125c.kernel import generate,isolated
from research.clip_defect_checks_20261117_v127c.kernel import defects
from research.product_tree_checks_20261117_v126c.kernel import product_tree
from research.shared_cubic_prefix_20261117_v122c.circuit import xor_poly
from research.residue_quotient_20261116_v99b.kernel import basis,preimage,apply
from research.delayed_block_20261117_v137b.kernel import complement,coefficients
from research.precision_filtration_20261117_v132b.kernel import filtration

def reduction(rows,width):
 a=list(rows);comb=[1<<i for i in range(len(a))];piv=[];r=0
 for col in range(width):
  k=next((i for i in range(r,len(a))if a[i]>>col&1),None)
  if k is None:continue
  a[r],a[k]=a[k],a[r];comb[r],comb[k]=comb[k],comb[r]
  for i in range(len(a)):
   if i!=r and a[i]>>col&1:a[i]^=a[r];comb[i]^=comb[r]
  piv.append(col);r+=1
 S=[0]*width
 for i,p in enumerate(piv):S[p]=comb[i]
 free=[j for j in range(width)if j not in piv];K=[(1<<j)^sum(((a[i]>>j)&1)<<p for i,p in enumerate(piv))for j in free]
 return {'rank':r,'S':S,'kernel':K,'left_null':comb[r:],'free':free,'pivots':piv}

def numeric(rec,b,t):
 z=sum(((row&b).bit_count()%2)<<j for j,row in enumerate(rec['S']))^apply(rec['kernel'],t&((1<<len(rec['kernel']))-1));valid=all((row&b).bit_count()%2==0 for row in rec['left_null'])and t>>len(rec['kernel'])==0
 return z,valid

def catalog(records,width,m):
 slots=max(len(r['kernel'])for r in records);left=max(len(r['left_null'])for r in records);values=[]
 for rec in records:
  bits=[row>>k&1 for row in rec['S']for k in range(m)];bits +=[(rec['kernel'][t]>>j&1)if t<len(rec['kernel'])else 0 for t in range(slots)for j in range(width)];bits +=[(rec['left_null'][i]>>k&1)if i<len(rec['left_null'])else 0 for i in range(left)for k in range(m)];bits +=[int(t<len(rec['kernel']))for t in range(slots)];values.append(bits)
 return values,slots,left

def emit_catalog(C,state_wires,records,width,m):
 values,slots,left=catalog(records,width,m);cache={0:1};forms={};monomials={}
 def term(mask):
  if mask not in cache:
   low=mask&-mask;cache[mask]=C.op('and',state_wires[low.bit_length()-1],term(mask^low))
  return cache[mask]
 def emit(truth):
  key=tuple(truth)
  if key not in forms:
   a=list(truth)
   for bit in range(len(state_wires)):
    for mask in range(len(a)):
     if mask>>bit&1:a[mask]^=a[mask^(1<<bit)]
   value=0
   for mask,v in enumerate(a):
    if v:value=C.xor(value,term(mask))
   forms[key]=value
  return forms[key]
 flat=[emit([row[j]for row in values])for j in range(len(values[0]))];off=0;S=[flat[j*m:(j+1)*m]for j in range(width)];off+=width*m;N=[flat[off+t*width:off+(t+1)*width]for t in range(slots)];off+=slots*width;L=[flat[off+i*m:off+(i+1)*m]for i in range(left)];off+=left*m;active=flat[off:]
 return {'S':S,'N':N,'L':L,'active':active,'flat':flat,'slots':slots,'left':left,'function_count':len(flat),'distinct_functions':len(forms),'monomial_count':len(cache)}

def xor_all(C,values):
 out=0
 for v in values:out=C.xor(out,v)
 return out

def correction_cofactors(F,d,ns,C):
 A=[[0],[0],[1]];B=[[1],[1],[1],[1]];m=d-1;result={}
 for n in range(1,max(ns)+1):
  e=[C.xor(F[j][n]if n<len(F[j])else 0,coefficient(A,B,j,n,C))for j in range(5)];a1=C.xor(e[0],e[1]);a=[e[0],a1];b=[C.xor(e[1],e[2]),C.xor(e[1],e[3]),C.xor(a1,e[4])]
  if n in ns:result[n]=a1
  for factor,degree,values in [(A,2,a),(B,3,b)]:
   for j,v in enumerate(values):
    if n<=(degree-j)*m:factor[j].append(v)
 return [result[n]for n in ns]

def build(base,columns,d,count,K,ell,ns,table,max_early_rank=8,max_nodes=200000):
 width=len(K);U=complement(K,len(columns));ny=len(U);early=filtration(columns,max(ns)-ell+1)['lower_columns'];piv,_=basis(early);assert len(piv)<=max_early_rank;keys=sorted(piv);evec=[piv[k][0]for k in keys];ep,_=basis(evec);lookup={int(r['signature_hex'],16):r['rows']for r in table};assert len(lookup)==1<<len(keys);records=[reduction(lookup[apply(evec,x)],width)for x in range(1<<len(keys))];vals,slots,left=catalog(records,width,len(ns));assert min(r['rank']for r in records)>=1;C=Circuit(ny+slots);state_cols=[preimage(apply(early,u),ep)for u in U];state_wires=[xor_all(C,[j+2 for j,c in enumerate(state_cols)if c>>k&1])for k in range(len(keys))];ops=emit_catalog(C,state_wires,records,width,len(ns));ucols=[coefficients(columns,u)for u in U];F=[]
 for j,b in enumerate(base):F.append([xor_all(C,[b>>k&1]+[i+2 for i,col in enumerate(ucols)if col[j]>>k&1])for k in range((5-j)*(d-1)+1)])
 b=correction_cofactors(F,d,ns,C);cofactor_nodes=len(C.nodes)-2;t=[ny+i+2 for i in range(slots)];z=[xor_all(C,[C.op('and',s,v)for s,v in zip(ops['S'][j],b)]+[C.op('and',ops['N'][i][j],t[i])for i in range(slots)])for j in range(width)];compat=[xor_all(C,[C.op('and',s,v)for s,v in zip(row,b)])for row in ops['L']];canonical=[C.op('and',C.op('not',active),w)for active,w in zip(ops['active'],t)];D=[coefficients(columns,v)for v in K];FF=[[xor_all(C,[w]+[z[i]for i,col in enumerate(D)if col[j]>>k&1])for k,w in enumerate(group)]for j,group in enumerate(F)];theta=[xor_all(C,[j+2 for j,u in enumerate(U)if u>>i&1]+[z[j]for j,v in enumerate(K)if v>>i&1])for i in range(len(columns))];_,roots=generate(C,FF,d);selected=[r for r in defects(FF,d,C)if r['factor']=='A'][:count];corrections=[0 if r['precision']in ns and r['coefficient']==1 else r['wire']for r in selected];cc=product_tree(C,roots,(0,(1,(2,(3,4)))));checks=[]
 for a,v in zip(cc,[list(reversed(g))for g in FF]):checks+=xor_poly(a,v,C)
 oo=[{'exact_raw_root_product':True,'wires':[checks]},{'correction_basis_prefix':count,'wires':[corrections]},{'block_compatibility':True,'wires':[compat]},{'canonical_free_slots':True,'wires':[canonical]}];assert len(C.nodes)<=max_nodes,'blocknodecap';G,oo,rr=isolated(C,oo,roots+[theta,z,state_wires,b]+FF)
 meta={'inputs':G.n,'quotient_inputs':ny,'free_slots':slots,'variables':len(G.nodes)-2,'unpruned_variables':len(C.nodes)-2,'cofactor_nodes_before_final_prune':cofactor_nodes,'early_bits':len(keys),'states':len(records),'function_count':ops['function_count'],'distinct_functions':ops['distinct_functions'],'monomial_count':ops['monomial_count'],'rank_distribution':{str(r):sum(q['rank']==r for q in records)for r in sorted({q['rank']for q in records})},'U_hex':[hex(v)for v in U],'K_hex':[hex(v)for v in K]}
 return G,oo,rr[:5],rr[5],rr[6],rr[7],rr[8],rr[9:],meta
