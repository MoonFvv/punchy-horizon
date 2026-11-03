#!/usr/bin/env python3
"""Package source/data and validate with the pinned Horizon bundle parser."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import zipfile

ROOT = Path(__file__).resolve().parents[1]
PIN = '81ae4a7d8f27d38e5cabbdcda0a06b2c4b000045'
MIXINS = 'ddd9700b21997c95d95ac6ee5ecafe3830cf622a'
VERSION = '0.1.3'
ENGINE = ('engine.rs', 'resources.rs', 'actions.rs', 'tuning.rs',
          'horizon_pose.rs', 'skeleton.rs', 'attachments.rs', 'playback.rs',
          'phases.rs', 'policy.rs', 'player.rs')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--horizon-source', type=Path, required=True,
                        help='Horizon source checked out at the tested commit')
    parser.add_argument('--offline', action='store_true')
    args = parser.parse_args()
    horizon = args.horizon_source.resolve()
    revision = subprocess.check_output(
        ['git', '-C', str(horizon), 'rev-parse', 'HEAD'], text=True).strip()
    if revision != PIN:
        parser.error('Horizon checkout must be at ' + PIN)
    dependency = horizon / 'crates/horizon-mods'
    if not (dependency / 'Cargo.toml').is_file():
        parser.error('Missing horizon-mods crate')
    source = ROOT / 'mixins/punchy'
    manifest = {'schema': 1, 'id': 'punchy', 'version': VERSION,
                'game': '26.3', 'horizon': PIN, 'mixins': MIXINS}
    files = {'horizon-native.json': (json.dumps(manifest, indent=2) + '\n').encode()}
    prefix = 'mixins/punchy/'
    for path in sorted(source.iterdir()):
        if path.is_file() and path.suffix in ('.rs', '.toml') and path.name not in ENGINE:
            files[prefix + path.name] = path.read_bytes()
    for name in ENGINE:
        files[prefix + 'engine/' + name] = (source / name).read_bytes()
    files[prefix + 'engine/Cargo.toml'] = b'''[package]
name = "punchy-engine"
version = "3.1.0"
edition = "2024"
publish = false
[lib]
path = "engine.rs"
[dependencies]
glam = "0.30"
serde_json = "1.0.151"
zip = { version = "9.1.3", default-features = false, features = ["deflate"] }
'''
    for path in sorted((source / 'data').rglob('*')):
        if path.is_file():
            files[prefix + 'engine/data/' + path.relative_to(source / 'data').as_posix()] = path.read_bytes()
    for name in ('LICENSE', 'LICENSE-Punchy', 'LICENSE-Horizon', 'THIRD_PARTY_NOTICES.md', 'README.md'):
        bundle_name = 'NOTICE.md' if name == 'THIRD_PARTY_NOTICES.md' else name
        files[prefix + bundle_name] = (ROOT / name).read_bytes()
    output = ROOT / 'dist' / ('punchy-rust-preview-' + VERSION + '.hnmod')
    output.parent.mkdir(exist_ok=True)
    with zipfile.ZipFile(output, 'w', compression=zipfile.ZIP_DEFLATED) as archive:
        for name, data in sorted(files.items()):
            info = zipfile.ZipInfo(name, (2026, 11, 3, 0, 0, 0))
            info.compress_type = zipfile.ZIP_DEFLATED
            info.external_attr = 0o100644 << 16
            archive.writestr(info, data)
    validator = ROOT / '.build/validator'
    (validator / 'src').mkdir(parents=True, exist_ok=True)
    (validator / 'Cargo.toml').write_text('''[package]
name = "punchy-package-validator"
version = "0.1.0"
edition = "2024"
publish = false
[dependencies]
horizon-mods = { path = ''' + json.dumps(str(dependency)) + ''', features = ["index"] }
''')
    shutil.copyfile(ROOT / 'tools/validator-src/main.rs', validator / 'src/main.rs')
    shutil.copyfile(ROOT / 'tools/validator-Cargo.lock', validator / 'Cargo.lock')
    command = ['cargo', 'run', '--manifest-path', str(validator / 'Cargo.toml'), '--locked']
    if args.offline:
        command.append('--offline')
    command += ['--', str(output)]
    subprocess.run(command, check=True, env=dict(os.environ))
    print(json.dumps({'file': output.name, 'sha256': hashlib.sha256(output.read_bytes()).hexdigest(),
                      'files': len(files), 'validation': 'pinned Horizon Bundle::read passed'}, indent=2))


if __name__ == '__main__':
    main()
