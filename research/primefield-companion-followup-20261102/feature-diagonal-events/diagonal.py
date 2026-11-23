"""Compact actual-feature Gram diagonal. Derived, numerically unexecuted."""
from dataclasses import dataclass
from reference_constructor import power


def patterns(q):
    yield (0,)
    for d in range(1,q):
        yield (0,d)
    for d in range(1,q):
        for e in range(d+1,q):
            yield (0,d,e)


@dataclass(frozen=True)
class Diagonal:
    prime: int
    window: int
    gamma_inverse: int
    nonconstant_background: int
    background: int
    corrections: tuple

    def _field(self,m):
        if m.r!=self.prime:
            raise ValueError('same coefficient field required')

    def b_value(self,k,m):
        self._field(m)
        if type(k) is not int or not 0<=k<self.prime:
            raise ValueError('canonical evaluation coordinate required')
        z=(-k)%self.prime
        m.add('feature_diagonal_point_queries')
        return self.background if z>=self.window else m.sub(self.background,self.corrections[z])

    def a_value(self,i,m):
        self._field(m)
        if type(i) is not int or not 0<=i<self.prime:
            raise ValueError('canonical companion coordinate required')
        return m.mul(power(self.gamma_inverse,i,m),self.b_value(self.prime-1-i,m))

    def zero_diagonal_candidates(self,m):
        self._field(m)
        exceptional=[]
        for z,c in enumerate(self.corrections):
            if m.sub(self.background,c)==0:
                exceptional.append((z-1)%self.prime)
        m.add('feature_diagonal_zero_tests',self.window+1)
        return dict(exceptional_a_indices=tuple(sorted(exceptional)),
                    bulk_a_half_open_interval=(self.window-1,self.prime-1)
                        if self.background==0 else None,
                    zero_columns_certified=False,
                    zero_prefix_upper_bound=self.window-1 if self.background!=0 else None)

    def forbidden_constant_weights(self,m):
        """Diagnostic only: never replaces the caller's fixed weights."""
        self._field(m)
        values={m.sub(0,self.nonconstant_background)}
        values.update(m.sub(c,self.nonconstant_background) for c in self.corrections)
        return tuple(sorted(v for v in values if v!=0))


def build(r,q,gamma,weights,m):
    if (type(r) is not int or r<=3 or m.r!=r or type(q) is not int or not 1<=q<r or
            type(gamma) is not int or not 1<gamma<r):
        raise ValueError('caller-certified prime, proper window and gamma outside {0,1} required')
    count=1+q*(q-1)//2
    if len(weights)!=count+1 or any(type(w) is not int or not 0<w<r for w in weights):
        raise ValueError('canonical nonzero feature weights required')
    inverse=m.inv(gamma)
    inverse_gap=m.inv(m.sub(gamma,1))
    powers=[1]
    for i in range(q): powers.append(m.mul(powers[-1],gamma))
    inverse_powers=[1]
    for i in range(1,q): inverse_powers.append(m.mul(inverse_powers[-1],inverse))
    events=[0]*(q+1)
    background=0
    for index,P in enumerate(patterns(q)):
        w=weights[index+1]
        ell=q-P[-1]
        geometric=m.mul(m.sub(powers[ell],1),inverse_gap)
        background=m.plus(background,m.mul(w,geometric))
        for d in P:
            scaled=m.mul(w,inverse_powers[d])
            events[d]=m.plus(events[d],scaled)
            events[d+ell]=m.sub(events[d+ell],scaled)
            m.add('feature_diagonal_interval_events',2)
    correction=[]
    running=0
    for z in range(q):
        running=m.plus(running,events[z])
        correction.append(m.mul(powers[z],running))
    constant=m.plus(weights[0],background)
    m.add('feature_diagonal_patterns',count)
    return Diagonal(r,q,inverse,background,constant,tuple(correction))
