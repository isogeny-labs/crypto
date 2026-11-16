#!/usr/bin/env python3
"""Diagram of the cross-summand equation budget (README section 4). No data."""
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.patches import FancyBboxPatch
import os
root=os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
fig,axs=plt.subplots(1,3,figsize=(9.6,5.4))
def panel(ax,title,unk,eqs,ratio,note):
    ax.set_xlim(0,10); ax.set_ylim(0,10); ax.axis("off"); ax.set_title(title,fontsize=10,weight="bold")
    ax.add_patch(FancyBboxPatch((0.3,5.2),4.5,4.2,boxstyle="round,pad=0.05",fc="#e8f0fe",ec="#1f5fbf"))
    ax.text(2.55,8.9,"unknown bits\nper summand",ha="center",va="center",fontsize=8,weight="bold")
    ax.text(2.55,6.6,unk,ha="center",va="center",fontsize=7.0)
    ax.add_patch(FancyBboxPatch((5.2,5.2),4.5,4.2,boxstyle="round,pad=0.05",fc="#fdecea",ec="#c0392b"))
    ax.text(7.45,8.9,"cross-summand\nequations",ha="center",va="center",fontsize=8,weight="bold")
    ax.text(7.45,6.6,eqs,ha="center",va="center",fontsize=7.0)
    ax.text(5,4.0,ratio,ha="center",fontsize=9.5,weight="bold")
    ax.text(5,1.8,note,ha="center",va="center",fontsize=7.6)
panel(axs[0],"Weil descent over F_{2^n}","n' bits\n(F_2 coordinates\nof x_i in V)","n equations, degree 2\n(the n components\nof S_{m+1} over F_2)","eqs / bits = n/(m n') ~ 1","balance: bounded first fall degree\n(HKY, Petit-Quisquater)")
panel(axs[1],"PKM tower over F_p","t = log2 B bits\n(y_{i,j}^2 = y_{i,j+1})","1 equation (S_{m+1});\n[k]-multiples free but\nenter at degree >= 6","eqs / bits = 1/(m log2 B)","one-generator system in a 2^N-point ring:\nd_F grows (1/8 per bit in t, 1/2 per bit in m)")
panel(axs[2],"Any extra equation over F_p","log2 B + j log2 p bits\n(each image phi_j(P) in\nits own tower; the towers\nmultiply to B p^j)","j + 1 equations","eqs / bits -> 1/log2 p","the three exits (F_{p^k}, Z/p^k, jets)\npay the same log2 p per component")
fig.suptitle("The cross-summand equation budget: why a prime-field tower system stays a one-generator system (README section 4, derived)",fontsize=10)
fig.tight_layout()
fig.savefig(os.path.join(root,"fig_budget.svg")); fig.savefig(os.path.join(root,"fig_budget.png"),dpi=150)
print("wrote fig_budget.svg/png")
