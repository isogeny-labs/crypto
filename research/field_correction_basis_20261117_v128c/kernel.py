from research.odd_coprime_factor_20261116_v119c.kernel import initial,inverse_block,multiply,matvec,trim

def audited_lift(F,d,f):
 leading=initial(f);assert [(v[0]if v else 0)for v in F]==[0,0]+leading;S,I=inverse_block(f);A=[[0],[0],[1]];B=[[b]for b in leading];m=d-1;audit=[]
 for n in range(1,3*m+1):
  old=multiply(A,B,f);e=[f.sub(F[j][n]if n<len(F[j])else 0,old[j][n]if n<len(old[j])else 0)for j in range(5)];correction=matvec(I,e,f);assert matvec(S,correction,f)==e;expired=[];basis=[]
  for factor,degree,values,label in [(A,2,correction[:2],'A'),(B,3,correction[2:],'B')]:
   for j,value in enumerate(values):
    if n<=(degree-j)*m:factor[j].append(value)
    else:
     expired.append(value)
     if label=='A':basis.append(value)
  audit.append({'precision':n,'error':e,'correction':correction,'expired':expired,'basis':basis})
 return {'A':A,'B':B,'audit':audit,'bounded_factorization':[trim(v)for v in multiply(A,B,f)]==[trim(v)for v in F]}
