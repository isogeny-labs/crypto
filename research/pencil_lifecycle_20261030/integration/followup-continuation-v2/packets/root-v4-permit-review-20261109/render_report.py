from pathlib import Path
import json,hashlib,fitz
P=Path(__file__).resolve().parent
r=json.loads((P/'review.json').read_text())
steps=['Validate current owner + cgroup','Publish bounded atomic permit','Checkpoint and flush metadata','Worker: strict identity + expiry','Expired read: one immediate reread']
svg=['<svg xmlns="http://www.w3.org/2000/svg" width="900" height="410"><rect width="900" height="410" fill="#f6f8fc"/>']
for i,t in enumerate(steps):
 y=20+i*76;svg.append(f'<rect x="25" y="{y}" width="850" height="52" rx="7" fill="white" stroke="#446080"/><text x="45" y="{y+33}" font-size="22" font-family="sans-serif">{i+1}. {t}</text>')
svg.append('</svg>');(P/'permit-order.svg').write_text('\n'.join(svg))
d=fitz.open();p=d.new_page(width=595,height=842);p.insert_text((35,38),'Root v4 permit source review',fontsize=18)
s=fitz.open(stream=(P/'permit-order.svg').read_bytes(),filetype='svg');q=fitz.open('pdf',s.convert_to_pdf());p.show_pdf_page(fitz.Rect(30,60,565,305),q,0)
text='Status: '+r['verdict']+'\n\n'+r['summary']+'\n\nLimits: 10-second permit; 15-second lease margin; single 1800-second validation; 2 CPUs / 16 GiB / swap 0.\n\nReviewer authored v3 ancestry but did not author v4. Review plan preceded source inspection. No compiler, payload or qualification execution by reviewer.\n\nRequired next bindings: actual qualification result, fresh post-v3 mutable-cache inventory, current ledger and lease, exact publication readback.\n\nMachine-readable evidence: review.json, initial-findings.json, source snapshots and manifest.json. No measured runtime improvement is asserted.'
assert p.insert_textbox(fitz.Rect(35,330,560,790),text,fontsize=11,lineheight=1.3)>=0
d.save(P/'report.pdf',garbage=4,deflate=True);p.get_pixmap().save('/var/tmp/root-v4-permit-review.png')
