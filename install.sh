#!/usr/bin/env bash

URL="$(curl -o- https://api.github.com/repos/gustawdaniel/gpt-cli/releases/latest | jq -r '.assets[] | select(.name=="gpt-cli_v0.0.5_x86_64-unknown-linux-musl.zip").browser_download_url')"
echo "$URL";
#curl "${URL}" --output /tmp/gpt-cli_v0.0.5_x86_64-unknown-linux-musl.zip
cd /tmp && { curl -O "${URL}" ; cd -; }


#sudo wget -q -O /usr/local/bin
#sudo ln -s /usr/local/bin/gpt-cli /usr/local/bin/p