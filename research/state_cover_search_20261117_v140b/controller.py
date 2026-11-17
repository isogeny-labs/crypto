def gray_order(bits):
 assert 0<=bits<=8
 values=[i^(i>>1)for i in range(1<<bits)];assert len(set(values))==1<<bits and set(values)==set(range(1<<bits));assert all((a^b).bit_count()==1 for a,b in zip(values,values[1:]));return values

def assumption_oracle(z3,make_solver,seed):
 X=[z3.Bool('cover_oracle_'+str(i))for i in range(4)];state=[z3.Xor(X[0],X[1]),z3.Xor(X[2],X[3])];S=make_solver(z3,seed);S.set(timeout=1000);S.add(z3.Not(z3.Xor(z3.And(X[0],X[2]),X[1])));found=set();calls=0
 for cell in gray_order(2):
  assumptions=[w if cell>>j&1 else z3.Not(w)for j,w in enumerate(state)]
  while True:
   answer=S.check(*assumptions);calls+=1
   if answer==z3.unsat:break
   assert answer==z3.sat,'smallassumptionoracleunknown';m=S.model();x=sum(int(z3.is_true(m.eval(w,model_completion=True)))<<i for i,w in enumerate(X));assert ((x&1)^((x>>1)&1))|((((x>>2)&1)^((x>>3)&1))<<1)==cell;assert x not in found;found.add(x);S.add(z3.Or(*[z3.Not(w)if x>>i&1 else w for i,w in enumerate(X)]))
 expected={x for x in range(16)if(((x&1)&((x>>2)&1))^((x>>1)&1))==0};assert found==expected
 return {'source_assignments':16,'cells':4,'complete_toy_solutions':len(found),'native_calls':calls,'all_expected_tuples_equal':True}
