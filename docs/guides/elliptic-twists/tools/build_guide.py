"""Create an illustrated, numerically checked guide to elliptic-curve twists.

All diagrams are original vector artwork.  PDF figures are embedded as vector
forms; separate PNG and SVG files are exported for reuse.
"""

from pathlib import Path
from math import pi, cos, sin
import os
import json
import zipfile
from tempfile import gettempdir

os.environ.setdefault('MPLCONFIGDIR',str(Path(gettempdir()) / 'twist-guide-matplotlib'))
os.environ.setdefault('XDG_CACHE_HOME',str(Path(gettempdir()) / 'twist-guide-cache'))
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from matplotlib.patches import FancyBboxPatch, FancyArrowPatch, Circle, Polygon
import numpy as np

from reportlab.pdfgen import canvas
from reportlab.lib.colors import HexColor
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.platypus import Paragraph
from reportlab.lib.styles import ParagraphStyle
from pypdf import PdfReader, PdfWriter, Transformation


OUT = Path(__file__).resolve().parents[1]
DIAG = OUT / 'diagrams'
BUILD = OUT / '.build'
PREVIEW = BUILD / 'previews'
for path in (OUT, DIAG, BUILD, PREVIEW):
    path.mkdir(parents=True, exist_ok=True)

C = dict(ink='#172E40', blue='#2B63A0', orange='#BF6428', purple='#7652A2',
         teal='#087C83', muted='#556775', line='#D7DFE4', bg='#FAF8F3',
         pale_blue='#EBF1F8', pale_orange='#FAEEE4', pale_purple='#F0EBF7',
         pale_teal='#E7F3F1', white='#FFFFFF')

plt.rcParams.update({
    'font.family': 'DejaVu Sans', 'font.size': 15, 'text.color': C['ink'],
    'axes.labelcolor': C['ink'], 'mathtext.fontset': 'dejavusans',
    'svg.fonttype': 'path', 'pdf.fonttype': 42,
    'figure.facecolor': C['bg'], 'savefig.facecolor': C['bg'],
})

MANIFEST = []
OVERLAYS = []


def figbase():
    fig, ax = plt.subplots(figsize=(10, 6))
    fig.subplots_adjust(0.015, 0.025, 0.985, 0.975)
    ax.set_xlim(0, 10)
    ax.set_ylim(0, 6)
    ax.axis('off')
    return fig, ax


def txt(ax, x, y, s, size=16, color='ink', ha='center', weight='normal', **kw):
    return ax.text(x, y, s, fontsize=size, color=C.get(color, color),
                   ha=ha, va='center', weight=weight, linespacing=1.45, **kw)


def box(ax, x, y, w, h, fill='white', edge='line', lw=1.25, radius=.16):
    p = FancyBboxPatch((x, y), w, h,
                       boxstyle=f'round,pad=0.02,rounding_size={radius}',
                       facecolor=C.get(fill, fill), edgecolor=C.get(edge, edge),
                       linewidth=lw)
    ax.add_patch(p)
    return p


def arrow(ax, x1, y1, x2, y2, color='muted', lw=2, bend=0, dashed=False, ms=18):
    p = FancyArrowPatch((x1,y1),(x2,y2), arrowstyle='-|>', mutation_scale=ms,
                        linewidth=lw, color=C.get(color,color),
                        connectionstyle=f'arc3,rad={bend}',
                        linestyle=(0,(4,3)) if dashed else 'solid')
    ax.add_patch(p)
    return p


def save(fig, name, title, description):
    for ext in ('png','svg','pdf'):
        destination = (BUILD if ext == 'pdf' else DIAG) / f'{name}.{ext}'
        fig.savefig(destination, dpi=210)
    plt.close(fig)
    MANIFEST.append(dict(name=name, title=title, description=description))


# All finite-field arithmetic lives in the native verifier. Python reads its
# checked fixture and handles only typography, geometry and document layout.
FIXTURES = json.loads((OUT / 'tools' / 'verified_examples.json').read_text())
SEXTIC = FIXTURES['sextic_F7']


def grid(ax, x0,y0,w,h,p=7,a=0,b=1,color='blue', dotsize=55):
    if (p, a, b) not in ((7, 0, 1), (7, 0, 6)):
        raise ValueError('No native-generated affine fixture for this grid')
    for x in range(p):
        for y in range(p):
            ax.plot(x0+w*x/(p-1), y0+h*y/(p-1), '.',color=C['line'],ms=5,zorder=1)
    pts=FIXTURES['affine_F7'][str(b)]
    ax.scatter([x0+w*x/(p-1) for x,y in pts],
               [y0+h*y/(p-1) for x,y in pts],s=dotsize,
               color=C[color],edgecolor=C['bg'],linewidth=.8,zorder=3)


def cover_diagram():
    fig,ax=figbase()
    box(ax,.3,1.25,3.2,3.9,'pale_blue',edge='blue')
    box(ax,6.5,1.25,3.2,3.9,'pale_orange',edge='orange')
    txt(ax,1.9,4.73,'CURVE',16,'blue',weight='bold')
    txt(ax,8.1,4.73,'TWIST',16,'orange',weight='bold')
    grid(ax,.9,2.0,2.0,2.0,b=1,dotsize=55)
    grid(ax,7.1,2.0,2.0,2.0,b=6,color='orange',dotsize=55)
    txt(ax,1.9,1.65,'12 points over '+r'$\mathbb{F}_7$',15,'blue')
    txt(ax,8.1,1.65,'4 points over '+r'$\mathbb{F}_7$',15,'orange')
    arrow(ax,3.75,3.45,6.25,3.45,'teal',lw=3)
    txt(ax,5,4.1,'A hidden\ncoordinate change',16,'teal',weight='bold')
    txt(ax,5,2.65,r'needs $\sqrt{3}$',18,'teal')
    txt(ax,5,.55,'Add the missing root: both have 48 points over '+r'$\mathbb{F}_{49}$.',16)
    save(fig,'01_hidden_bridge','A hidden bridge',
         'Actual affine points over F7 for y^2=x^3+1 and y^2=x^3+6. The identity O is included in the stated counts. They become isomorphic over F49.')


