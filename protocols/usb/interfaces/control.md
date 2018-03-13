# Control (HID interface 5)

This interface exposes two interrupt endpoints - an input and an output.

Commands can be written to the output endpoint.

**FIXME**: Document the input endpoint (#5).

This interface can be used to

* Power the processing unit on/off
* Change the display mode][display_mode] of the headset

## Foreword

The commands in this document are not officially documented.
These have been reverse engineered from various people across the internet.

As such, the commands listed here may be special cases of more
general commands.

For example, suppose we know of a setting to enable FOO. We
might document that here as the "enable FOO" command. In the future,
it may turn out that the command is actually a "Set FOO" command,
and the previously constant payload bytes actually includes an on/off bit.

This also means that all of the terminology, command names, and field names used are by no way official.

The goal is to have the commands listed here as general as possible.

As such, this document will change over time.

## Command -> Response architecture

Writes and reads to/from this interface use the same format.

**FIXME**: it is unclear if there is always a response to a command

In certain error cases, the processing unit will respond with
errors.

**FIXME**: Find out what errors look like. Some information
           [here](https://github.com/gusmanb/PSVRFramework/wiki/PSVR-Control:--USB-Command-Response-Format).


## Command/response frame format

This is the format of every frame that is written to and read from the control endpoint.

```
|-------------|----------------------|
|   Header    |   Payload    |
|  (4 bytes)  |  (variable length)   |
|-------------|----------------------|
```

The header looks like this

```c
struct {
    // The unique identifier of the command.
    //
    // This changes for every command type. 
    uint8_t command_id;
    // Unknown byte, 0 seems to always work.
    uint8_t unknown;
    // A constant magic number, always set to 0xAA.
    uint8_t magic;
    // The length of the payload.
    //
    // This does not include the size of the header.
    uint8_t payload_length;
} header;
```

The payload has a different format per command.

### Error responses

Commands may encounter errors. It is unclear how well the PSVR reports
errors to the host, but a few have been encountered.

These errors also follow the same frame format.

If an error is reported, it looks like this

```
|-------------|----------------------------------|
|   Header    |          Error message           |
|  (4 bytes)  |  (ASCII text, variable length)   |
|-------------|----------------------------------|
```

## Commands

Here is a list of all known PSVR commands. Each command is preceded by
its command ID.

### `0x11` Enable VR tracking

**FIXME**: Find out what this command actually does. I've always
           sent this command at the same time as "Set VR mode",
           and it seems to work well.
           
#### Command

| Field         | Type     | Description
|---------------|----------|------------
| Unknown       | `u64`    | Always `0xFFFFFF0000000000`


#### Response

Undocumented

### `0x13` Power off processing unit

This command powers off the processing unit.

#### Command

| Field         | Type     | Description
|---------------|----------|------------
| Unknown       | `u32`    | Always `1`

#### Response

Undocumented

### `0x15` Set headset lights

This sets the state of the LEDs on the outside of the headset.

**FIXME**: Document this command more. Some information can be found [here](https://github.com/gusmanb/PSVRFramework/wiki/Report-0x15---LED-Brightness).

#### Command

| Field         | Type     | Description
|---------------|----------|------------
| `light_mask`  | `u16`    | **FIXME** write this
| `values`      | 9 bytes  | **FIXME** write this + give field a better name
| Unknown       | 5 bytes  |

#### Response

Undocumented

### `0x17` Set headset power

This command is used to turn the headset on or off.

#### Command

Format

| Field         | Type     | Description
|---------------|----------|------------
| `is_on`       | `u32`    | `1` to power on headset, `0` to power off

#### Response

Undocumented

### `0x21` Set cinematic mode configuration

This command is used to set the display properties of the
[cinematic mode][cinematic mode].

**FIXME**: What happens when this is sent and the machine is not
           in cinematic mode. In fact, is this the command to enable
           cinematic mode? More investigation needed.

#### Command

Format

| Field                     | Type     | Description
|---------------------------|----------|------------
| `mask`                    | `u8`     | **FIXME** write this
| `screen_size`             | `u8`     | **FIXME** write this
| `screen_distance`         | `u8`     | **FIXME** write this
| `interpupillary_distance` | `u8`     | **FIXME** write this
| Unknown                   | 6 bytes  |
| `brightness`              | `u8`     | **FIXME** write this
| `mic_volume`              | `u8`     | **FIXME** write this
| Unknown                   | 4 bytes  |

#### Response

Undocumented

### `0x23` Set VR mode

This command is used to enable [VR mode][vr mode].

#### Command

Format

| Field             | Type     | Description
|-------------------|----------|------------
| `vr_mode_enabled` | `u32`    | `1` to enable VR mode, `0` to disable

**FIXME**: The PSVR has to be in some mode. It probably doesn't make
sense to turn off the current mode. Perhaps `0` enables cinematic mode?

#### Response

Undocumented

### `0x81` Get device information

This command asks the processing unit for information about the headset.

**FIXME**: Document this more, see information [here](https://github.com/gusmanb/PSVRFramework/wiki/Report-0x81-Device-ID-and-Calibration).

#### Command

| Field             | Type     | Description
|-------------------|----------|------------
| Unknown           | 1 byte   | Always `0x80`
| Reserved          | 7 bytes  | Always zeroed

#### Response

Undocumented

[display_mode]: /modes/README.md
[vr mode]: /modes/virtual_reality.md
[cinematic mode]: /modes/cinematic.md

