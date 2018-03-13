# The PSVR USB HID protocol

The PSVR is controlled by the processing unit, which itself is
controlled by computer via USB.

The PSVR acts as a [USB Human Interface Device](https://en.wikipedia.org/wiki/USB_human_interface_device_class), much like
other devices like mice, keyboards or joysticks.

## Caveat: connecting to HID devices

Be wearly of the library you use to connect to HID devices like
the PSVR. Many operating systems (including Mac, Linux, and
Windows) have kernel drivers which automatically take exclusive
access to all HID devices as they are connected. This means that
the only way to connect to HID devices is to use platform-specific
HID APIs (such as [IOKit](iokit)).

There are workarounds for this. I've found that I could get it to work
on Linux if I execute my program under `root`. I've seen libraries
with special code to detach the kernel drivers from each USB interface needed.
I personally haven't had much success with that though.

Note that `libusb` also suffers from this problem.

I've found that the [HIDAPI][hidapi] library works well for this, although I had
to maintain a [custom fork](https://github.com/signal11/hidapi/pull/380) to work
around an issue on Mac.

[iokit]: https://developer.apple.com/library/content/documentation/DeviceDrivers/Conceptual/IOKitFundamentals/Introduction/Introduction.html
[libusb]: https://github.com/libusb/libusb
[hidapi]: https://github.com/signal11/hidapi
