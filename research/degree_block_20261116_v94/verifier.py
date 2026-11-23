"""Certificate checker: independent substitution and linear solution verification."""
def plus(a,b):return sorted(set(a).symmetric_difference(b))
def evaluate(p,z):return sum(1 for m in p if m&z==m)&1
def compose(p,table):
 answer=[]
 for m in p:
  terms=[0]
  for j,q in enumerate(table):
   if m&(1<<j):
    out=[]
    for a in terms:
     for b in q:out=plus(out,[a|b])
    terms=out
  answer=plus(answer,terms)
 return answer

def verify(original,n,report):
 current=[sorted(q)for q in original];total=[[1<<j]for j in range(n)]
 for stage in report['stages']:
  assert stage['n']==n
  equations=[]
  for w in stage['linear_witnesses']:
   assert type(w)is int and 0<=w<1<<len(current)
   q=[]
   for i,p in enumerate(current):
    if w>>i&1:q=plus(q,p)
   assert all(m.bit_count()<=1 for m in q);equations.append(q)
  # Independent row rank/consistency with low pivots, unlike producer.
  rows={};bad=False
  for p in equations:
   a=sum((m<<1)if m else 1 for m in p)
   while a>>1:
    bit=(a&~1)&-(a&~1)
    if bit not in rows:rows[bit]=a;break
    a^=rows[bit]
   if a==1:bad=True
  if stage['inconsistent']:
   assert bad and report['unsat'];return True
  assert not bad
  table=stage['maps'];k=stage['next_n'];assert len(table)==n and k==n-len(rows)
  for q in table:assert len(q)==len(set(q))and all(m==0 or(m.bit_count()==1 and m<1<<k)for m in q)
  assert all(compose(q,table)==[]for q in equations)
  # Dimension plus an explicit identity coordinate for every free input proves injectivity.
  assert all([1<<j]in table for j in range(k))
  current=[compose(q,table)for q in current];total=[compose(q,table)for q in total];n=k
 assert not report['unsat'] and report['n']==n
 assert [q for q in current if q]==report['polys']and total==report['maps']
 return True
