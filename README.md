# Vizuara test

A simple application made as a task for vizuara recruitment.

## Working

The application uses a simple **GTK** interface that detects and opens the underlying **WebGL**
files. These files are hosted to the local port using a simple **server** binary written using
**tiny_http**. In the frontend, the **GUI** runs the game on the port and opens the output using
**WebView** provided in the **webkit2gtk** library.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)

- [GTK](https://www.gtk.org/docs/installations/linux)

- latest iteration of 2.0 at [WebKitGtk](https://webkitgtk.org/releases/)
