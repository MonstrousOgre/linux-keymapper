# linux-keymapper

A tool that lets you remap a keyboard/keypad on Linux, written in Rust

Since this uses [evdev](https://www.freedesktop.wiki/Software/libevdev/) (which sits lower on the stack than X, Wayland, or even the tty), it should work in any environment.

---

**NOTE**

This blocks input events to other applications from the physical device you choose to remap, and redirects it through a virtual device. Any applications that rely on hardware-specific input from the original device may not function as expected.

---
