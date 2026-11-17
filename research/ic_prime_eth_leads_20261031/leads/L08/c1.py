import sys, json, argparse, random, time
REF="/Volumes/SSD990-2/cryptanalysis/experiments/hashed-ecdl-tensor-reference"
sys.dont_write_bytecode=True
sys.path.insert(0,REF)
import tensor_cli
inst = sys.argv[1]
a = argparse.Namespace(instance=inst, slots=2, seed=1, seconds=60, max_operations=2000000, max_edges=10**7, plan_only=True, tight_precision=False, output=None)
t=time.time()
r = tensor_cli.challenge_network(a)
n = r["network"]
print(json.dumps({"inst":inst,"edges":n["boolean_network_edges"],"vars":n["variables"],"constraints":n["constraints"],"maxdeg":n["maximum_degree"],"blocks":n["multiplication_blocks"],"sec":round(time.time()-t,1)}))
