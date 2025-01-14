# STM32-rust-ledchaser

## Purpose

The purpose of this repository is to share the sandbox I'm using to learn **embedded Rust**.
The objective is to simulate a Formula 1 steering wheel LED display.

## Installation

### Requirements

* Rust and cargo : see [Rust installation](https://www.rust-lang.org/tools/install)
* Add the appropriate target to rustup.

``` bash
rustup target add thumbv7em-none-eabihf
```

* You need VSCode on a ARM compatible OS to run this project (it works with WSL2). Recommended extensions are listed into the [extension.json file](/.vscode/extensions.json). Make sure you install them while cloning the project.

### Software

1. Clone this repository using git

    ``` bash
    git clone https://github.com/Etilam224/STM32-rust-ledchaser.git
    ```

2. [Embassy](https://github.com/embassy-rs/embassy) dependency needs to be downloaded separately as we need to specify the path.

    ``` bash
     git clone https://github.com/embassy-rs/embassy.git
    ```

    * This will locate your Embassy folder in **/..**
    * If you clone the Embassy repository elsewhere, specify your own path into the [Cargo.toml](Cargo.toml) file.

* Use cargo to download other dependencies and build the project.

    ``` bash
    cd STM32-rust-ledchaser
    cargo build
    ```

### Hardware

* NUCLEO-F722ZE board
* Some LED, resistors, jump wires and a potentiometer

### Wiring

... Work in progress ...

## Run the program

Open a terminal and use the following command to build and flash the program.

``` bash
cargo r
```

### Using WSL2

If you're using WSL2, you **must attach the USB I/O to WSL** before using the ```cargo r``` command.

1. Open a Windows Power Shell prompt and get the **ST-Link BUSID** using the following command.

    ``` powershell
    usbipd list
    ```

    The resulting format should be ```[X]-[X] yyyy:yyyy ST-Link Debug```.

2. Now run the following command where ```[X]-[X]``` is your BUSID to attach the USB I/O.

    ``` powershell
    usbipd attach --wsl --busid [X]-[X]
    ```

3. Verify by using the ```lsusb``` command on the WSL terminal. You should see something like :

    ``` bash
    Bus XXX Device XXX: ID yyyy:yyyy STMicroelectronics ST-LINK/V2.1
    ```

See [this tuto from Microsoft](https://learn.microsoft.com/en-us/windows/wsl/connect-usb) for help.

## Licence

This repository is licenced under the [BSD 2-Clause "Simplified" License](LICENCE)

## TODO

* [ ] Achieve and doc an installation from scratch to see if there is no major issue;
* [ ] Add an Embassy redirection into this repo or fix the path/features bug;
* [ ] Insert a clear wiring diagram.
