"""Regenerate numerical map data from the public arXiv TeX source (optional).

Run with Sage Python, or ordinary Python with SymPy. The experiments themselves
only need tian_data.json, so no network is needed to reproduce a run.
"""
import hashlib
import json
import re
from pathlib import Path
from sympy import symbols, Poly
from sympy.parsing.sympy_parser import (parse_expr, standard_transformations,
                                       implicit_multiplication_application)

ROOT=Path(__file__).resolve().parent
text=(ROOT/'sources/tian.tex').read_text()
x,y,w=symbols('x y w')


def clean(expr):
    expr=expr.replace(r'\mathbf{p}(x)','den')
    expr=re.sub(r'\\(?:Bigl|Bigr|bigl|bigr)', '', expr)
    expr=expr.replace('\\','').replace('&','').replace('{','').replace('}','')
    expr=re.sub(r'\s+','',expr).rstrip(',.')
    expr=expr.replace('^','**')
    # The downloaded text is data: reject everything except this tiny grammar.
    if not re.fullmatch(r'[0-9xyw()+*/.\-]+',expr):
        raise ValueError('Unexpected mathematical token in source')
    return parse_expr(expr,local_dict={'x':x,'y':y,'w':w},
                      transformations=standard_transformations+(implicit_multiplication_application,))


def coefficients(expr):
    poly=Poly(expr,x)
    out=[]
    for i in range(poly.degree()+1):
        c=Poly(poly.nth(i),w)
        if c.degree()>2:
            raise ValueError('Coefficient not in the published cubic basis')
        out.append([int(c.nth(j)) for j in range(3)])
    return out


functions=[]
for i in range(1,7):
    pattern=r'\\mathbf\{C\}_'+str(i)+r'\(x,y\)&=(.*?)\\end\{align\*\}'
    body=re.search(pattern,text,re.S).group(1).split(r'/\mathbf{p}(x)')[0]
    expr=clean(body).expand()
    functions.append({'A':coefficients(expr.coeff(y,0)),
                      'B':coefficients(expr.coeff(y,1))})
denbody=re.search(r'\\mathbf\{p\}\(x\)&=(.*?)\\end\{align\*\}',text,re.S).group(1)
data={'source':'https://arxiv.org/abs/2012.07173',
      'source_section':'3.4, arXiv v1',
      'source_tex_sha256':hashlib.sha256(text.encode()).hexdigest(),
      'p':523,'modulus':[2,0,0,1], 'functions':functions,
      'denominator':coefficients(clean(denbody))}
(ROOT/'tian_data.json').write_text(json.dumps(data,indent=2)+'\n')
print('Extracted six rational functions and their common denominator.')
