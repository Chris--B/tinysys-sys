#!/bin/bash

set -ex
./scripts/01-update_c_sdk.sh
./scripts/02-build_c_sdk.sh
./scripts/03-update_rs.sh
