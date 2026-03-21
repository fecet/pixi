# pyright: basic
#!/usr/bin/env python3
"""Clean up CI workflow files for fork usage after rebasing from upstream.

Run after `git rebase upstream/main`:
- Remove jobs targeting macOS/Windows (namespace-profile or standard runners, + cascade)
- Filter matrix strategies to keep only Linux entries
- Replace namespace-profile-ubuntu-* with public GitHub-hosted runners
- Clean up actionlint.yml, dist-workspace.toml, release.yml, and called workflows
"""

import argparse
import re
from io import StringIO
from pathlib import Path

import tomlkit
from ruamel.yaml import YAML

ROOT = Path(__file__).resolve().parent.parent
WORKFLOWS = ROOT / ".github" / "workflows"

UBUNTU_RE = re.compile(r"namespace-profile-ubuntu-(\d+)-(\d+)-x86-64-\d+")

CUSTOM_RUNNERS = {
    "8core_ubuntu_latest_runner": "ubuntu-latest",
    "16core_windows_latest_runner": "windows-latest",
    "windows_arm64_2025_large": "windows-latest",
    "windows_x64_2025_large": "windows-latest",
}

KEEP_PLATFORMS = {"linux"}

KEEP_TARGETS = {
    "aarch64-unknown-linux-musl",
    "x86_64-unknown-linux-musl",
}

RELEASE_REMOVE_STEPS = [
    re.compile(r"Unpack binaries zip"),
    re.compile(r"Upload unpacked artifact for pixi-(aarch64|x86_64)-(apple-darwin|pc-windows)"),
    re.compile(r"Upload unpacked artifact for pixi-riscv64"),
    re.compile(r"Upload install\.ps1"),
]


def _make_yaml() -> YAML:
    y = YAML()
    y.preserve_quotes = True
    y.width = 4096
    # GitHub Actions style: 2-space mapping, 4-space sequence, dash at offset 2
    y.indent(mapping=2, sequence=4, offset=2)
    return y


def _yaml_round_trip(text: str) -> tuple[dict, YAML]:
    """Load YAML text, auto-detecting indent. Returns (data, configured yaml instance)."""
    from ruamel.yaml.util import load_yaml_guess_indent

    y = _make_yaml()
    data, ind, bsi = load_yaml_guess_indent(text, yaml=y)
    if ind is not None and bsi is not None:
        y.indent(mapping=2, sequence=ind, offset=bsi)
    return data, y


def _yaml_dumps(data, y: YAML) -> str:
    buf = StringIO()
    y.dump(data, buf)
    return buf.getvalue().rstrip("\n") + "\n"


PLATFORM_KEYWORDS = [
    ("ubuntu", "linux"),
    ("linux", "linux"),
    ("macos", "macos"),
    ("apple", "macos"),
    ("windows", "windows"),
]


def _detect_platform(runner: str) -> str | None:
    """Detect platform from runner string. Returns platform name or None."""
    r = runner.lower()
    for keyword, platform in PLATFORM_KEYWORDS:
        if keyword in r:
            return platform
    return None


def _replace_runner(runner: str) -> str | None:
    """Return replacement runner, or None to signal job removal."""
    platform = _detect_platform(runner)
    if platform is not None and platform not in KEEP_PLATFORMS:
        return None
    if runner.startswith("namespace-profile-"):
        return UBUNTU_RE.sub(r"ubuntu-\1.\2", runner)
    return runner


def _normalize_needs(needs) -> list[str]:
    if needs is None:
        return []
    if isinstance(needs, str):
        return [needs]
    return list(needs)


def _filter_matrix(job: dict) -> None:
    """Remove non-Linux entries from matrix strategy (os list and include entries)."""
    strategy = job.get("strategy")
    if not strategy:
        return
    matrix = strategy.get("matrix")
    if not matrix:
        return

    # Filter plain os list: e.g. os: [ubuntu-latest, macos-15]
    if "os" in matrix:
        matrix["os"] = [o for o in matrix["os"] if _detect_platform(o) in KEEP_PLATFORMS]

    # Filter include entries: e.g. include: [{os: ubuntu-latest, ...}, {os: macos-14, ...}]
    if "include" in matrix:
        matrix["include"] = [
            entry
            for entry in matrix["include"]
            if _detect_platform(entry.get("os", "")) in KEEP_PLATFORMS
        ]


def _commit(path: Path, text: str, original: str, label: str, msg: str, dry_run: bool) -> None:
    if text == original:
        print(f"  {label}: no changes needed")
    else:
        print(f"  {label}: {msg}")
        if not dry_run:
            path.write_text(text)


