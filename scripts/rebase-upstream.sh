#!/bin/bash
set -euo pipefail

# Rebase onto upstream/main, automatically handling fork CI cleanup.
#
# The CI cleanup commit (always on top) is dropped before rebase
# and regenerated afterward by fork_ci_cleanup.py.

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
CLEANUP_MARKER="[fork-ci-cleanup]"

# Find and drop the CI cleanup commit if it's on top
if git log -1 --format=%B | grep -qF "$CLEANUP_MARKER"; then
    echo "Dropping CI cleanup commit..."
    git reset --hard HEAD~1
else
    echo "No CI cleanup commit on top, proceeding..."
fi

# Fetch and rebase
git fetch upstream
echo "Rebasing onto upstream/main..."
git rebase upstream/main

# Regenerate CI cleanup
echo "Regenerating CI cleanup..."
python "$SCRIPT_DIR/fork_ci_cleanup.py"

# Commit if there are changes
if ! git diff --quiet .github/ 2>/dev/null; then
    git add .github/
    git commit -m "ci: apply fork cleanup

$CLEANUP_MARKER"
    echo "CI cleanup commit created."
else
    echo "No CI changes needed."
fi
