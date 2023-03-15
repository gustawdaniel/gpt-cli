#!/usr/bin/env bash

FALLBACK_RELEASE_URL=https://api.github.com/repos/gustawdaniel/gpt-cli/releases/95689674

URL="$(wget -qO- https://api.github.com/repos/gustawdaniel/gpt-cli/releases/latest |\
 jq -r '.assets[] | select(.name=="gpt-cli").browser_download_url')"

if [ -z "$URL" ]; then
  echo "Fallback release"
  URL="$(wget -qO- ${FALLBACK_RELEASE_URL} |\
   jq -r '.assets[] | select(.name=="gpt-cli").browser_download_url')"
fi

echo "$URL";
wget "${URL}" -O /tmp/gpt-cli
cd /tmp && {
  echo "Your password is required to save binary in /usr/local/bin"
  sudo rm -f /usr/local/bin/gpt-cli /usr/local/bin/p
  sudo mv gpt-cli /usr/local/bin/gpt-cli;
  sudo chmod +x /usr/local/bin/gpt-cli;
  sudo ln -s /usr/local/bin/gpt-cli /usr/local/bin/p;
  sudo chmod +x /usr/local/bin/p;
  cd - || exit;
}
