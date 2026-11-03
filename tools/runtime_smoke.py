#!/usr/bin/env python3
"""Exercise the compiled native mod in a fresh world; never touch player data."""
import argparse
import json
import math
import os
from pathlib import Path
import subprocess
import tempfile
import time

p=argparse.ArgumentParser()
p.add_argument('--binary',type=Path,required=True)
p.add_argument('--ctl',type=Path,required=True)
p.add_argument('--resources',type=Path,required=True)
p.add_argument('--report',type=Path,required=True)
a=p.parse_args()
for key in vars(a):setattr(a,key,getattr(a,key).resolve())
a.report.parent.mkdir(parents=True,exist_ok=True)
root=Path(tempfile.mkdtemp(prefix='punchy-smoke-'))
home=root/'home';home.mkdir()
socket=root/'control.sock'
diagnostic=root/'punchy.json'
render_diagnostic=root/'render.json'
env=dict(os.environ)
env.pop('PUNCHY_SOURCE_JAR',None)
for k,folder in [('HORIZON_HOME',home),('HORIZON_LAUNCHER_HOME',root/'launcher'),('XDG_DATA_HOME',root/'data'),('XDG_CONFIG_HOME',root/'config'),('HORIZON_LAUNCHER_TRASH',root/'trash'),('HORIZON_LAUNCHER_CLIPBOARD',root/'clipboard'),('HORIZON_LAUNCHER_DESKTOP',root/'desktop')]:
    folder.mkdir(exist_ok=True);env[k]=str(folder)
env.update(HORIZON_NO_AUDIO='1',HORIZON_LAUNCHER_NO_SELF_UPDATE='1',PUNCHY_DIAGNOSTICS=str(diagnostic),PUNCHY_RENDER_DIAGNOSTICS=str(render_diagnostic))
(home/'options.txt').write_text('enableVsync:false\nmaxFps:60\npauseOnLostFocus:false\nbobView:false\nguiScale:2\n')
(home/'horizon-graphics.txt').write_text('lodDistance:0\nhdr:false\npickedShaderPack:\n')
result={'scratch':str(root),'checks':{},'screenshots':[],'active_clips':[]}
log=a.report.with_suffix('.log').open('w')
process=None

def ctl(*args):
    r=subprocess.run([str(a.ctl),'--socket',str(socket),'--timeout','45',*map(str,args)],env=env,capture_output=True,text=True,timeout=55)
    if r.returncode:raise RuntimeError(' '.join(map(str,args))+': '+r.stdout+r.stderr)
    return json.loads(r.stdout)

def observe(seconds):
    rows=[];deadline=time.monotonic()+seconds
    while time.monotonic()<deadline:
        if process.poll() is not None:raise RuntimeError('game exited')
        if diagnostic.exists():
            try:rows.append(json.loads(diagnostic.read_text()))
            except json.JSONDecodeError:pass
        if render_diagnostic.exists():
            try:
                rendered=json.loads(render_diagnostic.read_text())
                matrices=[m for key in ('arm_matrices','item_matrices') for m in rendered[key] if m is not None]
                assert all(isinstance(x,(float,int)) and math.isfinite(x) for m in matrices for x in m),rendered
                if matrices:result['checks']['observed_render_matrices_finite']=True
            except json.JSONDecodeError:pass
        time.sleep(.025)
    result['active_clips']+=list(dict.fromkeys(r['active']['name'] for r in rows if r.get('active')))
    return rows

def screenshot(name):
    out=a.report.parent/(name+'.png');ctl('screenshot',out);result['screenshots'].append(str(out))

