"""Explicit two-workflow bookkeeping. No implicit native calls or retries."""
def execute(driver, native_selection, copied_controls):
    # Called only after frozen protocol custody admission; it records actual
    # Python workflow entry even when controls return on a missing prerequisite.
    driver.receipt['workflows']=[]
    errors=[]
    def invoke(arm, action, missing):
        from datetime import datetime, timezone
        now=lambda:datetime.now(timezone.utc).isoformat()
        start=len(driver.receipt['commands'])
        row={'arm':arm,'invoked':True,'started_utc':now(),'finished_utc':None,'status':'running','reason':'in_progress','command_names':[]}
        driver.receipt['workflows'].append(row); driver.checkpoint()
        result=None
        try:
            result=action()
            row.update(status='succeeded' if result is not None else 'failed',reason='mechanical_predicates_passed' if result is not None else missing)
        except BaseException as error:
            errors.append(error)
            cs=driver.receipt['commands'][start:]
            message=str(error)
            if message=='signal 14' or (cs and cs[-1]['status']=='timeout') or 'wall admission ceiling' in message:
                status='timeout'
            elif isinstance(error,(KeyboardInterrupt,SystemExit)) or message in ('signal 2','signal 15') or (cs and cs[-1]['status']=='interrupted'):
                status='interrupted'
            elif 'storage' in message and 'ceiling' in message:
                status='cancelled'
            else:
                status='failed'
            row.update(status=status,reason=repr(error))
        finally:
            row.update(finished_utc=now(),command_names=[c['name'] for c in driver.receipt['commands'][start:]])
            driver.checkpoint()
        return result
    selected=invoke('native-sat',native_selection,'missing_fixture')
    def control_action():
        if driver.receipt['workflows'][0]['status']!='succeeded': return None
        copied_controls(selected)
        return True
    invoke('copied-source-controls',control_action,'missing_native_precondition')
    if errors: raise errors[0]
    driver.receipt['status']='native_fixture_passed_unified_export_pending' if selected is not None else 'missing_fixture'
    driver.checkpoint()
