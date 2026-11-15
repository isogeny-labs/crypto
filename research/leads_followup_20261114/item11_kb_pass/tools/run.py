import sys,json
sys.path.insert(0,'tools')
import q
qs=json.load(open(sys.argv[1]))
for tag,lst in qs.items():
    print('\n######',tag)
    for s in lst:
        print('--',s)
        try: rows=q.search(s,5)
        except Exception as e: print('ERR',e);continue
        for r in rows: print('  ',r[0][7:19],'|',r[2][:8],r[3][-70:],'|',r[5].replace('\n',' ')[:170])
