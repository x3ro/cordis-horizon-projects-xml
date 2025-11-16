#!/bin/bash
set -e
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null && pwd )"
cd "${SCRIPT_DIR}/.."

mkdir -p source/
cd source/

if [[ ! -f "cordis-HORIZONprojects-xml.zip" ]]; then
    wget https://cordis.europa.eu/data/cordis-HORIZONprojects-xml.zip
else
    echo "Already downloaded.."
fi

# -q[uiet] -o[verride]
unzip -q -o cordis-HORIZONprojects-xml.zip
echo "Unzipped.."

${SCRIPT_DIR}/format.sh
echo "Formatted.."

if [[ `git status --porcelain=1 | wc -l` -eq 0 ]]; then
    echo "No changes found, not committing.."
else
    git config --global user.name 'Lucas'
    git config --global user.email 'x3ro@users.noreply.github.com'
    git add data/
    git commit -m "Updated on $(date +"%Y-%m-%dT%H:%M:%S%z")"
    git push
fi

echo "Done!"
