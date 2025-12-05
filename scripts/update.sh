#!/bin/bash
set -ex
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
    UPDATE_DATE="$(date +"%Y-%m-%d")"
    git config --global user.name "Lucas"
    git config --global user.email "public@x3ro.de"
    git remote set-url origin https://x-access-token:${GITHUB_TOKEN}@github.com/x3ro/cordis-horizon-projects-xml

    git checkout -b "ci/${UPDATE_DATE}"
    git add ${SCRIPT_DIR}/../data/
    git commit -m "Updated on ${UPDATE_DATE}"
    git push origin "ci/${UPDATE_DATE}"

    gh pr create \
        --title "Dataset update (${UPDATE_DATE})" \
        --body "..." \
        --base main \
        --head "ci/${UPDATE_DATE}"
fi

echo "Done!"
