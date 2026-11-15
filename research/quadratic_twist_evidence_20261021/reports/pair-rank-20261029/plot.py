import json,matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from pathlib import Path
r=Path(__file__).resolve().parent;d=json.loads((r/'plot-data.json').read_text())['records']
f,a=plt.subplots(figsize=(7.1,2.55));a.scatter([int(x['base_size']) for x in d],[int(x['pair_sum']) for x in d],s=115,marker='o',facecolors='none',edgecolors='#df7a25',label='Unordered pair count, summed over queries');a.scatter([int(x['base_size']) for x in d],[int(x['rank_sum']) for x in d],s=21,color='#116a8b',label='Coefficient rank, summed over queries');a.set(xlabel='Number of unsigned factor-base columns b',ylabel='Count / rank sum',xticks=[1,2,3,4],title='All 4,485 abstract bases; coincident profiles overlap');a.grid(alpha=.2);a.legend(fontsize=8,loc='upper left');f.tight_layout();f.savefig(r/'profiles.svg');f.savefig(r/'profiles.pdf')

# Collapse only byte-identical SVG draw instructions at coincident positions.
# This is geometry layout, not data selection; all raw plotted records remain.
import xml.etree.ElementTree as E
E.register_namespace('', 'http://www.w3.org/2000/svg')
E.register_namespace('xlink', 'http://www.w3.org/1999/xlink')
p=r/'profiles.svg';tree=E.parse(p)
for parent in tree.iter():
 seen=set()
 for child in list(parent):
  if child.tag.endswith('}use'):
   key=E.tostring(child)
   if key in seen:parent.remove(child)
   else:seen.add(key)
tree.write(p,encoding='utf-8',xml_declaration=True)

p.write_text('\n'.join(line.rstrip() for line in p.read_text().splitlines())+'\n')
