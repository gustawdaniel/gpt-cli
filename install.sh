#!/usr/bin/env bash

FALLBACK_RELEASE_URL=https://api.github.com/repos/gustawdaniel/gpt-cli/releases/96442904

# Define a function that prints a message
function download_binary {
  URL="$(wget -qO- https://api.github.com/repos/gustawdaniel/gpt-cli/releases/latest |
    jq -r '.assets[] | select(.name=="gpt-cli.gnu").browser_download_url')"

  URL_SUM="$(wget -qO- https://api.github.com/repos/gustawdaniel/gpt-cli/releases/latest |
    jq -r '.assets[] | select(.name=="gpt-cli.gnu.sha256.txt").browser_download_url')"

  if [ -z "$URL" ]; then
    echo "Fallback release"
    URL="$(wget -qO- ${FALLBACK_RELEASE_URL} |
      jq -r '.assets[] | select(.name=="gpt-cli.gnu").browser_download_url')"

    URL_SUM="$(wget -qO- ${FALLBACK_RELEASE_URL} |
      jq -r '.assets[] | select(.name=="gpt-cli.gnu.sha256.txt").browser_download_url')"
  fi

  echo "$URL"
  wget "${URL}" -O /tmp/gpt-cli.gnu
  wget "${URL_SUM}" -O /tmp/gpt-cli.gnu.sha256.txt

  # Read the expected checksum from the file
  EXPECTED_CHECKSUM=$(cat /tmp/gpt-cli.gnu.sha256.txt)

  # Compute the actual checksum of the binary
  ACTUAL_CHECKSUM=$(shasum -a 256 /tmp/gpt-cli.gnu | cut -d " " -f 1)

  # Compare the checksums and display an error message if they differ
  if [ "$EXPECTED_CHECKSUM" != "$ACTUAL_CHECKSUM" ]; then
    echo -e "\033[31mError: Checksums do not match.\033[0m"
    echo "Expected: $EXPECTED_CHECKSUM"
    echo "Actual: $ACTUAL_CHECKSUM"
    exit 1
  else
    echo -e "\033[32mChecksums match. The binary file is verified.\033[0m"
  fi
}

function compile_binary {
  rm -rf target
  cargo build --target x86_64-unknown-linux-gnu --release
  cp target/x86_64-unknown-linux-gnu/release/gpt-cli /tmp/gpt-cli.gnu
}

if [ "$1" = "--local" ]; then
  echo "Compilation started."
  compile_binary
else
  echo "Downloading started"
  download_binary
fi

cd /tmp && {
  echo "Your password is required to save binary in /usr/local/bin"
  sudo rm -f /usr/local/bin/gpt-cli /usr/local/bin/p
  sudo mv gpt-cli.gnu /usr/local/bin/gpt-cli
  sudo chmod +x /usr/local/bin/gpt-cli
  sudo ln -s /usr/local/bin/gpt-cli /usr/local/bin/p
  sudo chmod +x /usr/local/bin/p
  cd - || exit
}
