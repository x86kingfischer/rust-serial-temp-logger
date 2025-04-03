# Rust Serial Temp Logger

![GitHub last commit](https://img.shields.io/github/last-commit/x86kingfischer/rust-serial-temp-logger)
![GitHub repo size](https://img.shields.io/github/repo-size/x86kingfischer/rust-serial-temp-logger)
![GitHub](https://img.shields.io/github/license/x86kingfischer/rust-serial-temp-logger)
![Rust](https://img.shields.io/badge/made%20with-Rust-orange)

A lightweight command-line utility that logs temperature data from a serial device (like an Arduino with a TMP35 sensor) to a timestamped CSV file.

## Features

- Reads serial data via COM port
- Adds human-readable timestamps using `chrono`
- Saves data to a configurable CSV log file
- Easy CLI interface using `clap`

## Usage

```bash
cargo run -- --port COM4 --baud 9600 --out temp.csv

## Requirements

Rust (latest stable)
A USB-connected Arduino or serial device
A sensor like TMP35 that outputs readable temperature data