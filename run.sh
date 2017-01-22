#!/bin/bash

log_level="info" # error|warning|info|debug
rocket_env="dev" # dev|stage|prod

server="/home/mick/Programs/rust/motorsport_calander_api/target/release/motorsport_calander_api"
logdir="/home/mick/Programs/rust/motorsport_calander_api/logs"

echo "Starting..."
RUST_LOG="$log_level"      \
  ROCKET_ENV="$rocket_env" \
  $server                  \
  > "${logdir}/stdout.log" \
  2> "${logdir}/stderr.log"\
  &

echo "Server starting in background"

