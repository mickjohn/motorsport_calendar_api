#!/bin/zsh

echo "Starting..."


START=$(date +%s)
# for x in {0..400}; curl http://localhost:8000/old/DTM &>/dev/null
for x in {0..400}; curl http://localhost:8000/DTM &>/dev/null
FINISH=$(date +%s)
TIME=$(expr $FINISH - $START)

echo "Time was $TIME"
