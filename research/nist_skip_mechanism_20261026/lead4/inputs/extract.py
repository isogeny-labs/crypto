# PARSE ONLY: flattens committed sibling JSON into TSV for GP. No result is computed here.
import json,sys
S='/home/user/crypto-autoresearcher/research/'
acc=json.load(open(S+'nist_seed_1997_acceptance_20261024/analysis.json'))
near=json.load(open(S+'nist_seed_1997_nearprime_20261024/runs/positive-20261024T022524Z/analysis.json'))
out=sys.argv[1]
with open(out+'/acc69.tsv','w') as f:
    f.write('#stream\tindex\tlabel\tbranches\torder_bits\n')
    for r in acc['rows']:
        f.write('\t'.join([r['stream'],str(r['index']),r['label'],'+'.join(r['branches']),'+'.join(map(str,r['order_bits']))])+'\n')
with open(out+'/near_branches.tsv','w') as f:
    f.write('#stream\ttimestamp\tindex\tside\tp\torder\tsmooth_cofactor\n')
    for r in near['rows']:
        if r['eligible_1997_minimum_255']:
            f.write('\t'.join([r['stream'],str(r['timestamp']),str(r['index']),r['side'],r['p'],r['order'],r['smooth_cofactor']])+'\n')
with open(out+'/streams.tsv','w') as f:
    f.write('#stream\tpositive_exact_positions\tcensored_positions\n')
    for k,v in near['streams'].items():
        f.write('%s\t%d\t%d\n'%(k,v['positive_exact_seed_positions'],v['censored_seed_positions']))
