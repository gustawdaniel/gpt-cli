#!/usr/bin/env bash

URL="$(wget -qO- https://api.github.com/repos/gustawdaniel/gpt-cli/releases/latest |\
 jq -r '.assets[] | select(.name=="gpt-cli").browser_download_url')"

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
