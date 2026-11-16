CONFIG={'threads':1,'cut':True,'cut.xor':True,'cut.aig':True,'cut.delay':0,'anf':False}

def make_solver(z3,seed):
    z3.set_param('parallel.enable',False)
    for name,value in CONFIG.items():z3.set_param('sat.'+name,value)
    S=z3.Tactic('sat').solver();desc=S.param_descrs();names={desc.get_name(i)for i in range(desc.size())}
    assert set(CONFIG)<=names
    S.set(**CONFIG);S.set(random_seed=seed)
    for name,value in CONFIG.items():
        expected=str(value).lower();assert z3.get_param('sat.'+name)==expected
    return S

def readback(z3):return {name:z3.get_param('sat.'+name)for name in CONFIG}
