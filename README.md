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

TODO
