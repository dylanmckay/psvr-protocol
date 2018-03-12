# psvr-protocol

Breakdown of the PlayStation VR communication protocols for programmers.

# The parts

The PSVR comes with two main components - the actual headset, and a small
processing unit that handles most of the logic. These two parts are required -
the headset cannot function without the box.

## The processing unit

This device sits in-between the headset and the computer.

The processing unit has multiple IO connections

* On the back
  * A USB connection to a PlayStation or computer
      * This is used by the computer to control the headset, give
        orientation sensor readouts, send audio, read Mic data, and more
  * A HDMI video input that drives the video in the headset itself,
    labelled **"HDMI _PS4_"**
  * An optional HDMI video output that goes to an external TV,
    called the social screen as this is what the people around you see - labelled **"HDMI _TV_"**
* On the front
  * A HDMI video output that goes directly to the headset
  * A proprietary (AFACT) connector that goes directly to the headset
      * This is used internally by the processing unit for things like
        retrieving inertia sensor readouts

![psvr processing unit](res/images/psvr-processing-unit.jpg)

## The headset

![psvr headset](res/images/psvr-headset.jpg)

# The basics




