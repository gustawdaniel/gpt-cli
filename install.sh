#!/usr/bin/env bash

URL="$(wget -qO- https://api.github.com/repos/gustawdaniel/gpt-cli/releases/latest |\
 jq -r '.assets[] | select(.name | startswith("gpt-cli") and endswith("tar.gz")).browser_download_url')"

echo "$URL";
wget "${URL}" -O /tmp/gtp-cli.tar.gz
cd /tmp && {
  tar -zxvf gtp-cli.tar.gz;
  sudo rm /usr/local/bin/gpt-cli /usr/local/bin/p
  sudo mv gpt-cli /usr/local/bin/gpt-cli;
  sudo ln -s /usr/local/bin/gpt-cli /usr/local/bin/p;
  cd - || exit;
}
