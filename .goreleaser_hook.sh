#!/usr/bin/env bash

go_arch=$1
go_os=$2
project_name=$3

# Make Go -> Rust arch/os mapping
case $go_arch in
    amd64) rust_arch='x86_64' ;;
    arm64) rust_arch='aarch64' ;;
    *) echo "unknown arch: $go_arch" && exit 1 ;;
esac
case $go_os in
    linux) rust_os='linux' ;;
    darwin) rust_os='apple-darwin' rust_arch='x86_64' ;;
    windows) rust_os='windows' ;;
    *) echo "unknown os: $go_os" && exit 1 ;;
esac

# Find artifacts and uncompress in the corresponding directory
dist=$(find artifacts -name "*${rust_arch}*${rust_os}*")
echo Looking for artifacts in $(pwd)/artifacts: $dist
if [ -n "$dist" ]; then
    echo Artifacts dist - removing content of dist: $(find dist/${project_name}_${go_os}_${go_arch}*)
    rm ${dist}/* -f
    mkdir -p ./${dist};
    cp ./artifacts/*${rust_arch}*${rust_os}*/* ./${dist}
    chmod +x ./${dist}/*
fi

mkdir -p dist/${project_name}_${go_os}_${go_arch}_v1;
mkdir -p dist/${project_name}_${go_os}_${go_arch};

cp ./artifacts/*${rust_arch}*${rust_os}*/* ./dist/${project_name}_${go_os}_${go_arch}_v1
cp ./artifacts/*${rust_arch}*${rust_os}*/* ./dist/${project_name}_${go_os}_${go_arch}

chmod +x ./dist/${project_name}_${go_os}_${go_arch}_v1/*
chmod +x ./dist/${project_name}_${go_os}_${go_arch}/*