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

As well as being RGB, the image also needs to be distored to account
for the curvature of the lens.

### Barrel distortion

The image should use barrel distortion. If this is not done, the visible image in the headset itself will look disstored.

For testing purposes, sending a signal with no distortion correction will work okay, but it will not form a clear image, becoming uncomfortable after a small amount of time.

In the barrel distortion equation, there can be infinitely many coefficients.

The distortion coefficients of the PSVR are known to the second order.

The first and second order coefficients are

* The first order coefficient is `0.22`  (AKA `K1` or `a`)
* The second order coefficient is `0.24` (AKA `K2` or `b`)

Credit to [Agustín Gimenez Bernad](https://github.com/gusmanb) for measuring these.

Coefficients of zero have no effect, and thus if you are using a library whose API requires multiple distortion coefficients you can and should set the remaining coefficients to zero.

### Chromatic aberration

The color of the light passing through a spherical lens can be
distored due to refraction at slightly different angles, causing
specific colors ("wavelengths of light") to focus at slightly
different points.

This effect is called _chromatic aberration_, or simply put; color distortion.

Software can choose to correct for this by taking into account
chromatic aberration factors when performing the barrel distortion.

These are the measured chromatic aberration factors.

**FIXME**: Every formula I've seen for chromatic aberration
           correction makes no distinction between horizontal
           and vertical. How are these single values calculated?
           Find it out and put it here.

| Color    | Vertical factor | Horizontal factor
|----------|-----------------|-------------------
| Red      | 1.0             | 1.0
| Green    | 1.0078          | 1.0091
| Blue     | 1.0192          | 1.0224

Credit to [Agustín Gimenez Bernad](https://github.com/gusmanb) for measuring these.

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
