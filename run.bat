@echo off
IF NOT DEFINED CTRL_HOST SET "CTRL_HOST=raspberry.local"
IF NOT DEFINED CTRL_USER SET "CTRL_USER=pi"

SET TOOL=gpio-mqtt-bridge

echo running as %CTRL_USER%@%CTRL_HOST%

cargo build --target=armv7-unknown-linux-gnueabihf
scp config.toml %CTRL_USER%@%CTRL_HOST%:./config.toml
scp target\armv7-unknown-linux-gnueabihf\debug\%TOOL% %CTRL_USER%@%CTRL_HOST%:./%TOOL%
ssh -t %CTRL_USER%@%CTRL_HOST% "chmod +x ./%TOOL% && ./%TOOL%"
