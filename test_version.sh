#! /bin/sh

testEquality() {
  RES=$(grep -E "^version" Cargo.toml | cut -d "\"" -f 2)
  EXP=$(gpt-cli --version)
  assertEquals "${EXP}" "${RES}"
}

# Load shUnit2.
. /usr/share/shunit2/shunit2