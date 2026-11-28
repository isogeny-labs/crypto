"""Invertible linear differences plus linear constraints: total degree invariant."""
import sympy as sp

def equations(f,variables):
    """z_i=x_i-x_(i+1), z_last=x_last; no high-degree gate disappears."""
    zz=sp.symbols('z0:'+str(len(variables)))
    linear=[variables[i]-zz[i]-variables[i+1] for i in range(len(variables)-1)]
    linear.append(variables[-1]-zz[-1])
    replacement={variables[i]:sum(zz[j] for j in range(i,len(variables))) for i in range(len(variables))}
    return [sp.expand(f.xreplace(replacement))]+linear,zz
