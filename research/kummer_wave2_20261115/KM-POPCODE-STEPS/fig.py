import json
T=json.load(open('table.json'))
# rows: (label, tag, d, registered)
rows=[("W1 d2 (control)","gen-W1-d0-p40961",2,4),("W1 d3","gen-W1-d0-p40961",3,8),("W1 d4","gen-W1-d0-p40961",4,12),
      ("W2 d3","gen-W2-d0-p40961",3,6),("W2 d4","gen-W2-d0-p40961",4,10),("W3 d4","gen-W3-d0-p40961",4,9),
      ("W3 d3 (reg: undefined)","gen-W3-d0-p40961",3,None),("W2 d2 (reg: <=1)","gen-W2-d0-p40961",2,1)]
W=640;H=60+34*len(rows)+50
o=['<svg xmlns="http://www.w3.org/2000/svg" width="%d" height="%d" font-family="sans-serif" font-size="12">'%(W,H),
 '<rect width="100%" height="100%" fill="white"/>',
 '<text x="10" y="20" font-size="14">KM-POPCODE-STEPS: computed sigma vs registered (s-1 of last positive nu) [pcc:unverified]</text>',
 '<text x="10" y="38">grey bar = registered, blue = computed (limited to computed window), x axis = sigma 0..14</text>']
x0=190;sc=28
for i in range(15):
    o.append('<line x1="%d" y1="50" x2="%d" y2="%d" stroke="#ddd"/><text x="%d" y="%d" font-size="10">%d</text>'%(x0+i*sc,x0+i*sc,H-30,x0+i*sc-3,H-16,i))
for k,(lab,tag,d,reg) in enumerate(rows):
    y=60+34*k
    o.append('<text x="10" y="%d">%s</text>'%(y+14,lab.replace('&','&amp;').replace('<','&lt;')))
    if reg is not None: o.append('<rect x="%d" y="%d" width="%d" height="9" fill="#bbb"/><text x="%d" y="%d" font-size="10">reg %d</text>'%(x0,y,max(reg,0)*sc,x0+max(reg,0)*sc+4,y+8,reg))
    c=T.get(tag,{}).get(str(d))
    if c:
        sg=c['sigma']
        if sg is None: o.append('<text x="%d" y="%d" font-size="10">computed: none (nu=0 for all s)</text>'%(x0,y+22))
        else: o.append('<rect x="%d" y="%d" width="%d" height="9" fill="#2a6fb0"/><text x="%d" y="%d" font-size="10">sigma %d (s_max %d)</text>'%(x0,y+11,sg*sc,x0+sg*sc+4,y+19,sg,c['smax']))
o.append('</svg>')
open('fig_sigma.svg','w').write('\n'.join(o))
import xml.dom.minidom as m; m.parse('fig_sigma.svg'); print('svg ok')
