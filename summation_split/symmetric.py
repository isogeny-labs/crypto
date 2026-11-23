"""Elementary symmetric variables; optional quadratic elementary-symmetric circuit."""
import sympy as sp

def terminal_s4(a,b):
    from .core import s4_symbolic
    f,xx=s4_symbolic(a,b)
    ff,rem,subs=sp.symmetrize(f,list(xx), formal=True)
    if sp.expand(rem)!=0:raise ArithmeticError('S4 not symmetric')
    return ff,subs,xx

def ties(xs):
    """e_j^(i)=e_j^(i-1)+x_i*e_(j-1)^(i-1), each constraint degree <=2."""
    n=len(xs)
    E={(0,0):sp.Integer(1)};eqs=[]
    for i in range(1,n+1):
        E[i,0]=sp.Integer(1)
        for j in range(1,i+1):
            u=sp.Symbol(f'e{i}_{j}');E[i,j]=u
            eqs.append(u-E.get((i-1,j),sp.Integer(0))-xs[i-1]*E[i-1,j-1])
    return eqs,[E[n,j] for j in range(1,n+1)]

def s4_system(a,b):
    f,subs,xx=terminal_s4(a,b)
    tie,een=ties(xx)
    symbolmap={si:een[i] for i, (si,_) in enumerate(subs)}
    return tie+[sp.expand(f.xreplace(symbolmap))],xx
