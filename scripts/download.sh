#!/bin/bash
set -e
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null && pwd )"
cd "${SCRIPT_DIR}/.."

if [[ ! -f "download/cordis-HORIZONprojects-xml.zip" ]]; then
  mkdir download/
  cd download/
  echo "Downloading dataset..."
  wget --no-verbose https://cordis.europa.eu/data/cordis-HORIZONprojects-xml.zip
else
    echo "Already downloaded.."
fi
