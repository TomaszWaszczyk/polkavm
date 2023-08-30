#!/bin/bash

set -euo pipefail

mkdir -p linux
cd linux
wget -c "https://cdn.kernel.org/pub/linux/kernel/v6.x/linux-6.5.tar.xz"
tar -xf linux-6.5.tar.xz
cd linux-6.5
make headers_install ARCH=x86_64 INSTALL_HDR_PATH=../linux-6.5-headers
