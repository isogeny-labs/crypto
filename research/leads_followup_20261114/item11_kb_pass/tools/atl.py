import sys,re
sys.path.insert(0,'tools')
import doc
for i in sys.argv[1:]:
    a=doc.atlas(i)
    if not a: print('##',i,'NOT IN CORPUS\n');continue
    t=a[2]
    def f(k,n=700):
        m=re.search(r'- \*\*%s\*\*: (.*?)(?=\n- \*\*|\Z)'%k,t,re.S); return (m.group(1).strip()[:n] if m else '')
    print('##',i,a[1]);print(' title:',f('title',250));print(' disp:',f('disposition',100));print(' best:',f('best_measured',650));print(' mech:',f('mechanism',350));print()
