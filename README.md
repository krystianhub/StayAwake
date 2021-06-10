<p align="center">
    <img src="images/logo.jpg"
        alt="StayAwake logo" />
</p>

<p align="center">
    <a href="https://github.com/krystianhub/StayAwake/actions/workflows/Build.yml">
        <img src="https://github.com/krystianhub/StayAwake/actions/workflows/Build.yml/badge.svg"
            alt="Build status" />
    </a>
</p>

# StayAwake

## Introduction

StayAwake is a simple program for Windows & MacOS to keep your system awake without affecting your workflow.

Inspired by [stay-awake](https://pypi.org/project/stay-awake/) package for Python.

As with the Python package the program is only triggered when you don't do any mouse movements and it is completely headless (it is intended to be used as a command line tool).

### How does it work?

If in a span of **60** seconds you don't move your mouse, this program will automatically move your mouse for about **5** to **15** pixels randomly. There won't be any mouse displacement! If you are working, this will do absolutely nothing!

All settings like the time interval or the range of random mouse movement is fully customizable via environment variables or **[.env](.env)** file.

## Installation

You can download the [latest version](https://github.com/krystianhub/StayAwake/releases/latest) from Github Releases and execute the binary via terminal.

## Configuration

Binary comes with a default configuration. However, it can be overridden by creating **[.env](.env)** file in the same location as the binary itself.

There are only 4 available configuration properties:

```properties
RUST_LOG=INFO # logging level
STAYAWAKE_INTERVAL=60 # in seconds
OFFSET_PIXEL_MIN=5 # in pixels
OFFSET_PIXEL_MAX=15 # in pixels
```
