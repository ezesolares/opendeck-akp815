![Plugin Icon](assets/icon.png)

# OpenDeck Ajazz AKP815 Plugin

An unofficial plugin for the Ajazz AKP815 device.

## OpenDeck version

Requires OpenDeck 2.5.0 or newer

## Supported devices

- Ajazz AKP815 (5548:6672)

## Platform support

- Linux: Guaranteed.
- Mac: Best effort.
- Windows: Zero effort.

## Installation

1. Download an archive from [releases](https://github.com/4ndv/opendeck-akp815/releases)
2. In OpenDeck: Plugins -> Install from file
3. Linux: Download [udev rules](./40-opendeck-akp815.rules) and install them by copying into `/etc/udev/rules.d/` and running `sudo udevadm control --reload-rules`
4. Unplug and plug again the device, restart OpenDeck
4. Unplug and plug again the device, restart OpenDeck

## Known issues

- All the "old" devices come with the same serial number. You cannot use two of the same devices at the same time (for example a pair of 153R-s), but you can use two different devices at the same time (for example a 153R and a 153E)

## AKP815 Turn off / On (on Linux on sleep / restart)

- Tested on Fedora 43

File to create in

```sh
sudo nano /usr/lib/systemd/system-sleep/opendeck.sh
```

```sh
#!/bin/bash

USERNAME="your-username"
USER_ID=$(id -u "$USERNAME")
GROUP_ID=$(id -g "$USERNAME")

case "$1" in
  pre)

    pkill -f "opendeck"
    ;;
  post)
    sleep 2
    systemd-run --uid=$USER_ID --gid=$GROUP_ID \
      --setenv=DISPLAY=:0 \
      --setenv=DBUS_SESSION_BUS_ADDRESS=unix:path=/run/user/${USER_ID}/bus \
      --setenv=XDG_RUNTIME_DIR=/run/user/${USER_ID} \
      opendeck
    ;;
esac

```

Test
```sh
# Simulate pre-sleep
sudo /usr/lib/systemd/system-sleep/opendeck.sh pre suspend

# Simulate post-resume
sudo /usr/lib/systemd/system-sleep/opendeck.sh post suspend
```

## Building

### Prerequisites

You'll need:

- A Linux OS of some sort
- Rust 1.87 and up with `x86_64-unknown-linux-gnu` and `x86_64-pc-windows-gnu` targets installed
- Docker
- [just](https://just.systems)

### Preparing environment

```sh
$ just prepare
```

This will build docker image for macOS crosscompilation

### Building a release package

```sh
$ just package
```

## Acknowledgments

This plugin is heavily based on work by contributors of [elgato-streamdeck](https://github.com/streamduck-org/elgato-streamdeck) crate
