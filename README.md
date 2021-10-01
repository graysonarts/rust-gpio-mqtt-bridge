![GitHub Workflow Status](https://img.shields.io/github/workflow/status/graysonarts/rust-gpio-mqtt-bridge/ci?style=for-the-badge) [![GitHub issues](https://img.shields.io/github/issues/graysonarts/rust-gpio-mqtt-bridge?style=for-the-badge)](https://github.com/graysonarts/rust-gpio-mqtt-bridge/issues) [![GitHub license](https://img.shields.io/github/license/graysonarts/rust-gpio-mqtt-bridge?style=for-the-badge)](https://github.com/graysonarts/rust-gpio-mqtt-bridge/blob/main/LICENSE) ![Requires.io](https://img.shields.io/requires/github/graysonarts/rust-gpio-mqtt-bridge?style=for-the-badge)
![Gitmoji](https://img.shields.io/badge/gitmoji-%20😜%20😍-FFDD67.svg?style=for-the-badge)

<!-- ALL-CONTRIBUTORS-BADGE:START - Do not remove or modify this section -->

[![All Contributors](https://img.shields.io/badge/all_contributors-1-orange.svg?style=for-the-badge)](#contributors)

<!-- ALL-CONTRIBUTORS-BADGE:END -->

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

## CONTRIBUTING

We use [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/) and [conventional comments](https://conventionalcomments.org/)

More details coming later.

## Contributors ✨

Thanks goes to these wonderful people ([emoji key](https://allcontributors.org/docs/en/emoji-key)):

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tr>
    <td align="center"><a href="https://graysonarts.com/"><img src="https://avatars.githubusercontent.com/u/94549?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Grayson Hay</b></sub></a><br /><a href="https://github.com/graysonarts/rust-gpio-mqtt-bridge/commits?author=graysonarts" title="Code">💻</a> <a href="https://github.com/graysonarts/rust-gpio-mqtt-bridge/commits?author=graysonarts" title="Documentation">📖</a></td>
  </tr>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind welcome!
