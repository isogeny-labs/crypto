"""Exact binary affine-fiber composition; no candidate selection."""
def xor(values):
 out=0
 for v in values:out^=v
 return out
def compose(base,guards,solve):
 if not base['consistent']:return dict(consistent=False,particular=None,basis=[],base_inconsistent=True)
 B=base['basis'];p=base['particular'];rows=[(sum(((mask&v).bit_count()%2)<<i for i,v in enumerate(B)),rhs^((mask&p).bit_count()%2))for mask,rhs in guards]
 small=solve(rows,len(B))
 lift=lambda t:xor(v for i,v in enumerate(B)if t>>i&1)
 return dict(consistent=small['consistent'],particular=p^lift(small['particular'])if small['consistent']else None,basis=[lift(v)for v in small['basis']],inner_rows=rows,inner_solution=small,inner_width=len(B))
def affine_values(sol):
 if not sol['consistent']:return []
 return sorted(sol['particular']^xor(v for i,v in enumerate(sol['basis'])if t>>i&1)for t in range(1<<len(sol['basis'])))
