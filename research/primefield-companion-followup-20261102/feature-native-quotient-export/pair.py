"""Frozen analysis entrypoint for the two raw collector transcripts."""
import argparse,json
from pathlib import Path
from common import pair
if __name__=='__main__':
 p=argparse.ArgumentParser();p.add_argument('first',type=Path);p.add_argument('second',type=Path);p.add_argument('out',type=Path);a=p.parse_args()
 result=pair(a.first,a.second)
 with a.out.open('x')as f:json.dump(result,f,indent=2);f.write('\n')
