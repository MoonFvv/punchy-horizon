"""Position/scale regression checks driven through Horizon's real settings UI."""
import json
import math


def check_viewmodel(ctl, observe, screenshot, launch, process, home, diagnostic, result):
    def find(prefix):
        # Native mod buttons are a scrollable list. Click real visible widgets.
        size = ctl('ui')['gui_size']
        x, y = size[0]/2, size[1]/2
        ctl('scroll-at', x, y, 40)
        observe(.1)
        for _ in range(16):
            ui = ctl('ui')
            nodes = [n for n in ui['widgets'] if (n.get('label') or '').startswith(prefix)
                     and 38 < n['bounds'][1]+n['bounds'][3]/2 < ui['gui_size'][1]-38]
            if nodes:
                return nodes[0]
            ctl('scroll-at', x, y, -2)
            observe(.1)
        raise AssertionError('Missing settings widget: ' + prefix)

    def click(prefix):
        ctl('click', find(prefix)['id'])
        ctl('ui')

    def open_settings():
        ctl('options', 'mods')
        click('Settings')

    def play():
        for _ in range(12):
            if ctl('ui')['screen'] == 'playing':
                return
            ctl('key', 'escape', 'tap')
            observe(.1)
        raise AssertionError('settings did not close')

    def switch_to_section(label_substring):
        for _ in range(5):
            node = find('▶ Section: [')
            if label_substring in node['label']:
                return
            ctl('click', node['id'])
            ctl('ui')
        raise AssertionError('could not switch to section: ' + label_substring)

    def adjust(name, increase, count=1):
        prefix = ('[+] ' if increase else '[-] ') + name
        for _ in range(count):
            click(prefix)

    def rendered():
        observe(.5)
        return json.loads(diagnostic.read_text())

    def equal_settings(actual, expected):
        return all(math.isclose(a, b, abs_tol=1e-6)
                   for key in expected for a, b in zip(actual[key], expected[key]))

    def matrix_check(before, after, key, offset, scale):
        for a, b in zip(before[key], after[key]):
            assert a is not None and b is not None
            for i in range(12):
                assert math.isclose(b[i], a[i] * scale, abs_tol=2e-5), (key, i, a[i], b[i])
            for i in range(3):
                assert math.isclose(b[12+i], a[12+i] + offset[i]/16, abs_tol=2e-5), (key, a, b)

    ctl('command', 'item replace entity @s weapon.mainhand with minecraft:diamond_sword')
    observe(2)
    baseline = rendered()
    screenshot('viewmodel-default')
    open_settings()
    switch_to_section('Arms')
    adjust('X', True, 2)
    adjust('Y', False)
    adjust('Z', True)
    adjust('Scale', False, 2)
    screenshot('viewmodel-settings')
    play()
    arms = rendered()
    matrix_check(baseline, arms, 'arm_matrices', [1, -.5, .5], .9)
    matrix_check(baseline, arms, 'item_matrices', [0, 0, 0], 1)
    result['checks']['arm_position_and_scale_independent_of_items'] = True
    screenshot('viewmodel-arms-adjusted')
    open_settings()
    switch_to_section('Items')
    adjust('X', False, 2)
    adjust('Y', True)
    adjust('Z', False)
    adjust('Scale', True, 2)
    play()
    items = rendered()
    matrix_check(arms, items, 'arm_matrices', [0, 0, 0], 1)
    matrix_check(arms, items, 'item_matrices', [-1, .5, -.5], 1.1)
    result['checks']['item_position_and_scale_independent_of_arms'] = True
    screenshot('viewmodel-both-adjusted')
    expected = {'arms': [1, -.5, .5, .9], 'items': [-1, .5, -.5, 1.1]}
    assert equal_settings(items['viewmodel'], expected), items
    ctl('quit'); process().wait(timeout=30)
    launch()
    observe(2)
    restored = rendered()
    assert equal_settings(restored['viewmodel'], expected), restored
    result['checks']['position_and_scale_survive_game_restart'] = True
    open_settings()
    switch_to_section('Arms')
    click('Reset 1st-Person Arms Viewmodel to defaults')
    play()
    reset_arm = rendered()
    assert equal_settings(reset_arm['viewmodel'], dict(expected, arms=[0, 0, 0, 1])), reset_arm
    result['checks']['reset_group_preserves_other_group'] = True
    open_settings()
    click('Reset all mod settings to defaults')
    play()
    reset_all = rendered()
    assert reset_all['viewmodel'] == {'arms': [0, 0, 0, 1], 'items': [0, 0, 0, 1]}, reset_all
    result['checks']['reset_all_restores_defaults'] = True
    # The same persisted storage used by the actual UI contains the defaults.
    folder = home/'horizon-mods/.horizon-mod-data/punchy'
    for target in ['arms', 'items']:
        for key in ['x', 'y', 'z', 'scale']:
            assert float((folder/(target+'_'+key)).read_text()) == (1 if key == 'scale' else 0)
