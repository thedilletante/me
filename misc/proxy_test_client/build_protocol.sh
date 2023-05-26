#!/usr/bin/env bash

set -x

SCRIPT_DIR=$(dirname $(readlink -f $0))
PROTO_DIR="${SCRIPT_DIR}/../../protocols/control"
GENERATED_DIR="${SCRIPT_DIR}/protocol"

mkdir -p "${GENERATED_DIR}"

python -m grpc_tools.protoc -I"${PROTO_DIR}" --python_out="${GENERATED_DIR}" --grpc_python_out="${GENERATED_DIR}" ${PROTO_DIR}/*.proto
find "${GENERATED_DIR}" -name '*_grpc.py' | xargs sed -i.back '/^import [^.]*_pb2/ s/import \(.*\)$/import protocol.\1/'
find "${GENERATED_DIR}" -name '*_pb2.py' | xargs sed -i.back '/^import [^.]*_pb2/ s/import \(.*\)$/import protocol.\1/'
