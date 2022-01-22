#!/bin/bash
location=$(pwd)
targets=("x86_64-pc-windows-gnu" "x86_64-unknown-linux-gnu")
cd bin || exit

for i in "${targets[@]}"
do
  mkdir "${i}" || true
  rustup target add "${i}" || true
  cross build --target "${i}" --release
  cp "${location}"/target/"${i}"/release/catrina "${location}"/bin/"${i}" || cp "${location}"/target/"${i}"/release/catrina.exe "${location}"/bin/"${i}"

done

