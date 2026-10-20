# SPDX-License-Identifier: GPL-2.0-or-later
# Copyright (C) 2026 CPE Monitor contributors.
"""Build a runtime-only portable ZIP while retaining all license notices."""
from pathlib import Path
import argparse
import hashlib
import json
import os
import re
from zipfile import ZipFile, ZIP_DEFLATED


def portable_files(root, exe):
    licenses = sorted(p for p in (root / 'LICENSES/third-party').rglob('*') if p.is_file())
    licenses += sorted(p for p in (root / 'vendor').rglob('LICENSE*') if p.is_file())
    groups = {}
    sections = {}
    for path in licenses:
        content = path.read_bytes().replace(b'\r\n', b'\n')
        digest = hashlib.sha256(content).hexdigest()
        entry = groups.setdefault(digest, {'content': content, 'names': [], 'section': len(groups) + 1})
        relative = path.relative_to(root).as_posix()
        entry['names'].append(relative)
        sections[relative] = entry['section']
    parts = [b'THIRD-PARTY LICENSE TEXTS\n\nIdentical texts are stored once; all original locations and copyright notices are retained.\n\n']
    for entry in groups.values():
        heading = ('=' * 72 + '\nLicense section ' + str(entry['section']) + '\nOriginal locations:\n' + '\n'.join(entry['names']) + '\n' + '=' * 72 + '\n\n').encode()
        parts.extend([heading, entry['content'], b'\n\n'])
    combined = b''.join(parts)
    # Verify preservation independently of the reference table.
    for path in licenses:
        assert path.read_bytes().replace(b'\r\n', b'\n') in combined
    notices = (root / 'THIRD_PARTY_NOTICES.md').read_text(encoding='utf-8')
    def replace_link(match):
        label, target = match.groups()
        if target in sections:
            return '[' + label + '，第 ' + str(sections[target]) + ' 节](THIRD_PARTY_LICENSES.txt)'
        return match.group(0)
    notices = re.sub(r'\[([^\]]+)\]\(([^)]+)\)', replace_link, notices)
    notices = notices.replace('LICENSES/GPL-3.0.txt', 'LICENSE-GPL-3.0.txt')
    notices = notices.replace('[vendor/README.md](vendor/README.md)', '对应源码归档中的 vendor/README.md')
    notices = notices.replace('下列离线文本来自依赖包', '合并后的离线许可正文来自依赖包')
    notices += '\n\n本便携包将内容完全相同的许可正文去重合并到 THIRD_PARTY_LICENSES.txt；各组件对应的节号见上表。源码仓库保留原始许可证文件和构建资料。\n'
    notice = (root / 'NOTICE').read_text(encoding='utf-8').replace('See THIRD_PARTY_NOTICES.md and vendor/README.md for third-party terms.', 'See THIRD_PARTY_NOTICES.md for third-party terms.')
    files = {
        'CPE-Monitor.exe': exe.read_bytes(),
        'LICENSE': (root / 'LICENSE').read_bytes(),
        'LICENSE-GPL-3.0.txt': (root / 'LICENSES/GPL-3.0.txt').read_bytes(),
        'NOTICE': notice.encode(),
        'THIRD_PARTY_NOTICES.md': notices.encode(),
        'THIRD_PARTY_LICENSES.txt': combined,
    }
    assert 'LICENSES/' not in notices
    return files, {'original_license_files': len(licenses), 'unique_license_texts': len(groups)}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--source-root', type=Path, default=Path(__file__).resolve().parents[1])
    parser.add_argument('--exe', type=Path)
    parser.add_argument('--output', type=Path)
    parser.add_argument('--directory', type=Path, help='Also create a new, empty staging directory')
    args = parser.parse_args()
    root = args.source_root.resolve()
    exe = args.exe or root / 'CPE-Monitor.exe'
    output = args.output or root / 'dist/CPE-Monitor-Windows-x64.zip'
    files, stats = portable_files(root, exe)
    if args.directory:
        args.directory.mkdir(parents=True, exist_ok=False)
        for name, content in files.items():
            (args.directory / name).write_bytes(content)
    output.parent.mkdir(parents=True, exist_ok=True)
    temp = output.with_name(output.name + '.tmp')
    try:
        with ZipFile(temp, 'w', ZIP_DEFLATED, compresslevel=9) as archive:
            for name, content in files.items():
                archive.writestr(name, content)
        with ZipFile(temp) as archive:
            assert archive.testzip() is None
            assert set(archive.namelist()) == set(files)
        os.replace(temp, output)
    finally:
        if temp.exists():
            temp.unlink()
    stats.update(files=len(files), unpacked_bytes=sum(map(len, files.values())), zip_bytes=output.stat().st_size)
    print(json.dumps(stats))


if __name__ == '__main__':
    main()
