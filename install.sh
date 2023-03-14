#!/usr/bin/env bash

sudo wget -q -O /usr/local/bin "$(wget -q -O - 'https://api.github.com/repos/gustawdaniel/gpt-cli/releases/latest' | jq -r '.assets[] | select(.name=="gpt-cli").browser_download_url')"
sudo ln -s /usr/local/bin/gpt-cli /usr/local/bin/p