#!/bin/sh

set -e
cd server
ORT_DYLIB_PATH=../libs/onnxruntime.dylib cargo tauri build
