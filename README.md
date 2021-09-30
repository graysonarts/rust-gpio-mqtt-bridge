[![Continuous Integration](https://github.com/graysonarts/rust-gpio-mqtt-bridge/actions/workflows/ci.yml/badge.svg)](https://github.com/graysonarts/rust-gpio-mqtt-bridge/actions/workflows/ci.yml) [![GitHub issues](https://img.shields.io/github/issues/graysonarts/rust-gpio-mqtt-bridge?style=for-the-badge)](https://github.com/graysonarts/rust-gpio-mqtt-bridge/issues) [![GitHub license](https://img.shields.io/github/license/graysonarts/rust-gpio-mqtt-bridge?style=for-the-badge)](https://github.com/graysonarts/rust-gpio-mqtt-bridge/blob/main/LICENSE) ![Requires.io](https://img.shields.io/requires/github/graysonarts/rust-gpio-mqtt-bridge?style=for-the-badge)

## Cross-Compile on Windows

Follow [these instructions](https://s2e-systems.github.io/Rust-RPi4-Windows-Cross-Compilation/) to setup rust to
cross compile from windows to raspberry pi.

## Running

1. set the environment variable `CTRL_HOST` to the hostname of your raspberry pi. If not set, we default to "raspberry". It can also be an IP address.
2. set the environment variable `CTRL_USER` to the username of your main user on your pi. If not set, we default to `pi`
3. run `run.bat` or `run.sh`

This runs the debug version, so it's not optimized. It will build, scp the file over, and then run it.
Make sure you can ssh into your pi without needing to type in a password by either using a ssh key agent
or having an unencrypted ssh key.

## Deployment

Same as running, but use the deploy script

## CONTRIBUTORS


## CONTRIBUTING

We use [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/) and [conventional comments](https://conventionalcomments.org/)

More details coming later.
