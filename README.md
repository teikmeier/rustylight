# Rust MIDI to DMX converter

This program works with devices which comply to the Enttex DMX USB Pro standard.
It receives incoming MIDI-data (JSON-encoded) via UDP-Socket (port 9001) and processes the signal into DMX-information sent to the connected DMX-device.

Run via:
```shell
cargo run
```

The Enttec-Devices run with a ftdi-chip. You may need to install the D2XX-drivers to communicate correctly.
See: https://ftdichip.com/drivers/d2xx-drivers/

More information about the USB-specification is to find here:
- https://enttec-web-cdn.s3.ap-southeast-2.amazonaws.com/pdf/assets/70304/70304_DMX_USB_PRO_DATASHEET.pdf
- https://enttec-web-cdn.s3.ap-southeast-2.amazonaws.com/pdf/assets/70304/70304_DMX_USB_PRO_API.pdf

Or the download-section of Enttec in general:

- https://www.enttec.com/product/lighting-communication-protocols/dmx512/dmx-usb-interface/

