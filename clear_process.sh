#!/usr/bin/bash

PORT=$(cat SERVE_PORT)

PID1=$(lsof -i:$PORT | awk 'NR > 1 {print substr($2, 1, 6)}')

if [ -z "$PID1" ]; then
    echo "No processes to kill in port $PORT";
else
    echo "Cleanup on aisle $PORT";
    for pid in $PID1; do
        kill $pid
        echo "Process $pid killed"
    done
fi
