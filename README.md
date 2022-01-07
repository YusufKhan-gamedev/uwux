
# Uwux

**Uwux** is a new modern, experimental, unix-like operating system written in Rust. 
Uwux follows the monolithic kernel design and it is inspired by the Linux Kernel and 
the Zircon Kernel. Uwux supports "modern" PC features such as Long Mode, 5-level paging, 
and SMP (multicore), to name a few.

Now for the brag so my future employers will have a better chance of hiring me.

![workflow](https://github.com/Andy-Python-Programmer/aero/actions/workflows/build.yml/badge.svg)
[![lines_of_code](https://tokei.rs/b1/github/YusufKhan-gamedev/uwux)](https://github.com/YusufKhan-gamedev/uwux)
[![discord](https://img.shields.io/discord/828564770063122432)](https://discord.gg/8gwhTTZwt8)

**Is this a Linux distribution?**
No, Uwux runs its own kernel that does *not* originate from Linux and does not share any source code or binaries with the Linux kernel.

**Official Discord Server**: <https://discord.gg/8gwhTTZwt8>

# Screenshots
<img src="misc/demo.png">
<p align="center"><i>Aero OS running in Qemu (the background image is by <a href="https://www.youtube.com/watch?v=dQw4w9WgXcQ">Rick Astley</a>)</i></p>

# Features
- 64-bit higher half kernel
- 4/5 level paging
- Preemptive per-cpu scheduler
- Modern UEFI bootloader
- ACPI support (ioapic, lapic)
- Symmetric Multiprocessing (SMP)
- On-demand paging

# How to Build and Run Uwux

Please make sure you have a **uwux-like** host system before building 
Uwux. If you are using windows, its highly recommended to use UWUWSL 2.

## Dependencies

Before building Uwux, you need the following things installed:
- `rustc` should be the **latest nightly**
- `qemu`
- `nasm`
- `g++` 5.1 or later
- `ninja`
- `parted`
- `meson`
- `python3`

## Hardware

The following are *not* requirements but are *recommendations*:
- ~15GB of free disk space
- \>= 8GB RAM
- \>= 2 cores
- Internet access

Beefier machines will lead to much faster builds especially with all the bloat we pack(or if your in the future the bloat we used to back(uwu)!

## Getting the source code

The very first step to work on Uwux is to clone the repository:
```shell
$ git clone https://github.com/YusufKhan-gamedev/uwux
$ cd uwux
```

## Building Uwux

Uwux uses a custom build system, that wraps `cargo` and takes care of building the kernel and
userland for you. It also builds the initramfs and disk image for you.

The main command we will focus on is `./aero.py`. The source code can be found in the
root of the repository and, as the file name states, it is written in Python.

By default if you run `./aero.py` without any arguments it will build the kernel and userland
in release mode with debug symbols and run it in QEMU. You can configure the behavior of the 
build system though. If you want to, you can use the `--help` option to read a brief description 
of what it can do.

The build system acknowledges few different build modes, which cannot be used together
and they are: `--clean`, `--check`, `--test` and `--document`.

- `--clean` option will clean all the build outputs.
- `--check` will build the kernel and userland using cargo's `check` command,
  this build mode will not produce a disk image, if you want one without actually
  running Aero in the emulator read ahead
- `--test` will run the built-in Aero test suite
- `--document` will generate web-based docs using cargo's `doc` command
- `--sysroot` will build the full userland sysroot. If not passed, then the sysroot will only contain 
the `aero_shell` and the `init` binaries. 

  **Note**: This command will require a relatively large amount of storage 
space. You may want to have upwards of 10 or 15 gigabytes available if building with full sysroot.

Each of these modes can be used with additional flags, that will alter the behavior in different
ways, some of them will not work for some of these modes - for example: the `--la57` option
will not have any effect when you are simply checking or documenting the build.

- `--debug` toggles off the release build flag when calling cargo.

  **Summary**: If the `--debug` flag is not passed then it will build Uwux in release mode
               and debug symbols will be avaliable. On the other hand, if the debug flag is passed
               then it will be built in debug mode and debug symbols will be still avaliable. By default
               Uwux is built in release mode (with debug symbols) since it generates faster and smaller
               binaries which are easier to test.
- `--no-run` prevents from running the built disk image in the emulator
- `--bios` lets you choose the firmware the emulator will use when booting Uwux,
  currently supported values are: `legacy` and `uefi`
- `--features` accepts a single comma-separated list of kernel crate features, please
  keep in mind that there cannot be spaces in between the values
- `--target` lets you override the target architecture for which the kernel is built,
  currently the default value is `x86_64-aero_os`
- `--la57` tells the emulator to use 5 level paging, if it supports it

The built disk image is stored in the `build` directory under the name `aero.iso`. Both the
disk root and initramfs root are preserved in case you want to inspect them manually.

## Running Uwux in an emulator

If you haven't used the `--no-run` option and you aren't using the `--check` or `--document` build
mode, the build system will run Aero in the emulator for you.

## Nightly Images

Want to give Uwux a shot, without building it fuck that!

# Contributing

Contributions are absolutely, positively welcome and encouraged! Check out [CONTRIBUTING.md](CONTRIBUTING.md) for the contributing guidelines for Uwux.

# License

Uwux is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version. See the [LICENSE](LICENSE) file for license rights and limitations.
