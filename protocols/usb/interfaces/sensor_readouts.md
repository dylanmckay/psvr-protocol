# Sensor readouts (HID interface 4)

This interface exposes a single interrupt input endpoint. As such,
this interface can only be read from.

This USB interface can be read from to get information about the headset's sensors.

This includes information such as:

* Measurements from the inertia sensor ([datasheet](https://ae-bst.resource.bosch.com/media/_tech/media/datasheets/BST-BMI055-DS000-08.pdf))
* The state of the volume up/down/mute buttons
* The current volume
* Whether the headset currently being worn

You can trigger an interrupt read of 64 bytes on the sensor interface at any time to get a sensor readout. No matter how fast you query in a loop, the PSVR will only return up to its maximum of 2000 readouts per second.

## Inertia measurements

In each frame of data received from the sensor interface, there are two IMU readout values from two different
instants of time taken very close together.

This means that for any sensor data received, we have the option of using the second readout for more
accurate results.

The PSVR generally samples 2000 readouts from the sensors every second, or 2000Hz. This means there is `1 / 2000 = 0.0005` seconds
between samples, or one sample every 500 microseconds.

You can find the number of milliseconds between measurements by reading the two timestamps from each frame and getting the difference.


## Frame format

Frames read from this interface are always 64 bytes in size.

The frame format looks like this

```c
struct {
  // Information about the headset's buttons.
  struct {
    uint8_t unknown:1;
    // Set if the volume up button is pressed.
    uint8_t plus:1;
    // Set if the volume down button is pressed.
    uint8_t minus:1;
    // Set if the mute button is pressed.
    uint8_t mute:1;
  } button;
  uint8_t unknown1;
  // The volume, from 0-50.
  uint8_t volume;
  uint8_t unknown2[5];
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
    uint8_t unknown:2;
    // Appears to alternate every second.
    uint8_t tick:1;
  } status;
  // FIXME: one of the bits in here specifies the display mode.
  // https://github.com/gusmanb/PSVRFramework/wiki/Report---Sensor-Data
  uint8_t unknown3[11];
  // Two readouts from the BMI055 IMU at different time instants.
  //
  // The integer values within are between -32768 and 32767, i.e.
  // they can take the entire range of a 16-bit signed int.
  //
  // These values can be converted to [-1.0, +1.0] floats by
  // casting to float and simply dividing by 32768.
  //
  // FIXME: There are fields for storing timestamps of
  //        readouts, document them here.
  struct {
    // Gyroscope readings.
    //
    // These values must be preprocessed before sensor fusion by
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
    // The components of these readings must be preprocessed before
    // sensor fusion, like so:
    //
    // float(-value << 4) / 32768.0
    struct  {
      int16_t x;
      int16_t y;
      int16_t z;
    } accel;
    uint8_t unknown[4];
  } data[2];
  uint8_t unknown4[12];
} frame;
```

## How to handle gyroscope and accelerometer data

The PSVR has a [BMI055][imu_datasheet] Inertial Measurement Unit ([IMU](https://en.wikipedia.org/wiki/Inertial_measurement_unit)) for detecting acceleration on the headset.

### IMU background

In order to improve accuracy, most IMUs combine two or three separate kinds of movement sensors inside their silicon. The more sensors the better. Each sensor gives three Degrees of Freedom (DoF) - one for each axis. Thus, a two sensor IMU is referred to as 6 DoF, and a three sensor IMU is referred to as 9 DoF.

The PSVR's BMI055 has 6 degrees of freedom due to its gyroscope and accelerometer.

### Sensor Fusion Algorithms

This means that extra processing must be done in software to _fuse_ the data from the sensors into one set of values, usually rotation angles for each axis.

There are a number well-known [sensor fusion](https://en.wikipedia.org/wiki/Sensor_fusion) algorithms for this.

* [Madgwick filter](http://x-io.co.uk/open-source-imu-and-ahrs-algorithms/) (most common algorithm with PSVR hackers)
* [Mahony filter](http://www.olliw.eu/2013/imu-data-fusing/)

The sensor fusion algorithm will convert the 6 input values into a 3D rotation which can then be directly used in software.


[imu_datasheet]: https://www.bosch-sensortec.com/bst/products/all_products/bmi055
