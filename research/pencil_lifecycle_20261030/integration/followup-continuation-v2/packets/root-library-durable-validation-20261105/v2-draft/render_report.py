from pathlib import Path
import json,typst
p=Path(__file__).resolve().parent
paragraphs=(p/'PLAN.md').read_text().split('\n\n')
source='#set page(paper: "a4", margin: 18mm)\n#set text(size: 9.5pt)\n= Durable validation source proposal, v2\n#image("flow.svg",width:100%)\n\n'
for para in paragraphs:
    source+='#text('+json.dumps(para.replace('`','').replace('#',''),ensure_ascii=False)+')\n\n'
(p/'report.typ').write_text(source)
typst.compile(str(p/'report.typ'),output=str(p/'report.pdf'))
