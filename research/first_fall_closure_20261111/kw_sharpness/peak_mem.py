import resource, time, sys
import run_table
t=time.time(); r=run_table.one((21,7,0)); 
print("n=21 n'=7 N=21 single sample: wall %.1fs, peak RSS %.0f MB (ru_maxrss)" % (time.time()-t, resource.getrusage(resource.RUSAGE_SELF).ru_maxrss/1024))
print("top Dff", r["Dff_top"], "op", r["Dff_op"], "nullity7", r["nullity7"])