def curve_points_diagram():
    fig,ax=figbase()
    x0,y0,w,h=.95,1.1,4.2,4.2
    grid(ax,x0,y0,w,h,b=1,dotsize=100)
    for i in range(7):
        txt(ax,x0+w*i/6,y0-.32,str(i),13,'muted')
        txt(ax,x0-.32,y0+h*i/6,str(i),13,'muted')
    txt(ax,3.05,.28,r'$x$',17)
    txt(ax,.25,3.2,r'$y$',17)
    box(ax,5.8,3.15,3.7,2.2,'white')
    txt(ax,6.1,4.9,'ONE VALID POINT',14,'blue',ha='left',weight='bold')
    txt(ax,7.65,4.3,r'$(x,y)=(1,3)$',20,'blue')
    txt(ax,7.65,3.7,r'$3^2=9\equiv 2\;(\mathrm{mod}\;7)$',17)
    box(ax,5.8,.95,3.7,1.8,'pale_blue',edge='blue')
    txt(ax,7.65,2.24,'11 plotted points',19,'blue',weight='bold')
    txt(ax,7.65,1.65,r'$+\ O\ =\ 12$ in the group',18)
    txt(ax,3.0,5.8,r'$y^2=x^3+1\quad\mathrm{over}\ \mathbb{F}_7$',20)
    save(fig,'02_finite_curve_points','A finite-field curve is a set of points',
         'A complete F7 grid for y^2=x^3+1, including an example modular check and the identity point in the total.')


def field_toolbox_diagram():
    fig,ax=figbase()
    box(ax,.25,2.9,4.15,2.6,'pale_blue',edge='blue')
    txt(ax,2.325,5.05,r'BASE FIELD $\mathbb{F}_7$',17,'blue',weight='bold')
    txt(ax,2.325,4.4,r'$0,1,2,3,4,5,6$',22)
    txt(ax,2.325,3.73,'Squares: 0, 1, 2, 4',18)
    txt(ax,2.325,3.22,r'No $s$ here has $s^2=3$.',16,'orange')
    box(ax,5.6,2.9,4.15,2.6,'pale_teal',edge='teal')
    txt(ax,7.675,5.05,r'EXTENSION $\mathbb{F}_{49}$',17,'teal',weight='bold')
    txt(ax,7.675,4.35,r'$a+bu,\quad u^2=3$',23,'teal')
    txt(ax,7.675,3.7,r'$a,b\in\mathbb{F}_7$',19)
    txt(ax,7.675,3.2,r'$7\times7=49$ elements',17)
    arrow(ax,4.6,4.25,5.4,4.25,'teal',lw=2.7)
    txt(ax,5,1.88,'We add an algebraic tool, not extra decimal precision.',18,weight='bold')
    txt(ax,5,.97,r'$u^2=3$ makes new coordinate changes possible.',20,'teal')
    txt(ax,5,.34,r'$\mathbb{F}_{49}\neq\mathbb{Z}/49\mathbb{Z}$',16,'muted')
    save(fig,'03_field_toolbox','Extending the coordinate toolbox',
         'The field F49 is obtained by adjoining a square root of 3 to F7. It is a field of pairs, not integers modulo 49.')


def hidden_map_diagram():
    fig,ax=figbase()
    box(ax,.25,2.35,3.55,3.12,'pale_blue',edge='blue')
    box(ax,6.2,2.35,3.55,3.12,'pale_orange',edge='orange')
    txt(ax,2.025,4.95,r'$E:\ y^2=x^3+1$',21,'blue')
    txt(ax,7.975,4.95,r'$T:\ Y^2=X^3+6$',21,'orange')
    txt(ax,2.025,3.95,r'$P=(0,1)$',24,'blue')
    txt(ax,7.975,3.95,r'$\phi(P)=(0,3u)$',23,'orange')
    txt(ax,2.025,2.95,r'$P\in E(\mathbb{F}_7)$',18)
    txt(ax,7.975,2.95,r'$\phi(P)\in T(\mathbb{F}_{49})$',17)
    arrow(ax,3.95,3.85,6.05,3.85,'teal',lw=3)
    txt(ax,5,4.55,r'$\phi$',22,'teal')
    txt(ax,5,1.65,r'$\phi(x,y)=(3x,\,3uy),\qquad u^2=3$',24,'teal')
    txt(ax,5,.82,r'$(3uy)^2=6y^2=(3x)^3+6\quad\mathrm{in\ characteristic}\ 7$',16)
    txt(ax,5,.23,'The map exists over the extension; it need not preserve base-field coordinates.',14,'muted')
    save(fig,'04_explicit_hidden_map','A point crossing the hidden bridge',
         'An explicit isomorphism from y^2=x^3+1 to y^2=x^3+6 over F49. A base-field point can map to a point with extension-field coordinates.')


def quadratic_flip_diagram():
    fig,ax=figbase()
    txt(ax,3.65,5.63,r'$E:\ y^2=f(x)$',20,'blue')
    txt(ax,7.0,5.63,r'$T:\ 3v^2=f(x)$',20,'orange')
    txt(ax,.9,4.98,r'$x$',16,weight='bold')
    txt(ax,2,4.98,r'$f(x)$',16,weight='bold')
    txt(ax,4.1,4.98,'number of y values',14,'blue')
    txt(ax,7.0,4.98,'number of v values',14,'orange')
    for row in FIXTURES['quadratic_fibers_F7']:
        x, f, ny, nv = (row[key] for key in ('x', 'rhs', 'ny', 'nv'))
        y=4.44-x*.47
        if x%2==0: box(ax,.45,y-.205,8.65,.41,'white',edge='white',radius=.06)
        txt(ax,.9,y,str(x),15)
        txt(ax,2,y,str(f),15)
        for k in range(ny): ax.add_patch(Circle((3.9+(k-(ny-1)/2)*.35,y),.09,color=C['blue']))
        for k in range(nv): ax.add_patch(Circle((6.8+(k-(nv-1)/2)*.35,y),.09,color=C['orange']))
        if ny==0: txt(ax,3.9,y,'0',14,'muted')
        if nv==0: txt(ax,6.8,y,'0',14,'muted')
        txt(ax,8.7,y,'= 2',14,'muted')
    txt(ax,5,.62,r'$11+1=12\qquad+\qquad3+1=4\qquad=16$',22)
    txt(ax,5,.12,'Each group also has its own identity point O.',14,'muted')
    save(fig,'05_quadratic_square_flip','Quadratic twisting flips square choices',
         'The complete fiber-by-fiber counts for y^2=x^3+1 and 3v^2=x^3+1 over F7. Each x contributes two points across the two models.')


