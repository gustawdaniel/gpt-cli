#!/usr/bin/env bash

FALLBACK_RELEASE_URL=https://api.github.com/repos/gustawdaniel/gpt-cli/releases/96442904

# Define a function that prints a message
function download_binary {
  BIN_SELECTOR=".assets[] | select(.name==\"gpt-cli.$([ "$PKG_MANAGER" = "apk" ] && echo "musl" || echo "gnu")\").browser_download_url"
  SUM_SELECTOR=".assets[] | select(.name==\"gpt-cli.$([ "$PKG_MANAGER" = "apk" ] && echo "musl" || echo "gnu").sha256.txt\").browser_download_url"

  URL="$(wget -qO- https://api.github.com/repos/gustawdaniel/gpt-cli/releases/latest |
    jq -r "${BIN_SELECTOR}")"

  URL_SUM="$(wget -qO- https://api.github.com/repos/gustawdaniel/gpt-cli/releases/latest |
    jq -r "${SUM_SELECTOR}")"

  if [ -z "$URL" ]; then
    echo "Fallback release"
    URL="$(wget -qO- ${FALLBACK_RELEASE_URL} |
      jq -r "${BIN_SELECTOR}")"

    URL_SUM="$(wget -qO- ${FALLBACK_RELEASE_URL} |
      jq -r "${SUM_SELECTOR}")"
  fi

  echo "$URL"
  wget "${URL}" -O /tmp/gpt-cli
  wget "${URL_SUM}" -O /tmp/gpt-cli.sha256.txt

  # Read the expected checksum from the file
  EXPECTED_CHECKSUM=$(cat /tmp/gpt-cli.sha256.txt)

  # Compute the actual checksum of the binary
  ACTUAL_CHECKSUM=$(shasum -a 256 /tmp/gpt-cli | cut -d " " -f 1)

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
  cp target/x86_64-unknown-linux-gnu/release/gpt-cli /tmp/gpt-cli
}

function set_pkg_manager {
  if [ -f /etc/os-release ]; then
    . /etc/os-release
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
    opensuse | opensuse-leap | suse)
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
    echo "Cannot determine the distribution, please check /etc/os-release"
    exit 1
  fi
}

function install_os_dependencies {
  case $PKG_MANAGER in
  apt)
    echo "Installation ${PKG_MANAGER} dependencies"
    sudo apt install jq libdigest-sha-perl libxcb1-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev -y
    ;;
  dnf)
    echo "Installation ${PKG_MANAGER} dependencies"
    sudo dnf install jq perl-Digest-SHA libxcb -y
    ;;
  yum)
    echo "Installation ${PKG_MANAGER} dependencies"
    sudo yum install jq perl-Digest-SHA libxcb -y
    ;;
  zypper)
    echo "Installation ${PKG_MANAGER} dependencies"
    sudo zypper --non-interactive install jq perl-App-cpanminus
    ;;
  pacman)
    echo "Installation ${PKG_MANAGER} dependencies"
    sudo pacman -Syu --noconfirm jq libxcb perl
    PATH="${PATH:+${PATH}:}/usr/bin/core_perl"
    ;;
  apk)
    echo "Installation ${PKG_MANAGER} dependencies"
    sudo apk add jq libxcb perl-utils
    ;;
  esac
}

if [ "$1" = "--local" ]; then
  echo "Compilation started."
  compile_binary
else
  set_pkg_manager
  install_os_dependencies
  echo "Downloading started"
  download_binary
fi

cd /tmp && {
  echo "Your password is required to save binary in /usr/local/bin"
  sudo rm -f /usr/local/bin/gpt-cli /usr/local/bin/p
  sudo mv gpt-cli /usr/local/bin/gpt-cli
  sudo chmod +x /usr/local/bin/gpt-cli
  sudo ln -s /usr/local/bin/gpt-cli /usr/local/bin/p
  sudo chmod +x /usr/local/bin/p
  cd - || exit
}
