#!/usr/bin/env bash

# Ensure script stops on error
set -euo pipefail

FALLBACK_RELEASE_URL=https://api.github.com/repos/gustawdaniel/gpt-cli/releases/96442904

function set_pkg_manager {
  if [ "$(uname)" = "Darwin" ]; then
    OS_TYPE="macos"
    PKG_MANAGER="brew"
  elif [ -f /etc/os-release ]; then
    . /etc/os-release
    OS_TYPE="linux"
    case $ID in
      debian | ubuntu | raspbian)
        PKG_MANAGER="apt"
        ;;
      fedora)
        PKG_MANAGER="dnf"
        ;;
      centos | rhel)
        PKG_MANAGER="yum"
        ;;
      opensuse* | suse)
        PKG_MANAGER="zypper"
        ;;
      arch | artix | manjaro)
        PKG_MANAGER="pacman"
        ;;
      alpine)
        PKG_MANAGER="apk"
        ;;
      *)
        echo "Unknown distribution, cannot determine the package manager"
        exit 1
        ;;
    esac
  else
    echo "Cannot determine OS. Please check manually."
    exit 1
  fi
}

function install_os_dependencies {
  case $PKG_MANAGER in
    apt)
      sudo apt update && sudo apt install jq libdigest-sha-perl libxcb1-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev -y
      ;;
    dnf)
      sudo dnf install jq perl-Digest-SHA libxcb -y
      ;;
    yum)
      sudo yum install jq perl-Digest-SHA libxcb -y
      ;;
    zypper)
      sudo zypper --non-interactive install jq perl-App-cpanminus xorg-x11-util-devel libxcb-composite0 libxcb-render0 libxcb-shape0 libxcb-xfixes0
      ;;
    pacman)
      sudo pacman -Syu --noconfirm jq libxcb perl
      PATH="${PATH:+${PATH}:}/usr/bin/core_perl"
      ;;
    apk)
      sudo apk add jq libxcb perl-utils
      ;;
    brew)
      brew install jq || true
      ;;
  esac
}

function download_binary {
  BIN_TYPE="$([ "$PKG_MANAGER" = "apk" ] && echo "musl" || echo "gnu")"
  [ "$OS_TYPE" = "macos" ] && BIN_TYPE="macos"

  BIN_SELECTOR=".assets[] | select(.name==\"gpt-cli.$BIN_TYPE\").browser_download_url"
  SUM_SELECTOR=".assets[] | select(.name==\"gpt-cli.$BIN_TYPE.sha256.txt\").browser_download_url"

  URL="$(wget -qO- https://api.github.com/repos/gustawdaniel/gpt-cli/releases/latest | jq -r "$BIN_SELECTOR")"
  URL_SUM="$(wget -qO- https://api.github.com/repos/gustawdaniel/gpt-cli/releases/latest | jq -r "$SUM_SELECTOR")"

  if [ -z "$URL" ]; then
    echo "Using fallback release"
    URL="$(wget -qO- ${FALLBACK_RELEASE_URL} | jq -r "$BIN_SELECTOR")"
    URL_SUM="$(wget -qO- ${FALLBACK_RELEASE_URL} | jq -r "$SUM_SELECTOR")"
  fi

  echo "Downloading $URL"
  wget "${URL}" -O /tmp/gpt-cli
  wget "${URL_SUM}" -O /tmp/gpt-cli.sha256.txt

  EXPECTED_CHECKSUM=$(cat /tmp/gpt-cli.sha256.txt)
  ACTUAL_CHECKSUM=$(shasum -a 256 /tmp/gpt-cli | cut -d " " -f 1)

  if [ "$EXPECTED_CHECKSUM" != "$ACTUAL_CHECKSUM" ]; then
    echo -e "\033[31mError: Checksums do not match.\033[0m"
    echo "Expected: $EXPECTED_CHECKSUM"
    echo "Actual:   $ACTUAL_CHECKSUM"
    exit 1
  else
    echo -e "\033[32mChecksums match. Binary verified.\033[0m"
  fi
}

function compile_binary {
  rm -rf target
  if [ "$OS_TYPE" = "macos" ]; then
    cargo build --release
    cp target/release/gpt-cli /tmp/gpt-cli
  else
    cargo build --target x86_64-unknown-linux-gnu --release
    cp target/x86_64-unknown-linux-gnu/release/gpt-cli /tmp/gpt-cli
  fi
}

# Main
if [ "${1:-}" = "--local" ]; then
  echo "Compilation started."
  set_pkg_manager
  compile_binary
else
  set_pkg_manager
  install_os_dependencies
  echo "Downloading started"
  download_binary
fi

cd /tmp && {
  echo "Your password is required to install the binary"
  sudo rm -f /usr/local/bin/gpt-cli /usr/local/bin/p
  sudo cp gpt-cli /usr/local/bin/gpt-cli
  sudo chmod +x /usr/local/bin/gpt-cli
  sudo ln -s /usr/local/bin/gpt-cli /usr/local/bin/p
  sudo chmod +x /usr/local/bin/p
  cd - || exit
}