def scaling_tree_diagram():
    fig,ax=figbase()
    box(ax,1.05,4.48,7.9,1.12,'pale_teal',edge='teal')
    txt(ax,5,5.14,r'$(x,y)\mapsto(u^2x,u^3y)$',25,'teal')
    txt(ax,5,4.75,r'$A\mapsto u^4A\qquad B\mapsto u^6B$',17)
    positions=(1.65,5,8.35)
    labels=(('A and B both nonzero',r'$u^4,u^6\in K$',r'$u^2\in K$','QUADRATIC','blue'),
            ('B = 0',r'$u^4\in K$',r'$u^4=c$','QUARTIC','orange'),
            ('A = 0',r'$u^6\in K$',r'$u^6=c$','SEXTIC','teal'))
    for x,(title,cond,root,name,color) in zip(positions,labels):
        arrow(ax,5,4.32,x,3.82,color,lw=1.9)
        box(ax,x-1.48,1.48,2.96,2.35,'white',edge=color)
        txt(ax,x,3.4,title,14,weight='bold')
        txt(ax,x,2.78,cond,21,color)
        txt(ax,x,2.2,root,19)
        txt(ax,x,1.73,name,14,color,weight='bold')
    txt(ax,8.25,.71,r'$u^3=a\ \Rightarrow\ u^6=a^2$',18,'purple')
    txt(ax,8.25,.2,'Cubic twists live inside this family.',14,'purple')
    txt(ax,2.35,.65,'The coefficient powers explain\nthe available root degrees.',16,'muted')
    save(fig,'06_coordinate_weight_tree','Why 2, 3, 4 and 6 appear',
         'The Weierstrass coordinate weights force the generic square-root case and permit fourth or sixth roots when a coefficient vanishes. Cubic twists are a subfamily of sextic twists.')


def symmetry_diagram():
    fig,ax=figbase()
    for cx,n,col,label,formula in (
        (1.65,2,'blue',r'$j\neq0,1728$',r'$(x,y)\mapsto(x,-y)$'),
        (5,4,'orange',r'$j=1728$',r'$(x,y)\mapsto(-x,iy)$'),
        (8.35,6,'teal',r'$j=0$',r'$(x,y)\mapsto(\omega x,-y)$')):
        theta=np.linspace(pi/2,pi/2+2*pi,n,endpoint=False)
        pts=np.c_[cx+1.02*np.cos(theta),3.48+1.02*np.sin(theta)]
        if n>2: ax.add_patch(Polygon(pts,closed=True,facecolor=C['white'],edgecolor=C[col],lw=2))
        else: ax.plot(pts[:,0],pts[:,1],color=C[col],lw=2)
        for x,y in pts: ax.add_patch(Circle((x,y),.09,color=C[col]))
        txt(ax,cx,3.48,str(n),30,col,weight='bold',
            bbox=dict(facecolor=C['white'],edgecolor='none',pad=2))
        txt(ax,cx,5.1,label,20,col)
        txt(ax,cx,1.87,formula,16)
        txt(ax,cx,1.25,f'{n} geometric symmetries',14,col,weight='bold')
    txt(ax,5,.57,r'$i^2=-1,\quad\omega^3=1,\quad\omega\neq1$',18)
    txt(ax,5,.13,'Polygons show symmetry groups, not the shape or points of the elliptic curve.',13,'muted')
    save(fig,'07_symmetry_groups','More symmetry permits more twists',
         'Symbolic diagrams of the origin-preserving geometric automorphism groups C2, C4 and C6. These polygons are not drawings of the curves.')


def cubic_diagram():
    fig,ax=figbase()
    nodes=[(1.7,2.55,1,12),(5,4.55,2,9),(8.3,2.55,4,3)]
    arrow(ax,2.9,3.25,3.8,3.9,'purple',lw=2.5)
    arrow(ax,6.2,3.9,7.1,3.25,'purple',lw=2.5)
    arrow(ax,6.85,1.83,3.15,1.83,'purple',lw=2.5,bend=-.12)
    for x,y,b,n in nodes:
        box(ax,x-1.4,y-.95,2.8,1.9,'pale_purple',edge='purple')
        txt(ax,x,y+.48,r'$y^2=x^3+'+str(b)+'$',20,'purple')
        txt(ax,x,y-.12,f'{n} points',22,weight='bold')
        txt(ax,x,y-.6,'over '+r'$\mathbb{F}_7$',14,'muted')
    txt(ax,5,.9,r'$B\mapsto a^2B,\qquad v^3=a$',24,'purple')
    txt(ax,5,.27,r'All three become isomorphic over $\mathbb{F}_{7^3}$.',17)
    save(fig,'08_cubic_family','The cubic-twist triangle',
         'The three cubic twists of y^2=x^3+1 over F7 have coefficients 1, 2 and 4, and point counts 12, 9 and 3. Arrows represent multiplication of B by 2, a square, with a cube-root coordinate change.')


def quartic_diagram():
    fig,ax=figbase()
    xs=[1.3,3.77,6.23,8.7]
    vals=[(1,20,1),(2,10,4),(4,8,2),(8,18,4)]
    for x,(a,n,d) in zip(xs,vals):
        box(ax,x-1.1,2.0,2.2,2.65,'pale_orange',edge='orange')
        txt(ax,x,4.17,r'$A='+str(a)+'$',23,'orange')
        txt(ax,x,3.51,str(n),30,weight='bold')
        txt(ax,x,3.04,'points',15)
        txt(ax,x,2.43,f'degree {d}',16,'orange')
    for i in range(3): arrow(ax,xs[i]+1.13,3.35,xs[i+1]-1.13,3.35,'orange',ms=14)
    txt(ax,5,5.42,r'$y^2=x^3+Ax\quad\mathrm{over}\ \mathbb{F}_{13}$',23)
    txt(ax,5,1.18,r'$A\mapsto 2A\qquad u^4=2$',23,'orange')
    txt(ax,5,.44,'Four forms: original, two exact quartic twists, one quadratic twist.',15)
    save(fig,'09_quartic_family','The quartic-twist family',
         'The four quartic twist classes over F13 of y^2=x^3+x, with coefficients 1,2,4,8 and exact extension degrees 1,4,2,4.')


