import sys,re
sys.path.insert(0,'tools')
import doc
repo,path,pat=sys.argv[1],sys.argv[2],sys.argv[3]
w=int(sys.argv[4]) if len(sys.argv)>4 else 400
r=doc.body(repo,path)
if not r: print('NOT FOUND');sys.exit()
t=r[1]; print('len',len(t),'source',r[0])
n=0;last=-10**9
for m in re.finditer(pat,t,re.I):
    if m.start()<last+w: continue
    last=m.start(); n+=1
    print('>>',t[max(0,m.start()-w//3):m.start()+w].replace('\n',' '));
    if n>=int(sys.argv[5]) if len(sys.argv)>5 else n>=4: break
