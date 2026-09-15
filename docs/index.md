# Introduction

Welcome to the **`wgc`** tutorial!

`wgc` is a simple, ergonomic, and high-performance Rust wrapper for the **Windows Graphics Capture (WGC)** API (`Windows.Graphics.Capture`). It allows Rust developers to easily capture windows or entire monitors on Windows 10 and 11.

## Key Features

- **Realtime & AI-Optimized**: High-performance frame capture suitable for streaming, computer vision, and machine learning pipelines.
- **Ergonomic Iterator API**: Process frames sequentially with standard Rust iterator patterns (`Wgc`).
- **Interactive Picker & Explicit Handles**: Pick target windows or monitors using the native Windows UI picker or construct targets explicitly from window handles (`HWND`) or monitor handles (`HMONITOR`).
- **Configurable Formats & Letterboxing**: Supports `RGBA8` and `BGRA8` pixel formats, along with automatic resolution scaling and letterboxing (`pixels_fitted`).
- **Zero-Copy & Direct3D Access**: Direct access to underlying DirectX/Direct3D 11 surface textures and zero-copy frame handling.

## System Requirements

- **Operating System**: Windows 10 October 2018 Update (version 1809 / build 17763) or later. Windows 11 is recommended.
- **Rust Toolchain**: Rust 2024 edition (or compatible Rust compiler toolchain).
- **Target Platform**: `x86_64-pc-windows-msvc` or `aarch64-pc-windows-msvc`.

In the following chapters, you will learn how to set up `wgc`, configure capture options, capture frames, and build real-world screen capture applications.
