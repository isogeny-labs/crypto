"""Run one isolated experiment; the parent records every result and timeout."""
import json
import sys
import traceback
from common import seed_all


def execute(case):
    seed_all(case['seed'])
    f=case['family']; a=case.get('args',{})
    if f=='trace_zero_structure':
        from trace_zero import trace_structure
        return trace_structure(**a)
    if f=='trace_zero_relations':
        from trace_zero import trace_relations
        return trace_relations(**a)
    if f=='quadratic_genus_two_transfer':
        from quadratic_cover import cover_experiment
        return cover_experiment(**a)
    if f=='cubic_genus_three_transfer':
        from cubic_cover import cubic_experiment
        return cubic_experiment(**a)
    if f=='intermediate_field_controls':
        from towers import tower_controls
        return tower_controls(**a)
    if f=='binary_ghs_geometry_screen':
        from binary_ghs import binary_screen
        return binary_screen(**a)
    if f=='tian_cubic_kernel':
        from tian import tian_kernel
        return tian_kernel()
    if f=='tian_cubic_jacobian':
        from tian import tian_published_pair
        return tian_published_pair()
    if f=='tian_cubic_rational_map':
        from tian import tian_full_map
        return tian_full_map()
    raise ValueError('Unknown experiment family: '+f)


if __name__=='__main__':
    case=json.loads(sys.argv[1])
    try:
        result=execute(case)
    except Exception as exc:
        # Only a narrowly identified eligibility failure is classified this way.
        # Assertion errors, unexpected ValueErrors, and implementation errors
        # remain errors and make the parent run fail.
        ineligible=(isinstance(exc,ValueError) and
                    str(exc).startswith('No suitable odd subgroup for this cubic cover'))
        result={'family':case['family'],'status':'ineligible' if ineligible else 'error',
                'error_type':type(exc).__name__,'reason':str(exc)}
        if not ineligible:
            traceback.print_exc(file=sys.stderr)
    result.update({'case_id':case['case_id'],'seed':case['seed'],
                   'parameters':case.get('args',{}),'replicate':case['replicate']})
    print(json.dumps(result),flush=True)
    sys.exit(1 if result['status']=='error' else 0)