try:
    process=subprocess.Popen([str(a.binary),'--game-dir',str(home),'--resources',str(a.resources),'--name','PunchySmoke','--render-distance','3','--width','960','--height','540','--offscreen','--control','--control-socket',str(socket),'--world','PunchySmoke','--seed','0','--creative'],cwd=home,env=env,stdout=log,stderr=subprocess.STDOUT)
    deadline=time.monotonic()+60
    while not socket.exists():
        if process.poll() is not None:raise RuntimeError('game failed to launch ('+str(process.returncode)+'); see '+str(a.report.with_suffix('.log')))
        if time.monotonic()>deadline:raise TimeoutError('no control socket')
        time.sleep(.1)
    ctl('wait-ready','45')
    rows=observe(.5)
    assert rows and rows[-1]['loaded'] and rows[-1]['clips']==223,rows[-1:]
    assert rows[-1]['resource_source']=='embedded'
    assert not list(home.rglob('*.jar'))
    result['checks']['bundled_223_clips_without_external_jar']=True
    deadline=time.monotonic()+15
    while True:
        render=json.loads(render_diagnostic.read_text())
        if render['authored_poses']==2 and render['frames']>5:break
        if time.monotonic()>deadline:raise AssertionError(render)
        observe(.1)
    result['checks']['source_skeleton_reaches_renderer']=True
    result['initial_render']=render
    ctl('command','tp @s 0 90 0 0 0')
    ctl('command','fill -3 88 -3 3 88 5 stone')
    ctl('command','give @s diamond_sword')
    ctl('wait','5')
    ctl('select','minecraft:diamond_sword')
    observe(1)
    screenshot('punchy-idle')
    ctl('mouse','left','down')
    rows=observe(.75)
    ctl('mouse','left','up')
    assert any(r.get('active',{}).get('name','').startswith('attack_') for r in rows if r.get('active')),rows[-3:]
    result['checks']['sword_attack_reaches_runtime']=True
    ctl('key','f6','tap')
    rows=observe(.15)
    deadline=time.monotonic()+3
    while not any(r.get('active',{}).get('name','')=='new_inspect_right' for r in rows if r.get('active')) and time.monotonic()<deadline:
        rows+=observe(.1)
    screenshot('punchy-inspect')
    result['checks']['inspect_key_accepted']=any(r.get('active',{}).get('name','')== 'new_inspect_right' for r in rows if r.get('active'))
    ctl('release');observe(.5)
    ctl('command','item replace entity @s weapon.offhand with minecraft:shield')
    observe(1)
    ctl('mouse','right','down');rows=observe(.5)
    screenshot('punchy-shield')
    ctl('mouse','right','up')
    result['checks']['shield_use_reaches_runtime']=any('shield' in r.get('active',{}).get('name','') for r in rows if r.get('active'))
    ctl('command','item replace entity @s weapon.offhand with air')
    ctl('command','tp @s 0 90 0 0 -60')
    for item, prefix in [('golden_apple','eat'),('milk_bucket','drink')]:
        ctl('command','give @s '+item)
        ctl('wait','5');ctl('select','minecraft:'+item);observe(1)
        ctl('mouse','right','down');rows=observe(2.5)
        ctl('mouse','right','up');rows+=observe(2)
        names=list(dict.fromkeys(r['active']['name'] for r in rows if r.get('active')))
        result[prefix+'_sequence']=names
        result['checks'][prefix+'_start_loop_end']=all(any(n.startswith(prefix+'_'+phase) for n in names) for phase in ['start','loop','end'])
    ctl('command','gamemode survival')
    ctl('command','item replace entity @s weapon.mainhand with minecraft:diamond_pickaxe')
    ctl('command','tp @s 0 90 0 0 0')
    ctl('command','fill -1 90 3 1 93 3 obsidian')
    observe(1)
    ctl('mouse','left','down');rows=observe(2.5)
    ctl('mouse','left','up');rows+=observe(2)
    names=list(dict.fromkeys(r['active']['name'] for r in rows if r.get('active')))
    result['mining_sequence']=names
    phases=set(r.get('mining_phase') for r in rows)
    result['checks']['mining_start_loop_end']=all(phase in phases for phase in ['Start','Loop','End'])
    result['checks']['mining_releases_after_key_up']=rows[-1].get('active') is None
    ctl('command','setblock 0 89 0 stone')
    ctl('command','tp @s 0 90 0 0 0')
    ctl('command','setblock 0 91 3 grass_block')
    ctl('command','setblock 0 92 3 air')
    ctl('command','item replace entity @s weapon.mainhand with minecraft:diamond_shovel')
    observe(1.5)
    ctl('mouse','right','down');rows=observe(.4)
    ctl('mouse','right','up');rows+=observe(.4)
    result['interaction_sequence']=list(dict.fromkeys(r['active']['name'] for r in rows if r.get('active')))
    result['checks']['successful_shovel_use_triggers_configured_clip']=any('mining_use_digging' in n for n in result['interaction_sequence'])
    ctl('command','gamemode creative')
    ctl('command','fill -1 90 3 1 92 3 stone')
    ctl('command','item replace entity @s weapon.mainhand with minecraft:diamond_pickaxe')
    observe(1)
    ctl('mouse','left','down');rows=observe(.5)
    ctl('mouse','left','up');rows+=observe(1)
    result['checks']['creative_instant_break_triggers_mining']=any(r.get('mining_phase') in ('Start','Loop','End') for r in rows)
    ctl('quit');process.wait(timeout=30);process=None
    result['success']=all(result['checks'].values())
except Exception as e:
    result['success']=False;result['error']=str(e)
finally:
    if process is not None and process.poll() is None:
        process.terminate()
        try:process.wait(timeout=10)
        except subprocess.TimeoutExpired:process.kill();process.wait()
    result['active_clips']=list(dict.fromkeys(result['active_clips']))
    a.report.write_text(json.dumps(result,indent=2)+'\n')
    log.close()
print(json.dumps(result,indent=2))
raise SystemExit(0 if result['success'] else 1)
