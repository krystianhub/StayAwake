<p align="center">
    <img src="images/logo.jpg"
        alt="StayAwake logo" />
</p>

[![CI](https://github.com/krystianhub/StayAwake/actions/workflows/Build.yml/badge.svg?branch=master)](https://github.com/krystianhub/StayAwake/actions/workflows/Build.yml)
[![codecov](https://codecov.io/gh/krystianhub/StayAwake/branch/master/graph/badge.svg?token=S84RWLU7YA)](https://codecov.io/gh/krystianhub/StayAwake)
[![crates.io](https://img.shields.io/crates/v/stayawake.svg)](https://crates.io/crates/stayawake)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/krystianhub/StayAwake/blob/master/LICENSE)

# StayAwake

## Introduction

StayAwake is a simple program for Windows & MacOS to keep your system awake without affecting your workflow.

It is inspired by the [stay-awake](https://pypi.org/project/stay-awake/) package for Python.

The program is only triggered when you don't do any mouse movements and it is completely headless, intended to be used as a command line tool.

### How does it work?

If in a span of **15** seconds you don't move your mouse, StayAwake will automatically move your mouse for about **100** to **150** pixels randomly. There won't be any mouse displacement! If you are working, this will do absolutely nothing!

All settings like the time interval or the range of random mouse movement are fully customizable via environment variables or the **[.env](.env)** file.

## Installation

You can download the [latest version](https://github.com/krystianhub/StayAwake/releases/latest) from Github Releases.

Alternatively, you can install it via **cargo** command:

```bash
cargo install stayawake
```

## Configuration

The application ships with a default configuration, but it can be overridden by creating a **[.env](.env)** file in the same location as the binary itself.

There are 6 available configuration properties:

```properties
RUST_LOG=INFO # logging level
STAYAWAKE_INTERVAL=15 # in seconds
JUMP_BY_PIXEL_MIN=100 # in pixels
JUMP_BY_PIXEL_MAX=150 # in pixels
INIT_POINT=0x0 # in pixels, starting point of the operating window area (usually it's a top-left corner of the screen; for example, 0x0)
WORKING_AREA=1024x768 # in pixels, operating window area (usually it's a display resolution; for example, 1024x768)
```
