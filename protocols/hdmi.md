# The PSVR HDMI signal format

The PSVR headset's video signal is driven through the HDMI connection labelled "HDMI _PS4_" on the processing unit.

The two display modes expect different image formats

* [Virtual Reality mode](#virtual-reality-mode)
* [Cinematic mode](#cinematic-mode)

## Virtual Reality Mode

When the headset is in [Virtual Reality mode][vr mode], the HDMI signal **must** use [4:4:4 chroma subsampling](https://en.wikipedia.org/wiki/Chroma_subsampling#4:4:4) and the RGB color format.

If the signal is not RGB 4:4:4, then a black screen will be shown
in the headset instead of the desired signal. This problem can be
hard to diagnose withour prior knowledge.

Be sure to check the list of [systems that have issues with RGB 4:4:4](#systems-that-have-rgb-4:4:4-display-issues).

## Cinematic mode

In [Cinematic mode][cinematic mode], things are simpler.

The HDMI signal can be in the RGB or YCbCr color spaces, and potentially more.

In a sentence - render a 1920x1080 image to the headset and it'll Just Work™.

## Systems that have RGB 4:4:4 display issues

If this setting needs to be changed, it can generally done via the operating system display preferences, or the configuration program of the graphcis card.

### AMD cards

Settings usually need to be changed manually for AMD cards to match the RGB 4:4:4 requirement.

* Go to AMD Radeon Settings -> Preferences -> Radeon Additional Settings
* Change the pixel format to RGB 4:4:4 Standard

### Mac computers

MacOS will automatically format the HDMI signal as YCbCr if it detects that a monitor is a television, whereas it will format the signal as RGB if it believes it is a computer monitor.

MacOS detects the PSVR headset as a television, causing it to automatically choose the YCbCr color space. This means that without hacks, running your VR app on a Mac in VR mode will always result in a black screen.

Here are some hacks to get around it

#### Modify display EDID data to force RGB mode

It is possible to create a file in the `/System` folder that forces RGB mode for the PSVR display only.

This isn't a particularly nice hack because

* It requires the user to boot into recovery mode and disable
  [System Integrity Protection](https://support.apple.com/en-nz/HT204899), at least temporarily
* It must be done individually on every MacBook the program will be run on

You can see the steps to do this [here](http://www.mathewinkson.com/2013/03/force-rgb-mode-in-mac-os-x-to-fix-the-picture-quality-of-an-external-monitor).

##### Anecdote

The first time I tried this, it worked perfectly.. for a few hours.
After that, the headset started displaying a black screen again.
I am not aware of what stopped it from working, and when I applied the hack again, nothing changed.

#### Render to the computer's RGB display, mirroring it to the headset

Another way of forcing RGB color format is by rendering the target
video directly to the Mac's system display, an RGB monitor.

The display preferences must then be changed to mirror the builtin
screen **to** the headset. It is important that the screen mirrors to the headset, not the other way around.

This likely works because when the display is mirrored, its color space probably is too.

This can be made somewhat more bearable by automatically maximizing and focusing the render window.

This hack has the downside that you can no longer display a separate image on the system monitor (such as a social screen).



[vr mode]: /modes/virtual_reality.md
[cinematic mode]: /modes/cinematic.md
