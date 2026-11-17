"""L08 refit: integer Schoenhage gadget with (J,k) freed, mirroring the freed char-2 gadget.
Validity: L power of 2, L >= 2n, L | 2J (root 2^(2J/L) of order L mod 2^J+1),
no wraparound: coefficients < n*2^(2k) <= 2^J  i.e. 2k + ceil(log2 n) <= J."""
import sys, json, math, random
REF="/Volumes/SSD990-2/cryptanalysis/experiments/hashed-ecdl-tensor-reference"
sys.dont_write_bytecode=True; sys.path.insert(0,REF)
from tensor_encoding import WordCircuit
from tensor_prime import PrimeCurveEncoding
import eccp, ecc2k

def valid(d,J,k):
    n=(d+k-1)//k; L=1
    while L<2*n: L*=2
    return (2*J)%L==0 and 2*k+math.ceil(math.log2(n))+1<=J, n, L

class FreeInt(WordCircuit):
    CHOICE={}   # d -> (J,k)
    def integer_multiply(self, left, right):
        d=max(len(left),len(right))
        if d not in self.CHOICE: return WordCircuit.integer_multiply(self,left,right)
        left=tuple(left)+self.constant(0,d-len(left)); right=tuple(right)+self.constant(0,d-len(right))
        J,k=self.CHOICE[d]; ok,n,L=valid(d,J,k); assert ok
        e=L.bit_length()-1; L1,L2=1<<(e//2),1<<((e+1)//2)
        M,root=(1<<J)+1,1<<(2*J//L)
        add=lambda a,b:(a+b)%M; mul=lambda a,b:a*b%M
        P=[pow(root,i,M) for i in range(L)]; assert pow(root,L,M)==1 and (L==1 or pow(root,L//2,M)!=1)
        IP=[P[-i%L] for i in range(L)]
        def chunks(w):
            w=w+self.constant(0,n*k-d); return [w[i*k:(i+1)*k] for i in range(n)]+[self.constant(0,k)]*(L-n)
        T=[self.transform(chunks(w),L1,L2,J+1,P,add,mul,name="integer_fft") for w in (left,right)]
        pr=[self.compute((a,b),(J+1,),lambda v:(mul(*v),),"integer_ring_product")[0] for a,b in zip(*T)]
        co=self.transform(pr,L1,L2,J+1,IP,add,mul,pow(L,-1,M),"integer_ifft")
        for w in co[2*n-1:]: self.equal(w,self.constant(0,len(w)))
        co=co[:2*n-1]+[self.constant(0,J+1)]
        carry,res=self.constant(0,J+1),[]
        for c in co:
            lo,carry=self.compute((c,carry),(k,J+1),lambda v:((v[0]+v[1])%(1<<k),(v[0]+v[1])>>k),"carry"); res.extend(lo)
        self.equal(carry,self.constant(0,len(carry))); self.equal(tuple(res[2*d:]),self.constant(0,len(res)-2*d))
        self.mul_metadata.append({"type":"integer_free","input_bits":d,"ring_bits":J+1,"transform_length":L,"k":k})
        return tuple(res[:2*d])

def gadget_edges(d,choice=None):
    c=FreeInt();
    if choice: c.CHOICE={d:choice}
    a=c.word(d); b=c.word(d); out=c.integer_multiply(a,b)
    return c.statistics(tuple(dict.fromkeys(a+b+out)))["boolean_network_edges"]
