#!/usr/bin/env python3
"""Run the exact lower-bound certificate and fail closed on GP errors.

Requires PARI/GP. No GRH-dependent class-number computation is run here.
"""
import argparse
from pathlib import Path
import shutil
import subprocess


def main():
    root = Path(__file__).resolve().parent
    local = root/'pari-runtime/usr/bin/gp'
    parser = argparse.ArgumentParser()
    parser.add_argument('--gp', default=str(local) if local.exists() else shutil.which('gp'))
    args = parser.parse_args()
    if not args.gp:
        raise SystemExit('PARI/GP is required; supply --gp /path/to/gp.')
    result = subprocess.run([args.gp,'-q','-f',str(root/'verify_structure.gp')],
                            capture_output=True,text=True,timeout=60)
    output = result.stdout+result.stderr
    if result.returncode or '***' in output:
        raise SystemExit('Certificate failed:\n'+output)
    expected = [
        'UNCONDITIONAL_SUBGROUP_ORDER=74173379224089213153040',
        'UNCONDITIONAL_LOWER_BOUND_VERIFIED=1',
        'FROBENIUS_CLASS_ORDER=155',
        'ALL_CERTIFICATE_CHECKS_PASSED=1',
    ]
    if not all(line in output.splitlines() for line in expected):
        raise SystemExit('Certificate output is incomplete:\n'+output)
    (root/'verify_structure.log').write_text(output)
    print(output,end='')


if __name__ == '__main__':
    main()
