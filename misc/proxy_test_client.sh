#!/usr/bin/env bash

# This script is used to test the proxy server.
# Requirements:
# * grpcurl
# * jq

PROXY_ENDPOINT="127.0.0.1:7777"

# find-out the absolute path to this script file
SCRIPT_DIR=$(dirname $(readlink -f $0))

SESSION_ID=$(grpcurl -import-path "${SCRIPT_DIR}/../protocols/control" -proto proxy.proto -d '{}' -plaintext "${PROXY_ENDPOINT}" proxy.Proxy/CreateSession | jq ".sessionId" | tr -d '"')
echo "Session ID: ${SESSION_ID}"

SDP=$(cat "${SCRIPT_DIR}/offer.sdp")
# Send multi-line SDP value in JSON
grpcurl -import-path "${SCRIPT_DIR}/../protocols/control" -proto proxy.proto -d "{\"sessionId\": \"${SESSION_ID}\", \"sdp\": \"${SDP}\"}" -plaintext "${PROXY_ENDPOINT}" proxy.Proxy/ProcessOffer
