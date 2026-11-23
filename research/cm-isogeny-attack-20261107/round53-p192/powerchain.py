P192=2**192-2**64-1
def nist_power(x,multiply):
 cache={1:x}
 for width in [2,4,8,16,32,64]:
  y=cache[width//2]
  for _ in range(width//2):y=multiply(y,y)
  cache[width]=multiply(y,cache[width//2])
 y=cache[64]
 for width in [32,16,8,4,2,1]:
  for _ in range(width):y=multiply(y,y)
  y=multiply(y,cache[width])
 y=multiply(y,y)
 for _ in range(64):y=multiply(y,y)
 return multiply(y,cache[64])
def audit():
 count=[0]
 def combine(a,b):count[0]+=1;return a+b
 exponent=nist_power(1,combine);assert exponent==P192 and count[0]==204
 return {'exact_exponent':exponent,'multiplication_gates':count[0],'uses_inversion':False,'valid_at_zero':True}
