"""Rebuild per-curve summary and publication figure from raw quartic runs."""
from __future__ import annotations

import csv
import json
import math
import pathlib
import statistics
import sys

import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt

ROOT=pathlib.Path(__file__).resolve().parent
matplotlib.rcParams['svg.hashsalt']='quartic-twist24-20261022'
matplotlib.rcParams['svg.fonttype']='none'


def med(xs):
    return statistics.median(xs)


def costs(rec):
    base=rec['base_fp_muls']+rec['table_fp_muls']
    extra=(0,rec['cubic_map_fp_muls'],rec['twist_map_fp_muls'])
    setup_cubic=rec.get('cubic_eigen_fp_muls',0)
    setup_twist=(rec.get('twist_geometry_fp_muls',0)+
                 rec.get('frob_eigen_fp_muls',0))
    return [
        base+extra[i]+rec['stream_fp_muls_at_solve'][i]+
        rec['verification_fp_muls'][i]+(0,setup_cubic,setup_cubic+setup_twist)[i]
        for i in range(3)
    ]


def summarize(path):
    records=[json.loads(s) for s in path.read_text().splitlines() if s.strip()]
    assert len(records)==21
    out=[]
    for p in sorted({x['p'] for x in records}):
        rows=sorted((x for x in records if x['p']==p),key=lambda x:x['seed'])
        assert [x['seed'] for x in rows]==[0,1,2]
        assert all(all(x[a]['correct'] for a in ('flat','cubic','twist24')) for x in rows)
        assert all(all(z['correct'] for z in x['rho']) for x in rows)
        ratio_res=[x['cubic']['residuals_at_solve']/x['twist24']['residuals_at_solve'] for x in rows]
        ratio_la=[x['cubic']['la_ops']/x['twist24']['la_ops'] for x in rows]
        ratio_m=[costs(x)[1]/costs(x)[2] for x in rows]
        rho_m=[]
        gap=[]
        for x in rows:
            setup=(x.get('cubic_eigen_fp_muls',0)+x.get('twist_geometry_fp_muls',0)+
                   x.get('frob_eigen_fp_muls',0))
            rho=med(z['fp_muls']+setup for z in x['rho'] if z['fold']=='Twist24')
            rho_m.append(rho)
            gap.append(costs(x)[2]/rho)
        out.append({
            'p':p,'r':rows[0]['n'],'log2_r':math.log2(rows[0]['n']),
            'base_abscissae':rows[0]['base_points'],
            'flat_columns':rows[0]['flat']['columns'],
            'cubic_columns':rows[0]['cubic']['columns'],
            'twist24_columns':rows[0]['twist24']['columns'],
            'flat_residuals_median':med(x['flat']['residuals_at_solve'] for x in rows),
            'cubic_residuals_median':med(x['cubic']['residuals_at_solve'] for x in rows),
            'twist24_residuals_median':med(x['twist24']['residuals_at_solve'] for x in rows),
            'residual_ratio_median':med(ratio_res),'residual_ratio_min':min(ratio_res),
            'residual_ratio_max':max(ratio_res),
            'cubic_la_median':med(x['cubic']['la_ops'] for x in rows),
            'twist24_la_median':med(x['twist24']['la_ops'] for x in rows),
            'la_ratio_median':med(ratio_la),'la_ratio_min':min(ratio_la),
            'la_ratio_max':max(ratio_la),
            'cubic_m_median':med(costs(x)[1] for x in rows),
            'twist24_m_median':med(costs(x)[2] for x in rows),
            'm_ratio_median':med(ratio_m),'m_ratio_min':min(ratio_m),
            'm_ratio_max':max(ratio_m),
            'rho24_m_median':med(rho_m),
            'twist24_to_rho_median':med(gap),'gap_min':min(gap),'gap_max':max(gap),
        })
    return out


def draw(rows,svg,png):
    fig,(ax1,ax2)=plt.subplots(1,2,figsize=(11.5,4.2),layout='constrained')
    xx=[r['log2_r'] for r in rows]
    styles=[('residual_ratio','Residual queries','tab:blue','o'),
            ('la_ratio','Modular row work','tab:purple','s'),
            ('m_ratio','Counted Fp multiplies','tab:green','^')]
    for prefix,label,color,marker in styles:
        yy=[r[prefix+'_median'] for r in rows]
        lo=[r[prefix+'_median']-r[prefix+'_min'] for r in rows]
        hi=[r[prefix+'_max']-r[prefix+'_median'] for r in rows]
        ax1.errorbar(xx,yy,yerr=[lo,hi],label=label,color=color,marker=marker,
                     lw=1.6,capsize=2.5,ms=5)
    ax1.axhline(1,color='0.5',lw=1,ls='--')
    ax1.set(xlabel='Prime subgroup size (log₂ r)',ylabel='Cubic-only / twist24 count',
            title='Same physical factor base')
    ax1.set_ylim(bottom=0)
    ax1.grid(alpha=.2)
    ax1.legend(fontsize=8,loc='upper left')
    yy=[r['twist24_to_rho_median'] for r in rows]
    lo=[r['twist24_to_rho_median']-r['gap_min'] for r in rows]
    hi=[r['gap_max']-r['twist24_to_rho_median'] for r in rows]
    ax2.errorbar(xx,yy,yerr=[lo,hi],color='tab:red',marker='D',lw=1.6,capsize=2.5,ms=5)
    ax2.axhline(1,color='0.5',lw=1,ls='--')
    ax2.set_yscale('log')
    ax2.set(xlabel='Prime subgroup size (log₂ r)',
            ylabel='Twist24 IC / matched twist24 rho (Fp multiplies)',
            title='One-target post-input gap to rho')
    ax2.grid(alpha=.2,which='both')
    for x,y,r in zip(xx,yy,rows):
        ax2.annotate(str(r['p']),(x,y),xytext=(0,7),textcoords='offset points',
                     ha='center',fontsize=7)
    fig.savefig(svg)
    fig.savefig(png,dpi=220)
    plt.close(fig)
    svg.write_text('\n'.join(line.rstrip() for line in svg.read_text().splitlines())+'\n')


def main(raw):
    rows=summarize(raw)
    csv_path=ROOT/'quartic_twist24_summary_v2.csv'
    with csv_path.open('w',newline='') as f:
        w=csv.DictWriter(f,fieldnames=list(rows[0]),lineterminator='\n')
        w.writeheader();w.writerows(rows)
    draw(rows,ROOT/'quartic_twist24_cost_v2.svg',ROOT/'quartic_twist24_cost_v2.png')
    print(json.dumps({'curves':len(rows),'runs':21,'csv':str(csv_path)},sort_keys=True))


if __name__=='__main__':
    main(pathlib.Path(sys.argv[1]))
