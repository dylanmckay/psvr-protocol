# Sensor readouts (HID interface 4)

This interface exposes a single interrupt input endpoint. As such,
this interface can only be read from.

This USB interface can be read from to get information about the headset's sensors.

This includes information such as:

* Measurements from the inertia sensor ([datasheet](https://ae-bst.resource.bosch.com/media/_tech/media/datasheets/BST-BMI055-DS000-08.pdf))
* The state of the volume up/down/mute buttons
* The current volume
* Whether the headset currently being worn

## Frame format

The frame format looks like this

```c
struct {
  // Information about the headset's buttons.
  struct {
    uint8_t reserved:1;
    // Set if the volume up button is pressed.
    uint8_t plus:1;
    // Set if the volume down button is pressed.
    uint8_t minus:1;
    // Set if the mute button is pressed.
    uint8_t mute:1;
  } button;
  uint8_t reserved0;
  // The volume, from 0-100.
  uint8_t volume;
  uint8_t reserved1[5];
  // Information about the current status of the headset itself.
  struct {
    // Set if the headset is currently on a head.
    uint8_t worn:1;
    uint8_t display_active:1;
    // Set if the HDMI cable disconnected.
    uint8_t hdmi_disconnected:1;
    // Set if the microphone is muted.
    uint8_t microphone_muted:1;
    // Set if headphones are connected to the headset.
    uint8_t headphone_connected:1;
    uint8_t reserved:2;
    uint8_t tick:1;
  } status;
  uint8_t reserved2[11];
  // Two readouts from the BMI055 IMU at different time instants.
  //
  // The integer values within are between -32768 and 32767, i.e.
  // they can take the entire range of a 16-bit signed int.
  //
  // These values can be converted to [-1.0, +1.0] floats by
  // casting to float and simply dividing by 32768.
  //
  // FIXME: Find out the time delta between these two measurements,
  //        document it.
  struct {
    // Gyroscope readings.
    //
    // These values can be preprocessed by
    //
    // (float(value) / 32768.0) * 2000.0 * DEGTORAD
    //
    // Where DEGTORAD = (PI/180.0).
    struct {
      int16_t yaw;
      int16_t pitch;
      int16_t roll;
    } gyro;
    // Accelerometer readings.
    //
    // IMPORTANT: These readings must be left-shifted by four bits.
    // The BMI055 accelerometer only has 12-bits resolution, and
    // so the bits that would always be zero have been rightshifted
    // out by the processing unit.
    //
    // The components of these readings can be preprocessed like so:
    //
    // float(-value << 4) / 32768.0
    struct  {
      int16_t x;
      int16_t y;
      int16_t z;
    } accel;
    uint8_t reserved[4];
  } data[2];
  uint8_t reserved3[12];
} frame;
```

