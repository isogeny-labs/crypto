"""Set inherited process limits, then exec the exact requested argv."""
import os
import resource
import sys


def main():
    cpu, memory, seconds, file_bytes = map(int, sys.argv[1:5])
    argv = sys.argv[5:]
    if not argv or cpu not in os.sched_getaffinity(0) or min(memory, seconds, file_bytes) <= 0:
        raise SystemExit('invalid resource arguments')
    os.sched_setaffinity(0, {cpu})
    resource.setrlimit(resource.RLIMIT_AS, (memory, memory))
    resource.setrlimit(resource.RLIMIT_CPU, (seconds, seconds))
    resource.setrlimit(resource.RLIMIT_FSIZE, (file_bytes, file_bytes))
    resource.setrlimit(resource.RLIMIT_CORE, (0, 0))
    os.environ['PYTHONDONTWRITEBYTECODE'] = '1'
    for name in ('OPENBLAS_NUM_THREADS', 'OMP_NUM_THREADS', 'MKL_NUM_THREADS', 'NUMEXPR_NUM_THREADS'):
        os.environ[name] = '1'
    os.execvpe(argv[0], argv, os.environ)


if __name__ == '__main__': main()
