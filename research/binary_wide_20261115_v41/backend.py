"""Provided-modulus wider binary fields; frozen legacy arithmetic preserved."""
from rootfinder import binaryfield as old

def rabin(modulus):
 n=modulus.bit_length()-1
 if n<1:return False
 if n==1:return True
 if not modulus&1 or modulus.bit_count()%2==0:return False
 checks={n//p for p in old._prime_divisors(n)};x=2;v=x
 for i in range(1,n+1):
  v=old._bit_remainder(old._bit_square(v),modulus)
  if i in checks and old._bit_gcd(v^x,modulus)!=1:return False
 return v==x
class WideBinaryField(old.BinaryField):
 __slots__=()
 def __init__(self,degree=83,modulus=None,*,max_degree=1024):
  old._integer(degree,'degree');old._integer(max_degree,'degree budget')
  if not 1<=degree<=max_degree:raise ValueError('degree exceeds declared field budget')
  if degree<=256:super().__init__(degree,modulus);return
  if modulus is None:raise ValueError('provide explicit modulus above degree256')
  old._integer(modulus,'modulus')
  if modulus<=0 or modulus.bit_length()!=degree+1:raise ValueError('modulus degree mismatch')
  if not rabin(modulus):raise ValueError('modulus must be irreducible')
  self.degree=degree;self.modulus=modulus;self.order=1<<degree;self.characteristic=2
