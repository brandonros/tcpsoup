#!/bin/bash

set -e

# cleanup local files
rm -rf /tmp/diag-tunnel-server.exe
rm -rf /tmp/vehicle-tunnel-client.exe
# compile on windows PC
ssh Brandon@192.168.0.72 << EOF
:: cd to directory
cd "C:\Users\Brandon\Desktop\tcpsoup"
:: pull fresh from github
git reset --hard
git pull
:: cleanup output
del "C:\Users\Brandon\Desktop\tcpsoup\target\release\diag-tunnel-server.exe"
del "C:\Users\Brandon\Desktop\tcpsoup\target\release\vehicle-tunnel-client.exe"
:: build vehicle
cargo build --release
EOF
# copy from windows PC to local
scp Brandon@192.168.0.72:C:/Users/Brandon/Desktop/tcpsoup/target/release/diag-tunnel-server.exe /tmp
scp Brandon@192.168.0.72:C:/Users/Brandon/Desktop/tcpsoup/target/release/vehicle-tunnel-client.exe /tmp
