"""Layered integration audit; kernel bit-volume correctness is checked upstream.

Execute the actual adapter then check each source result against explicit A.
Fixed kernel counts are independent formulas; data-dependent descriptor
volumes are preserved and summed, not claimed as a second independent proof.
"""
from adapter import FusedSchur
from reference import expected_fixed_counts
import numpy as np
import hashlib,json

def digest(x):return hashlib.sha256(json.dumps(x,sort_keys=True).encode()).hexdigest()

class AuditedSchur(FusedSchur):
    def __init__(self,*args,dense_reference,**kw):
        super().__init__(*args,**kw)
        self.reference=np.array(dense_reference,dtype=np.int64);self.source_receipts=[]
        assert self.reference.shape==(len(self.G),len(self.G))
        assert len(self.G)*(self.m.r-1)**2<2**63
        assert np.all(self.reference>=0) and np.all(self.reference<self.m.r)
    def _source(self,x,transpose=False):
        before=dict(self.m.counts);offset=len(self.m.blocks)
        y=super()._source(x,transpose)
        ref=self.reference.T if transpose else self.reference
        assert y==(ref@np.array(x,dtype=np.int64)%self.m.r).tolist()
        delta={k:v-before.get(k,0) for k,v in self.m.counts.items() if k not in before or v!=before[k]}
        fixed=expected_fixed_counts(self.m.r,len(x),len(self.G[0]),self.a,self.b,transpose)
        for k,v in fixed.items():assert delta.get(k,0)==v,(k,delta.get(k,0),v)
        direction='source_transpose_calls' if transpose else 'source_forward_calls'
        assert delta.get(direction)==1
        blocks=self.m.blocks[offset:]
        assert len(blocks)==4*len(self.G[0])+8
        assert delta.get('operand_bitlength_product_sum',0)==sum(z['a_bits']*z['b_bits'] for z in blocks)
        assert delta.get('product_result_bitlength_sum',0)==sum(z['product_bits'] for z in blocks)
        assert all(0<=z['product_bits']<=z['a_bits']+z['b_bits'] for z in blocks)
        self.source_receipts.append(dict(transpose=transpose,input_sha256=digest(list(x)),output_sha256=digest(y),counts=delta,block_offset=offset,block_count=len(blocks)))
        return y

def check_ledger(obj,forward,transpose,extras):
    receipts=obj.source_receipts
    assert sum(not z['transpose'] for z in receipts)==forward
    assert sum(z['transpose'] for z in receipts)==transpose
    expected={}
    for z in receipts:
        for k,v in z['counts'].items():expected[k]=expected.get(k,0)+v
    for k,v in extras.items():expected[k]=expected.get(k,0)+v
    # Preserve explicit zero-valued ledger keys emitted by the kernel.
    expected={k:expected.get(k,0) for k in set(expected)|set(obj.m.counts)}
    assert obj.m.counts==expected,(obj.m.counts,expected)
    return expected

def check_generated(m,r,n,s,a,b,transpose):
    """Layered direct generated-operator check; exact output checked by caller."""
    fixed=expected_fixed_counts(r,n,s,a,b,transpose)
    for k,v in fixed.items():assert m.counts.get(k,0)==v,(k,m.counts.get(k),v)
    assert len(m.blocks)==4*s+8
    assert m.counts['operand_bitlength_product_sum']==sum(z['a_bits']*z['b_bits'] for z in m.blocks)
    assert m.counts['product_result_bitlength_sum']==sum(z['product_bits'] for z in m.blocks)
    assert all(0<=z['product_bits']<=z['a_bits']+z['b_bits'] for z in m.blocks)
    return dict(m.counts)