def sextic_wheel_diagram():
    fig,ax=figbase()
    # Arrange all six models on a wheel; labels describe the degree relative to i=0.
    coords=[]
    for i in range(6):
        th=pi/2-i*pi/3
        coords.append((5+3.05*cos(th),3.12+1.98*sin(th)))
    colors=['blue','teal','purple','orange','purple','teal']
    for i in range(6):
        x1,y1=coords[i];x2,y2=coords[(i+1)%6]
        dx=x2-x1;dy=y2-y1;norm=(dx*dx+dy*dy)**.5
        arrow(ax,x1+dx*.29,y1+dy*.29,x2-dx*.29,y2-dy*.29,'line',lw=2,ms=13)
    # Show the original's cubic subfamily and its quadratic partner.
    for a,b in ((0,2),(2,4),(4,0)):
        x1,y1=coords[a];x2,y2=coords[b]
        ax.plot([x1,x2],[y1,y2],color=C['purple'],alpha=.25,lw=2,ls='--',zorder=0)
    x1,y1=coords[0];x2,y2=coords[3]
    ax.plot([x1,x2],[y1,y2],color=C['orange'],alpha=.4,lw=2,ls=':',zorder=0)
    for i,row in enumerate(SEXTIC):
        x,y=coords[i];col=colors[i]
        box(ax,x-1.18,y-.68,2.36,1.36,'white',edge=col,lw=1.8)
        txt(ax,x,y+.38,r'$B='+str(row['b'])+'$',18,col,weight='bold')
        txt(ax,x,y-.05,f"{row['count']} points",19)
        txt(ax,x,y-.43,f"degree {row['degree']}",13,col)
    box(ax,3.63,2.54,2.74,1.03,'bg',edge='bg')
    txt(ax,5,3.17,r'$y^2=x^3+B$',21)
    txt(ax,5,2.74,r'same $j=0$ over $\mathbb{F}_7$',14,'muted')
    txt(ax,5,.15,'Blue: original   Orange: quadratic   Purple: cubic   Teal: exact sextic',12.8)
    save(fig,'10_sextic_family_wheel','All six sextic-twist classes',
         'Six F7-isomorphism classes with j=0. Each node gives B, the actual point count, and the minimal isomorphism-extension degree relative to B=1. Dashed purple lines identify its cubic subfamily.')


