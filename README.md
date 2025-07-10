# Rustylight

Application to send a DMX signal over a 512 DMX interface based on scenes written in yaml that can be selected via MIDI signal.

## Show
The application is centered around a show that contians songs which contian scenes in yaml format.

## DMX
Rustylight works with devices which comply to the Enttex DMX USB Pro standard. For example the DMXIS or the DMX USB Pro. It can send up to 255 separate DMX channels in one universe.

## MIDI
The application receives incoming MIDI-data on one MIDI channel via Alsa or other system extensions. The following list of signals are used to select scenes or other properties.

### Control Changes
- Bank Select (0): will select a song
- Effect Control 1 (12): will set the tempo in BPM
- Effect Control 2 (13): will set the tempo in BPM
- All Notes Off (123): send 0 on all DMX channels until a new scene is selected
*Note* Effect Control 1 & 2 are added to extend the range of possible values up to 256

### Programm Change
Selects the current scene

### Note on & off
Planned to be used for special midi faders that can be set to a certain value in a scene yaml

## How to run
```shell
cargo run
```

## General info
The Enttec-Devices run with a ftdi-chip. You may need to install the D2XX-drivers to communicate correctly.
See: https://ftdichip.com/drivers/d2xx-drivers/

More information about the USB-specification can be found here:
- https://enttec-web-cdn.s3.ap-southeast-2.amazonaws.com/pdf/assets/70304/70304_DMX_USB_PRO_DATASHEET.pdf
- https://enttec-web-cdn.s3.ap-southeast-2.amazonaws.com/pdf/assets/70304/70304_DMX_USB_PRO_API.pdf

Or the download-section of Enttec in general:
- https://www.enttec.com/product/lighting-communication-protocols/dmx512/dmx-usb-interface/

A helpful introduction to midi messages:
- https://cmtext.indiana.edu/MIDI/chapter3_MIDI.php

## Wish list
- Introduce Artnet support
- Fix dmx beyond channel 256
- Browser support for scene creation, verification, and demoing
- Easy moving head controls
- Tempo over midi
- Fix midi input bugs
- Ease configuration
- Add a build in default show