def process_ci_yml(dry_run: bool) -> None:
    path = WORKFLOWS / "ci.yml"
    if not path.exists():
        return

    original = path.read_text()
    data, y = _yaml_round_trip(original)
    jobs = data.get("jobs", {})

    # Phase 1: mark jobs for removal by runner
    to_remove: set[str] = set()
    for name, job in jobs.items():
        runner = job.get("runs-on", "")
        if isinstance(runner, str) and _replace_runner(runner) is None:
            to_remove.add(name)

    # Phase 2: cascade — remove jobs whose ALL needs are removed
    changed = True
    while changed:
        changed = False
        for name, job in jobs.items():
            if name in to_remove:
                continue
            needs = _normalize_needs(job.get("needs"))
            if needs and all(n in to_remove for n in needs):
                to_remove.add(name)
                changed = True

    for name in to_remove:
        del jobs[name]

    # Fix needs references and replace runners in remaining jobs
    for job in jobs.values():
        needs = _normalize_needs(job.get("needs"))
        if needs:
            remaining = [n for n in needs if n not in to_remove]
            if not remaining:
                del job["needs"]
            elif len(remaining) == 1:
                job["needs"] = remaining[0]
            else:
                job["needs"] = remaining

        runner = job.get("runs-on", "")
        if isinstance(runner, str):
            new_runner = _replace_runner(runner)
            if new_runner and new_runner != runner:
                job["runs-on"] = new_runner

        _filter_matrix(job)

    msg = (
        f"removed {len(to_remove)} jobs: {', '.join(sorted(to_remove))}"
        if to_remove
        else "replaced runners"
    )
    _commit(path, _yaml_dumps(data, y), original, "ci.yml", msg, dry_run)


def process_actionlint(dry_run: bool) -> None:
    path = ROOT / ".github" / "actionlint.yml"
    if not path.exists():
        return

    original = path.read_text()
    data, y = _yaml_round_trip(original)

    if "self-hosted-runner" in data:
        del data["self-hosted-runner"]

    _commit(
        path,
        _yaml_dumps(data, y),
        original,
        "actionlint.yml",
        "removed self-hosted-runner config",
        dry_run,
    )


def process_dist_workspace(dry_run: bool) -> None:
    path = ROOT / "dist-workspace.toml"
    if not path.exists():
        return

    original = path.read_text()
    doc = tomlkit.parse(original)
    dist = doc.get("dist", {})

    if "targets" in dist:
        dist["targets"] = [t for t in dist["targets"] if t in KEEP_TARGETS]

    for key in ["windows-archive", "macos-sign", "macos-notarize"]:
        if key in dist:
            del dist[key]

    if "installers" in dist:
        dist["installers"] = [i for i in dist["installers"] if i != "msi"]

    if "allow-dirty" in dist:
        dist["allow-dirty"] = [d for d in dist["allow-dirty"] if d != "msi"]

    if "github-custom-runners" in dist:
        new_runners = tomlkit.table()
        new_runners.add(tomlkit.comment("Linux targets - using official GitHub runners"))
        for target in sorted(KEEP_TARGETS):
            new_runners.add(target, "ubuntu-latest")
        dist["github-custom-runners"] = new_runners

    _commit(
        path,
        tomlkit.dumps(doc),
        original,
        "dist-workspace.toml",
        "removed non-Linux targets and runners",
        dry_run,
    )


def process_called_workflows(dry_run: bool) -> None:
    """Filter matrix entries in reusable workflow files (workflow_call)."""
    for path in sorted(WORKFLOWS.glob("*.yml")):
        original = path.read_text()
        data, y = _yaml_round_trip(original)

        on = data.get("on", {})
        if not isinstance(on, dict) or "workflow_call" not in on:
            continue

        changed = False
        for job in data.get("jobs", {}).values():
            before = _yaml_dumps(job, y)
            _filter_matrix(job)
            if _yaml_dumps(job, y) != before:
                changed = True

        if changed:
            _commit(
                path,
                _yaml_dumps(data, y),
                original,
                path.name,
                "filtered non-Linux matrix entries",
                dry_run,
            )


def process_release_yml(dry_run: bool) -> None:
    path = WORKFLOWS / "release.yml"
    if not path.exists():
        return

    original = path.read_text()
    data, y = _yaml_round_trip(original)

    for job in data.get("jobs", {}).values():
        runner = job.get("runs-on", "")
        if isinstance(runner, str):
            new = CUSTOM_RUNNERS.get(runner, runner)
            new = UBUNTU_RE.sub(r"ubuntu-\1.\2", new)
            if new != runner:
                job["runs-on"] = new

        steps = job.get("steps")
        if steps:
            filtered = [
                s
                for s in steps
                if not any(p.search(s.get("name", "")) for p in RELEASE_REMOVE_STEPS)
            ]
            if len(filtered) != len(steps):
                job["steps"] = filtered

    _commit(
        path, _yaml_dumps(data, y), original, "release.yml", "cleaned up runners and steps", dry_run
    )


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--dry-run", action="store_true", help="Print changes without writing")
    args = parser.parse_args()

    print("Cleaning up CI for fork usage...")
    process_ci_yml(args.dry_run)
    process_actionlint(args.dry_run)
    process_dist_workspace(args.dry_run)
    process_release_yml(args.dry_run)
    process_called_workflows(args.dry_run)

    if args.dry_run:
        print("(dry run — no files written)")

    print("\nChecking for remaining namespace-profile references...")
    found = False
    for yml in sorted(WORKFLOWS.glob("*.yml")):
        for i, line in enumerate(yml.read_text().splitlines(), 1):
            if "namespace-profile" in line and not line.strip().startswith("#"):
                print(f"  {yml.relative_to(ROOT)}:{i}: {line.strip()}")
                found = True
    if not found:
        print("  All clean!")


if __name__ == "__main__":
    main()
