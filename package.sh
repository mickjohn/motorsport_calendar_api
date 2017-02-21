#!/bin/bash
set -x
set -e

DIR="/tmp/motorsport_calendar_api_package/api"

TAR_FILE="$(pwd)/api.tar.bz2"

mkdir -p "$DIR"
mkdir "$DIR/logs"

cp -r "data/" "$DIR"
cp -r "conf.yml" "$DIR"
cp -r "run.sh" "$DIR"
cp "target/release/motorsport_calendar_api" "$DIR"

cd "$DIR"
cd ".."
tar cvfj "${TAR_FILE}" *

rm -rf "$DIR"
