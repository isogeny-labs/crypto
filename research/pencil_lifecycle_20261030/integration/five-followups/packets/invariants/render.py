import pathlib,json,textwrap,hashlib
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages
from matplotlib.patches import FancyBboxPatch
p=pathlib.Path(__file__).parent
fig,ax=plt.subplots(figsize=(10,5));ax.axis('off');ax.set_xlim(0,10);ax.set_ylim(0,5)
boxes=[(.2,3.5,2.7,1,'Affine input\nvalidity asserted in collector'),(3.6,3.5,2.7,1,'Weighted constructor\nround trip proved over a field'),(7,3.5,2.7,1,'Repeated addition\nclosure still open'),(.2,1,2.7,1,'Distinct support\nprior guard'),(3.6,1,2.7,1,'B = 0 and dq = 0\nforces same support'),(7,1,2.7,1,'Zero-q error excluded\nfield interpretation proved')]
for i,(x,y,w,h,t) in enumerate(boxes):
 ax.add_patch(FancyBboxPatch((x,y),w,h,boxstyle='round,pad=.05',facecolor='#e5f3eb' if i in [1,4,5] else '#eef0f4',edgecolor='#435268'));ax.text(x+w/2,y+h/2,t,ha='center',va='center',fontsize=10)
for y in [4,1.5]:
 for a,b in [(2.95,3.5),(6.35,6.9)]:ax.annotate('',xy=(b,y),xytext=(a,y),arrowprops={'arrowstyle':'->'})
ax.text(5,.35,'Green: new producer-checked field lemmas. Machine correspondence and independent review remain open.',ha='center',fontsize=9)
fig.tight_layout();fig.savefig(p/'diagram.svg');fig.savefig(p/'diagram.png',dpi=120)
with PdfPages(p/'report.pdf') as pdf:
 f=plt.figure(figsize=(8.27,11.69));f.text(.08,.94,'Constructor invariants and branch exclusion',fontsize=17,weight='bold')
 txt='Five field-level declarations compiled; separate axiom readback exited 0. Only propext, Classical.choice and Quot.sound were reported. No scientific trial or benchmark was run. Independent review is pending.\n\nResults: weighted-coordinate round trip; preservation of Mumford divisibility under reweighting; preservation under negation; B=0 and dq=0 force equal support; hence the shared_zero_q_delta guard is excluded after the different-support test.\n\nThese propositions interpret the exact source expressions over a field. They do not establish UInt64 correspondence or closure of every collector transition.'
 f.text(.08,.88,'\n'.join('\n'.join(textwrap.wrap(x,88)) for x in txt.split('\n')),va='top',fontsize=11,linespacing=1.5)
 a=f.add_axes([.05,.22,.9,.32]);a.imshow(plt.imread(p/'diagram.png'));a.axis('off')
 f.text(.08,.15,'Evidence: InvariantBranches.lean; receipt.json; readback.json; sources.json.\nFirst failed compile retained under attempt1/. Source mapping: branches.json.\nDOI: 10.1007/s13389-024-00351-7. Full derivations and obligations: REPORT.md.',fontsize=10,linespacing=1.6)
 pdf.savefig(f);plt.close(f)
 f=plt.figure(figsize=(8.27,11.69));f.text(.08,.94,'Invariant boundary and next proof obligations',fontsize=17,weight='bold')
 paragraphs=[
 'Entry: collector lines 153–160 assert affine validity before conversion. Jacobian::valid checks degree, monicity and polynomial remainder; it does not check primality, canonical words or smoothness. Degree-two weighting/decoding now has a field-level preservation lemma.',
 'Transitions: identity returns its operand. Other dispatcher branches feed scalar multiplication, jump construction, initialization, restart and walk steps. A single closed dispatcher invariant would support induction over these loops. That closure remains unproved.',
 '1+1: common-x ordinate alternatives and doubling tangent validity need a source bridge. Generic 1+2: nonzero output weights factor from K and nonzero input weights; numerator congruence is the next local target.',
 'Same support: inverse and equal-v precede mixed. The prior weighted same-support theorem handles zero slope; the present result handles the different-support shared-zero-q branch. They are distinct exclusions.',
 'Shared positive support: extracted-point validity, matching ordinates and both secondary additions remain obligations. Generic and degree-drop 2+2 weight factors are nonzero under their guards; output congruence and divisor-class meaning remain separate.',
 'Scope: one failed compiler attempt is retained; its sorryAx output is rejected. A source comparison initially found drift in an older three_leads.rs; the current worktree version was then pinned instead. No unsupported historical source identity is assumed.',
 'Next: independent proposition/source review, then machine-operation correspondence and generic 1+2 output congruence. Report no cost ratio from proof-build timings.'
 ]
 y=.88
 for para in paragraphs:
  lines=textwrap.wrap(para,88);f.text(.08,y,'\n'.join(lines),va='top',fontsize=10.5,linespacing=1.4);y-=.022*len(lines)+.025
 pdf.savefig(f);plt.close(f)
