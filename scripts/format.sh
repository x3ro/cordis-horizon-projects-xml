#!/bin/bash
set -e
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null && pwd )"
cd "${SCRIPT_DIR}/.."

for f in $(ls ./source/project-*.xml); do
    echo "$f";
    cat "$f" | xmllint --format - > $(basename "$f");
done
