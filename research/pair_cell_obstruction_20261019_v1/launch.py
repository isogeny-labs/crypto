import os,sys
from pathlib import Path
p=str(Path(__file__).with_name('native'))
os.execv(p,[p,*sys.argv[1:]])
