# The PSVR USB HID protocol

The PSVR is controlled by the processing unit, which itself is
controlled by computer via USB.

The PSVR acts as a USB device, exposing both normal and some [HID](https://en.wikipedia.org/wiki/USB_human_interface_device_class) interfaces.

## Caveat: connecting to USB HID interfaces

Be wearly of the library you use to connect to HID interfaces on USB devices like
the PSVR. Many operating systems (including Mac, Linux, and
Windows) have kernel drivers which automatically take exclusive
access to all HID interfaces as they are connected. This means that
the only way to connect to HID interfaces is to use platform-specific
HID APIs (such as [IOKit](iokit)).

There are workarounds for this. I've found that I could get it to work
on Linux if I execute my program under `root`. I've seen libraries
with special code to detach the kernel drivers from each USB interface needed.
I personally haven't had much success with that though.

Note that `libusb` also suffers from this problem.

I've found that the [HIDAPI][hidapi] library works well for this, although I had
to maintain a [custom fork](https://github.com/signal11/hidapi/pull/380) to work
around an issue on Mac.

Note that only some of the USB interfaces exposed by the PSVR are HID interfaces.
Not all HID libraries support normal USB interfaces.

[iokit]: https://developer.apple.com/library/content/documentation/DeviceDrivers/Conceptual/IOKitFundamentals/Introduction/Introduction.html
[libusb]: https://github.com/libusb/libusb
[hidapi]: https://github.com/signal11/hidapi

# USB interfaces

Every USB device exposes one or more numbered interfaces. These
interfaces can be thought of as individual IO streams. In the PSVR,
there is a USB interface for each different piece of functionality.

| Interface number | Purpose                                  | Description
|------------------|------------------------------------------|------------
| 0                | 3D audio                                 |
| 1                | Audio control                            |
| 2                | Mic audio                                |
| 3                | Chat audio                               |
| 4 (HID)          | [Sensor readouts][sensor readouts]       | Sends headset state to the computer
| 5 (HID)          | Control                                  |
| 6                | Video stream H.264                       | Probably for social screen
| 7                | Bulk in                                  |
| 8 (HID)          | Control 2                                |

[sensor readouts]: #sensor-readouts

## Sensor readouts (HID interface 4)
