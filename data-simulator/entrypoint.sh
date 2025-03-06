#!/bin/bash

FILE_TO_STREAM=${FILE_TO_STREAM:-/file_to_stream}

if [[ ! -f "$FILE_TO_STREAM" ]]; then
	echo "Error: File $FILE_TO_STREAM not found."
	exit 1
fi

PORT=${PORT:-8000}
echo "Streaming $FILE_TO_STREAM on port $PORT..."

RATE=${RATE:-1M}
PORT=${PORT:-8000}

# Start netcat to stream the file
cat "$FILE_TO_STREAM" | pv -L "$RATE" | nc -l -p "$PORT"
