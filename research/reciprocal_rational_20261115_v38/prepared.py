"""Unexecuted polynomial rational-reconstruction prototype; no pair enumerator."""
def degree(x):return x.bit_length()-1
def divide(a,b):
 assert b;q=0
 while degree(a)>=degree(b):
  j=degree(a)-degree(b);q^=1<<j;a^=b<<j
 return q,a
def multiply(a,b):
 v=0
 while b:
  if b&1:v^=a
  b>>=1;a<<=1
 return v
def reconstruct(modulus,t,source_degree_bound):
 """Caller certifies irreducible modulus; exact unique bounded fraction or absent."""
 n=degree(modulus);d=source_degree_bound;A=d-1;B=2*(d-1)
 assert d>=1 and A+B<n and 0<=t<1<<n
 if not t:return dict(status='fraction',numerator=0,denominator=1)
 r0,r1=modulus,t;s0,s1=0,1
 while degree(r1)>A:
  q,r2=divide(r0,r1);s2=s0^multiply(q,s1);r0,r1=r1,r2;s0,s1=s1,s2
 if degree(s1)>B:return dict(status='no_bounded_fraction')
 assert s1 and divide(multiply(t,s1),modulus)[1]==r1
 return dict(status='fraction',numerator=r1,denominator=s1)