def field_dependence_diagram():
    fig,ax=figbase()
    box(ax,.25,1.0,4.45,4.5,'pale_teal',edge='teal')
    box(ax,5.3,1.0,4.45,4.5,'pale_blue',edge='blue')
    txt(ax,2.475,5.05,r'Over $\mathbb{F}_7$',23,'teal')
    txt(ax,7.525,5.05,r'Over $\mathbb{F}_5$',23,'blue')
    txt(ax,2.475,4.26,r'$c^6=1$ for every $c\neq0$',17)
    txt(ax,7.525,4.26,r'$c^6\in\{1,4\}$',20)
    for i in range(6):
        x=1.1+(i%3)*1.38;y=3.3-(i//3)*.95
        box(ax,x-.45,y-.31,.9,.62,'white',edge='teal',radius=.1)
        txt(ax,x,y,str(i+1),17,'teal')
    for j,(label,values) in enumerate((('class 1','1, 4'),('class 2','2, 3'))):
        x=6.5+j*2.05
        box(ax,x-.85,2.2,1.7,1.45,'white',edge='blue')
        txt(ax,x,3.22,label,14,'blue')
        txt(ax,x,2.65,values,21)
    txt(ax,2.475,1.5,'6 sextic classes',22,'teal',weight='bold')
    txt(ax,7.525,1.5,'2 sextic classes',22,'blue',weight='bold')
    txt(ax,5,.4,r'$\#\mathrm{classes}=\gcd(6,q-1)$',23)
    save(fig,'11_field_changes_class_count','The field changes the number of twists',
         'For j=0 and characteristic greater than 3, the sextic classes are multiplicative cosets modulo sixth powers. This gives six classes over F7 and two over F5.')


def frobenius_diagram():
    fig,ax=figbase()
    box(ax,.25,1.4,4.45,4.15,'pale_blue',edge='blue')
    box(ax,5.3,1.4,4.45,4.15,'pale_orange',edge='orange')
    txt(ax,2.475,5.08,'FIXED',18,'blue',weight='bold')
    txt(ax,7.525,5.08,'SWAPPED WITH ITS NEGATIVE',13.5,'orange',weight='bold')
    ax.add_patch(Circle((2.475,3.4),.16,color=C['blue']))
    arrow(ax,2.23,3.55,2.72,3.55,'blue',lw=2.5,bend=-2.4,ms=18)
    txt(ax,2.475,2.52,r'$\pi(P)=P$',23,'blue')
    txt(ax,2.475,1.88,r'Coordinates lie in $\mathbb{F}_q$.',15)
    for x in (6.5,8.55): ax.add_patch(Circle((x,3.45),.13,color=C['orange']))
    txt(ax,6.5,4.0,r'$P$',20,'orange')
    txt(ax,8.55,4.0,r'$-P$',20,'orange')
    arrow(ax,6.7,3.7,8.35,3.7,'orange',bend=-.3)
    arrow(ax,8.35,3.2,6.7,3.2,'orange',bend=-.3)
    txt(ax,7.525,2.52,r'$\pi(P)=-P$',23,'orange')
    txt(ax,7.525,1.88,r'$\pi^2(P)=P$',19)
    txt(ax,5,.74,r'$\pi(x,y)=(x^q,y^q)$',24)
    txt(ax,5,.15,'Under the twist isomorphism, twist-rational points satisfy the right-hand condition.',13,'muted')
    save(fig,'12_frobenius_visibility','Frobenius is the visibility test',
         'Base-field rational points are fixed by Frobenius. Under a quadratic untwist, rational points on the twist correspond to anti-fixed points pi(P)=-P, which are fixed by pi squared. Two-torsion can satisfy both conditions.')


def security_diagram():
    fig,ax=figbase()
    box(ax,3.55,4.8,2.9,.8,'white',edge='ink')
    txt(ax,5,5.2,'Input: one coordinate u',15,weight='bold')
    arrow(ax,4.25,4.65,2.7,3.9,'blue')
    arrow(ax,5.75,4.65,7.3,3.9,'orange')
    box(ax,.55,1.85,4.0,2.1,'pale_blue',edge='blue')
    box(ax,5.45,1.85,4.0,2.1,'pale_orange',edge='orange')
    txt(ax,2.55,3.5,'CURVE',18,'blue',weight='bold')
    txt(ax,7.45,3.5,'QUADRATIC TWIST',17,'orange',weight='bold')
    txt(ax,2.55,2.83,r'$N_E=h\,r$',24,'blue')
    txt(ax,7.45,2.83,r'$N_T=h^{\prime}r^{\prime}$',24,'orange')
    txt(ax,2.55,2.25,'large prime r',16)
    txt(ax,7.45,2.25,r'large prime $r^{\prime}$',16)
    txt(ax,2.5,4.25,'square right-hand side',13,'blue')
    txt(ax,7.5,4.25,'nonsquare right-hand side',13,'orange')
    box(ax,1.35,.35,7.3,.9,'pale_teal',edge='teal')
    txt(ax,5,.8,'Curve25519: curve cofactor 8; twist cofactor 4.',16,'teal',weight='bold')
    save(fig,'13_twist_security','Why the quadratic-twist order matters',
         'A Montgomery single-coordinate ladder can process inputs associated with either the curve or its quadratic twist. Each group needs the order properties required by the protocol. A nonzero right-hand side is assumed in the branch labels.')


def pairing_diagram():
    fig,ax=figbase()
    # Field degree ladder. The highlighted map concerns the selected G2 subgroup.
    box(ax,.25,3.0,2.35,2.32,'pale_blue',edge='blue')
    box(ax,3.55,3.0,2.85,2.32,'pale_purple',edge='purple')
    box(ax,7.35,3.0,2.4,2.32,'pale_teal',edge='teal')
    txt(ax,1.425,4.83,r'$\mathbb{F}_p$',25,'blue')
    txt(ax,4.975,4.83,r'$\mathbb{F}_{p^2}$',25,'purple')
    txt(ax,8.55,4.83,r'$\mathbb{F}_{p^{12}}$',25,'teal')
    txt(ax,1.425,3.94,r'$G_1\subset E(\mathbb{F}_p)$',15)
    txt(ax,4.975,4.0,'store G2 points',15,weight='bold')
    txt(ax,4.975,3.48,r'on $E^{\prime}(\mathbb{F}_{p^2})$',17,'purple')
    txt(ax,8.55,3.96,'pairing values',15,weight='bold')
    txt(ax,8.55,3.43,r'$\mu_r\subset\mathbb{F}_{p^{12}}^*$',15,'teal')
    arrow(ax,2.72,4.35,3.4,4.35,'muted',ms=17)
    arrow(ax,6.5,4.35,7.2,4.35,'teal',ms=17)
    txt(ax,3.02,4.83,r'$\times2$',15)
    txt(ax,6.86,4.83,r'$\times6$',15,'teal')
    txt(ax,5,2.35,r'$E:y^2=x^3+4\qquad E^{\prime}:y^2=x^3+4(1+i)$',19)
    txt(ax,5,1.56,r'$w^6=1+i,\qquad \phi(x,y)=(x/w^2,y/w^3)$',22,'teal')
    txt(ax,5,.77,'The untwist maps the stored G2 subgroup into '+r'$E(\mathbb{F}_{p^{12}})$.',14)
    txt(ax,5,.2,'Embedding degree = 12 over Fp.  Twist degree = 6 over Fp².',14,weight='bold')
    save(fig,'14_pairing_field_tower','A sextic twist makes pairing points cheaper',
         'BLS12-381 represents G2 on its M-type sextic twist over Fp2. The untwist embeds the selected subgroup in E(Fp12). Pairing values live in a multiplicative subgroup of Fp12, a different object from elliptic-curve points.')


def make_diagrams():
    for f in (cover_diagram, curve_points_diagram, field_toolbox_diagram,
              hidden_map_diagram, quadratic_flip_diagram, scaling_tree_diagram,
              symmetry_diagram, cubic_diagram, quartic_diagram,
              sextic_wheel_diagram, field_dependence_diagram,
              frobenius_diagram, security_diagram, pairing_diagram):
        f()
    (DIAG/'diagram_index.json').write_text(json.dumps(MANIFEST,indent=2))


from matplotlib.font_manager import FontProperties, findfont
for name, properties in [
    ('Guide', FontProperties(family='DejaVu Sans')),
    ('Guide-Bold', FontProperties(family='DejaVu Sans', weight='bold')),
    ('Guide-Oblique', FontProperties(family='DejaVu Sans', style='oblique')),
    ('Guide-Serif', FontProperties(family='DejaVu Serif')),
]:
    pdfmetrics.registerFont(TTFont(name, findfont(properties)))
pdfmetrics.registerFontFamily('Guide',normal='Guide',bold='Guide-Bold',italic='Guide-Oblique')

W,H=612,792
M=44
STYLES={
    'body':ParagraphStyle('body',fontName='Guide',fontSize=11.1,leading=16.6,textColor=HexColor(C['ink'])),
    'small':ParagraphStyle('small',fontName='Guide',fontSize=9.3,leading=13.5,textColor=HexColor(C['muted'])),
    'deck':ParagraphStyle('deck',fontName='Guide',fontSize=11.5,leading=17,textColor=HexColor(C['muted'])),
    'take':ParagraphStyle('take',fontName='Guide',fontSize=11.4,leading=16.3,textColor=HexColor(C['ink'])),
    'refs':ParagraphStyle('refs',fontName='Guide',fontSize=10.3,leading=15,textColor=HexColor(C['ink'])),
}


def para(c,text,x,y,width,style='body',max_height=None):
    p=Paragraph(text,STYLES[style])
    _,height=p.wrap(width,1000)
    if max_height is not None and height>max_height+.1:
        raise AssertionError(f'Text overflow: {height:.1f} > {max_height}: {text[:100]}')
    p.drawOn(c,x,y-height)
    return y-height


def line(c,x1,y1,x2,y2,color='line',width=1):
    c.setStrokeColor(HexColor(C[color]));c.setLineWidth(width);c.line(x1,y1,x2,y2)


def rr(c,x,y,w,h,fill='white',edge=None,r=9):
    c.setFillColor(HexColor(C[fill]))
    c.setStrokeColor(HexColor(C[edge or fill]))
    c.roundRect(x,y,w,h,r,fill=1,stroke=bool(edge))


def page_start(c,n,kicker,title,deck):
    c.setFillColor(HexColor(C['bg']));c.rect(0,0,W,H,fill=1,stroke=0)
    c.setFillColor(HexColor(C['muted']));c.setFont('Guide-Bold',8.4)
    c.drawString(M,754,'ELLIPTIC CURVE TWISTS')
    c.drawRightString(W-M,754,kicker.upper())
    line(c,M,739,W-M,739)
    title_size=min(25,25*(W-2*M)/pdfmetrics.stringWidth(title,'Guide-Bold',25))
    c.setFillColor(HexColor(C['ink']));c.setFont('Guide-Bold',title_size)
    c.drawString(M,702,title)
    para(c,deck,M,680,W-2*M,'deck',max_height=51)
    c.setFont('Guide',8.4);c.setFillColor(HexColor(C['muted']))
    c.drawString(M,27,'An illustrated guide • finite fields and cryptography')
    c.drawRightString(W-M,27,f'{n:02d} / 16')
    c.bookmarkPage(f'page_{n}')
    c.addOutlineEntry(title,f'page_{n}',0,False)


def place_figure(c,name,x=M,y=275,w=W-2*M,h=340):
    page=PdfReader(str(BUILD/f'{name}.pdf')).pages[0]
    bw,bh=float(page.mediabox.width),float(page.mediabox.height)
    scale=min(w/bw,h/bh)
    OVERLAYS.append((c._pageNumber-1,name,scale,
                     x+(w-bw*scale)/2,y+(h-bh*scale)/2))


def takeaway(c,text,label='REMEMBER',color='teal'):
    fill={'teal':'pale_teal','orange':'pale_orange','purple':'pale_purple','blue':'pale_blue'}[color]
    rr(c,M,61,W-2*M,72,fill)
    c.setFillColor(HexColor(C[color]));c.setFont('Guide-Bold',8.5)
    c.drawString(M+14,115,label)
    para(c,text,M+14,103,W-2*M-28,'take',max_height=35)


def two_cols(c,left,right,y=254):
    gutter=23;cw=(W-2*M-gutter)/2
    para(c,left,M,y,cw,max_height=105)
    para(c,right,M+cw+gutter,y,cw,max_height=105)


def standard(c,n,kicker,title,deck,figure,left,right,take,color='teal',note=None):
    page_start(c,n,kicker,title,deck)
    place_figure(c,figure)
    two_cols(c,left,right)
    takeaway(c,take,color=color)
    if note: para(c,note,M,49,W-2*M,'small',max_height=14)
    c.showPage()


def create_pdf():
    path=OUT/'Elliptic_Curve_Twists_Visual_Guide.pdf'
    c=canvas.Canvas(str(path),pagesize=(W,H),pageCompression=1)
    c.setTitle('Elliptic Curve Twists — A Visual Guide')
    c.setAuthor('OpenAI')
    c.setSubject('An intuitive illustrated guide to quadratic, cubic, quartic and sextic twists over finite fields, with cryptographic applications')

    page_start(c,1,'Start here','Same curve. Different visible points.',
        'A visual introduction to quadratic, cubic, quartic and sextic twists — with finite-field examples and cryptographic intuition.')
    place_figure(c,'01_hidden_bridge',y=268,h=360)
    two_cols(c,
        '<b>The big idea.</b> A twist can look different when coordinates must come from a small field. After adding a missing root, an exact coordinate map reveals the same elliptic curve.',
        '<b>How to read this guide.</b> Follow the pictures first. Then read the equations as instructions for the hidden map. All plotted finite-field points and point counts are exact.',y=246)
    takeaway(c,'A twist changes how a curve is defined over your field, while preserving its geometry over an algebraic closure.')
    c.showPage()

    standard(c,2,'1 / Meet the objects','A curve is a rule for allowed points.',
        'Over a finite field, an elliptic curve gives a finite set of coordinate pairs, together with a group law and an identity point O.',
        '02_finite_curve_points',
        '<b>Read the grid.</b> The axes contain the residues 0 through 6. A blue dot marks a pair satisfying y² = x³ + 1 modulo 7. The pale dots are rejected pairs.',
        '<b>Check one.</b> At (1,3), the two sides are 9 and 2. They agree modulo 7. The point O is the identity of the group and is not shown in this affine grid.',
        'The point count includes O. The group law adds points on the curve; it is not ordinary coordinate-wise addition.',color='blue')

    standard(c,3,'1 / Fields','Your field is your coordinate toolbox.',
        'A field tells you which numbers may appear in a coordinate or in the formula for a change of coordinates.',
        '03_field_toolbox',
        '<b>A missing tool.</b> In F₇ the square residues are 0, 1, 2 and 4. There is no square root of 3. A map needing that root cannot be written with F₇ coefficients.',
        '<b>Add the tool.</b> Introduce u with u² = 3. The numbers a + bu form F₄₉. The original field sits inside it as the elements with b = 0.',
        'An extension adds algebraic numbers. F₄₉ is a field of 49 elements, not arithmetic modulo the integer 49.')

    standard(c,4,'2 / The hidden map','What a twist actually does.',
        'These two equations describe isomorphic elliptic curves once the coordinate field contains u, a square root of 3.',
        '04_explicit_hidden_map',
        '<b>Follow a point.</b> The point (0,1) on E maps to (0,3u) on T. That image is valid over F₄₉, but its y-coordinate is outside F₇.',
        '<b>Why counts differ.</b> An isomorphism over the extension need not send the F₇-points of one model to the F₇-points of the other. Over F₄₉ both groups have 48 points.',
        'Two curves are twists when they become isomorphic over an extension. A twist is trivial if the isomorphism already exists over the base field.')

    standard(c,5,'2 / Quadratic twists','Flip the square / nonsquare choice.',
        'Keep f(x) = x³ + 1. Compare y² = f(x) with 3v² = f(x) over F₇, where 3 is a nonsquare.',
        '05_quadratic_square_flip',
        '<b>Each x balances.</b> A nonzero square gives 2 points on the first model and 0 on the twist; a nonsquare gives 0 and 2. A zero gives 1 on each.',
        '<b>The point-count rule.</b> Add the two identity points: #E + #T = 2(q + 1). Here 12 + 4 = 16. This twist model is F₇-isomorphic to Y² = X³ + 6.',
        'For a nonsquare quadratic twist: if #E = q + 1 − t, then #T = q + 1 + t.',color='orange')

    standard(c,6,'3 / Where the degrees come from','The powers 2 and 3 do the work.',
        'Start with y² = x³ + Ax + B. Scaling x by u² and y by u³ keeps the Weierstrass shape and scales A and B.',
        '06_coordinate_weight_tree',
        '<b>Generic curves.</b> If A and B are nonzero, both u⁴ and u⁶ must be in the field. Their ratio is u², so a square root is all the freedom available.',
        '<b>Special curves.</b> When a coefficient vanishes, one condition disappears. Fourth roots or sixth roots can then define new forms. The cubic case sits inside the sextic family.',
        'The degree describes the field needed for an isomorphism. The isomorphism is a one-to-one coordinate change, not a map of that degree.')

    standard(c,7,'3 / Why twists exist','Symmetry gives a map room to hide.',
        'For characteristic greater than 3, these are the geometric symmetries that preserve the identity O. “Geometric” allows algebraic-closure coefficients.',
        '07_symmetry_groups',
        '<b>Why symmetry matters.</b> Apply a field symmetry to the hidden map: you may get another map between the same curves. The discrepancy is a symmetry of the curve itself.',
        '<b>What j labels.</b> The j-invariant is a number identifying the geometric curve. The special labels j = 1728 and j = 0 mark extra symmetry, which permits more twists.',
        'The same j-invariant means the same geometric elliptic curve. It does not determine the number or group structure of base-field points.')

    standard(c,8,'4 / Cubic twists','Cubic twists form a three-way family.',
        'When A = 0, use y² = x³ + a²B. A cube root v³ = a removes the coefficient change.',
        '08_cubic_family',
        '<b>The hidden map.</b> From the twisted model back to the original, send (x,y) to (x/v², y/a). The cube root appears in the x-coordinate.',
        '<b>Notice the square.</b> The multiplier on B is a², even though this is called a cubic twist. Over F₇ the square multipliers are 1, 2 and 4.',
        'Cubic refers to the missing cube root in the coordinate map. For q ≡ 1 mod 3, there are two nontrivial cubic twists.',color='purple')

    standard(c,9,'4 / Quartic twists','Quartic twists need B to vanish.',
        'For y² = x³ + Ax, changing A to cA can be undone by a fourth root u⁴ = c. The coefficients here are successive powers of 2 in F₁₃.',
        '09_quartic_family',
        '<b>Read the four models.</b> The coefficients are A = 1, 2, 4 and 8. The displayed degree is the smallest extension over which each becomes isomorphic to the A = 1 model.',
        '<b>Find the quadratic member.</b> A = 4 is a square multiplier. It is the quadratic twist of the original. A = 2 and A = 8 require degree 4.',
        'A quartic-twist family includes the original and its quadratic twist, as well as the members of exact degree 4.',color='orange')

    standard(c,10,'4 / Sextic twists','One sextic family holds all six forms.',
        'For y² = x³ + B over F₇, let B = 3ⁱ for i = 0,…,5. Every node has j = 0; the point counts can be very different.',
        '10_sextic_family_wheel',
        '<b>Read the colors.</b> Relative to B = 1: B = 6 is quadratic; B = 2 and 4 are cubic; B = 3 and 5 are exact sextic twists. Degree 1 is the original.',
        '<b>Combine the moves.</b> Multiplying B by a square gives a cubic-twist operation. Multiplying it by a cube gives a quadratic-twist operation. Together they generate this six-class family.',
        '“Sextic family” does not mean every member needs degree 6. The exact degrees here are 1, 2, 3, 3, 6 and 6.')

    standard(c,11,'5 / The field matters','Six possible symmetries ≠ six classes.',
        'Two multipliers describe the same sextic class exactly when their ratio is a sixth power in the base field.',
        '11_field_changes_class_count',
        '<b>Count the classes.</b> Fq* is cyclic of order q − 1. Modding out by n-th powers leaves gcd(n, q − 1) classes. Use n = 2, 4 or 6 for the relevant family.',
        '<b>A useful surprise.</b> Over F₅ there are two j = 0 classes, not six. Both have 6 points. Counting points alone therefore cannot identify the twist class.',
        'Always name the base field. The j-invariant identifies the geometric curve; power classes distinguish its forms over that field.')

    standard(c,12,'5 / The deeper intuition','Frobenius asks: are you visible here?',
        'Frobenius raises each coordinate to the q-th power. Its fixed points are exactly the Fq-rational points.',
        '12_frobenius_visibility',
        '<b>Original curve.</b> A point with Fq coordinates stays fixed. After identifying a quadratic twist with the curve over Fq², its rational points satisfy π(P) = −P instead.',
        '<b>Two steps remove the sign.</b> Applying Frobenius twice fixes an anti-fixed point. Quadratic twists become isomorphic over Fq². Points with P = −P can satisfy both conditions.',
        'Twisting changes the rule for which geometric points count as rational over the base field.')

    standard(c,13,'6 / Cryptography','Some inputs can land on the twist.',
        'For Montgomery single-coordinate scalar multiplication, a nonzero right-hand side selects the curve or its quadratic twist by its square class.',
        '13_twist_security',
        '<b>Why group orders matter.</b> Small-order inputs can make scalar multiplication reveal information modulo a small factor. A large prime subgroup on the twist helps support the intended security.',
        '<b>Follow the protocol.</b> Twist security works alongside the required input, subgroup and all-zero-output rules. Unchecked full-coordinate inputs can reach additional curves too.',
        'Twist security means checking the arithmetic of the relevant twist group, not only that of the main curve.',color='orange')

    standard(c,14,'6 / Pairings','A sextic twist makes G₂ cheaper.',
        'BLS12-381 uses a special j = 0 curve. Its G₂ subgroup is represented on a sextic twist with coordinates in Fp².',
        '14_pairing_field_tower',
        '<b>Smaller coordinates.</b> Store and manipulate G₂ points on E′ over Fp². The untwist sends this subgroup to the corresponding points of E over Fp¹².',
        '<b>Keep the objects distinct.</b> Elliptic-curve points live on E or E′. Pairing outputs live in a multiplicative subgroup of Fp¹². The twist does not move all of E(Fp¹²) into E′(Fp²).',
        'The twist degree 6 is measured over Fp². The embedding degree 12 is measured over Fp.')

    # Page 15: a visual reference and short self-checks.
    page_start(c,15,'7 / Make it stick','A compact map you can remember.',
        'Assume characteristic greater than 3. Each formula below specifies a family; its exact degree can be smaller.')
    rows=[('Quadratic','any j','A → d²A;  B → d³B','square root of d','blue'),
          ('Cubic','j = 0','B → a²B','cube root of a','purple'),
          ('Quartic','j = 1728','A → cA','fourth root of c','orange'),
          ('Sextic','j = 0','B → cB','sixth root of c','teal')]
    xs=[M+12,M+122,M+236,M+402]
    for x,label in zip(xs,['FAMILY','CURVE','COEFFICIENT CHANGE','MISSING TOOL']):
        c.setFont('Guide-Bold',7.5);c.setFillColor(HexColor(C['muted']));c.drawString(x,622,label)
    for i,(fam,j,change,root,col) in enumerate(rows):
        yy=551-i*66
        rr(c,M,yy,W-2*M,57,'white')
        c.setFont('Guide-Bold',11);c.setFillColor(HexColor(C[col]));c.drawString(xs[0],yy+23,fam)
        para(c,j,xs[1],yy+36,108,'small',max_height=29)
        para(c,change,xs[2],yy+36,153,'small',max_height=29)
        para(c,root,xs[3],yy+36,105,'small',max_height=29)
    c.setFont('Guide-Bold',15);c.setFillColor(HexColor(C['ink']));c.drawString(M,311,'Check your intuition')
    para(c,'<b>1.</b> A curve over F₁₀₁ has 96 points. How many on its nonsquare quadratic twist?<br/>'
           '<b>2.</b> Does j = 0 guarantee six distinct twists over every finite field?<br/>'
           '<b>3.</b> In F₇, why can the same geometric curve have groups of sizes 12 and 4?',
         M,291,W-2*M,max_height=100)
    para(c,'<b>Answers.</b> 1: 108, because 2(101 + 1) − 96 = 108. '
           '2: No; the full sextic family has gcd(6, q − 1) classes. '
           '3: The isomorphism needs extension-field coefficients and need not preserve F₇ coordinates.',
         M,190,W-2*M,'small',max_height=46)
    takeaway(c,'Ask: Which field? Which coefficient changes? Which root makes the map possible?')
    c.showPage()

    # Page 16: boundaries, glossary and linked primary references.
    page_start(c,16,'Reference','The boundaries of the picture.',
        'The main guide assumes p > 3, nonsingular elliptic curves and isomorphisms that preserve the identity O.')
    y=625
    y=para(c,'<b>Characteristics 2 and 3.</b> At j = 0 the geometric symmetry groups become nonabelian, '
             'with orders 24 and 12 respectively. The class counts below include the original curve. '
             'In either characteristic, j ≠ 0 gives two classes.',M,y,W-2*M,max_height=72)-17
    # Small-characteristic table.
    c.setFont('Guide-Bold',10);c.setFillColor(HexColor(C['muted']))
    c.drawString(M+10,y,'BASE FIELD');c.drawString(M+310,y,'CLASSES AT j = 0');y-=24
    for label,value in [('F<sub>2<super>m</super></sub>, m odd','3'),
                        ('F<sub>2<super>m</super></sub>, m even','7'),
                        ('F<sub>3<super>m</super></sub>, m odd','4'),
                        ('F<sub>3<super>m</super></sub>, m even','6')]:
        rr(c,M,y-8,W-2*M,24,'white')
        para(c,label,M+10,y+10,240,'small',max_height=20)
        c.setFont('Guide',10);c.setFillColor(HexColor(C['ink']));c.drawString(M+370,y,value);y-=29
    y-=8
    y=para(c,'Extensions of degree 8 can occur in characteristic 2. Quadratic binary twists use '
             'Artin–Schreier equations s² + s = δ rather than nonsquare parameters. '
             'The simple 2, 3, 4, 6 classification therefore needs the p > 3 assumption. [2]',M,y,W-2*M,'small',max_height=56)-22
    y=para(c,'<b>An odd-characteristic exception.</b> At j = 1728 with q ≡ 3 mod 4, the usual '
             'nonsquare quadratic-twist construction gives an isomorphic curve. A second form '
             'still exists in the quartic family. [2]',M,y,W-2*M,'small',max_height=45)-20
    c.setFont('Guide-Bold',15);c.setFillColor(HexColor(C['ink']));c.drawString(M,y,'Sources and further study');y-=22
    refs=[
        ('1','Steven Galbraith, Mathematics of Public Key Cryptography, chapter 9, §§9.4–9.5. Definitions, automorphisms and twists.',
         'https://www.math.auckland.ac.nz/~sgal018/crypto-book/ch9.pdf'),
        ('2','Kronberg, Soomro and Top, Twists of Elliptic Curves (2017). Small characteristic, class counts and capitulation.',
         'https://sigma-journal.com/2017/083/sigma17-083.pdf'),
        ('3','RFC 7748, Elliptic Curves for Security. Montgomery ladders, Curve25519 and its twist.',
         'https://www.rfc-editor.org/rfc/rfc7748.html'),
        ('4','RFC 9380, §8.8 and Appendix G.3. BLS12-381 curve equations and Frobenius-based operations.',
         'https://www.rfc-editor.org/rfc/rfc9380.html#section-8.8'),
        ('5','Steven Galbraith, chapter 26, §26.6.2. Why twists improve pairing representations and arithmetic.',
         'https://www.math.auckland.ac.nz/~sgal018/crypto-book/ch26.pdf'),
    ]
    for num,label,url in refs:
        y=para(c,f'<b>[{num}]</b> <a href="{url}" color="{C["blue"]}">{label}</a>',M,y,W-2*M,'refs',max_height=45)-9
    y-=3
    para(c,'<b>Verification.</b> The F₇ and F₁₃ point counts were enumerated directly; the displayed F₇ group structures '
           'and the two F₄₉ counts were checked computationally. Diagrams are original. '
           'The separate image bundle contains PNG and vector SVG versions.',M,y,W-2*M,'small',max_height=54)
    c.showPage()
    c.save()
    # Merge vector figure pages into the reserved rectangles. No rasterization.
    reader=PdfReader(str(path))
    writer=PdfWriter()
    writer.clone_document_from_reader(reader)
    for idx,name,scale,x,y in OVERLAYS:
        source=PdfReader(str(BUILD/f'{name}.pdf')).pages[0]
        transform=Transformation().scale(scale).translate(x,y)
        writer.pages[idx].merge_transformed_page(source,transform,over=True)
    temporary=BUILD/'guide_merged.pdf'
    with temporary.open('wb') as f: writer.write(f)
    temporary.replace(path)
    return path


def package():
    (DIAG/'README.txt').write_text(
        'ELLIPTIC CURVE TWISTS — DIAGRAMS\n\n'
        'These 14 original teaching diagrams accompany the PDF visual guide.\n'
        'PNG files are high-resolution previews; SVG files preserve vector shapes.\n'
        'All curve point counts include the identity O; O is not drawn on affine grids.\n'
        'Symmetry polygons are symbolic group diagrams, not drawings of elliptic curves.\n'
        'Main classification assumes characteristic greater than 3.\n\n'+
        '\n\n'.join(f'{r["name"]}: {r["title"]}\n{r["description"]}' for r in MANIFEST)+'\n')
    zpath=OUT/'Elliptic_Twists_Diagrams_PNG_and_SVG.zip'
    with zipfile.ZipFile(zpath,'w',zipfile.ZIP_DEFLATED) as z:
        for p in sorted(DIAG.iterdir()): z.write(p,'elliptic_twist_diagrams/'+p.name)
    return zpath


if __name__=='__main__':
    make_diagrams()
    pdf=create_pdf()
    bundle=package()
    print(json.dumps({'pdf':str(pdf),'diagram_bundle':str(bundle),'diagrams':len(MANIFEST)},indent=2))
