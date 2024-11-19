# DR400 Tweaks

This is a strictly unofficial Rust plugin for the JustFlight DR400 for X-Plane 12.

https://www.justflight.com/product/robin-dr400-xplane
https://store.x-plane.org/Robin-DR400_p_1145.html

It fixes a couple of issues (as of JF_Robin_DR400 v1.9.2):
- landing lights not working when starting cold and dark
- panel brightness defaulting to eyeball-searing
- provide menu item to replace the on-screen arrow to open the JustFlight popup

### Landing lights
The DR400 has separate left and right landing lights, which are mapped to the
second and third landing light switches. The first landing light switch acts
like a master switch - it must be turned on for either landing light to work.

For a cold and dark start, the first landing light switch is off and there's
no way to turn it on (other than mapping a key or button to it), which
effectively stops the landing lights from working.

Also, when the first landing light switch is turned on, the SASL plugin will
turn on the left and right landing lights also.

This plugin turns the first landing light switch on, and then if necessary
it turns the left and right landing lights off a moment later.

### Panel brightness
The default brightness is very very bright and washes out the instruments.
The plugin turns it down to 0.01 (1%) which is bright enough at nighttime.

### JustFlight menu popup
The plugin adds a JustFlight submenu to the Plugins menu, which currently has
one entry, to show/hide the JustFlight popup panel.

![screenshot of plugin menu](/img/justflight_menu.png)

To remove the on-screen arrow:
- Open `[X-Plane]\Aircraft\JF_Robin_DR400\manifold.json` in a good text editor
  i.e. Notepad++
- Search for `Arrow1_Tab`
- On the previous line, change `"Load": true,` to `"Load": false,`

![screenshot of modified manifest.json](/img/manifold_json%20edit.png)

## Installation (Windows)

Copy the `dr400tweaks` folder from `compiled` to your DR400 `plugins` folder

To confirm it's running, load the DR400 and open Plugins > Show Plugin Admin.
The compilation date will be shown under Information > DR400 Tweaks

## Build instructions

```
cargo build --release
```
Rename the `dr400tweaks.dll`/`.so`/`dylib` to `.xpl`

Place it in your DR400 plugins folder [as per the X-Plane SDK](https://developer.x-plane.com/article/building-and-installing-plugins/)

The `release.bat` file automates these steps (for my setup - customise as required!)

## Contributing / future plans

Please raise issues, pull requests, etc on Github. For general discussion see
the [x-plane.org forums](https://forums.x-plane.org/index.php?/forums/forum/518-robin-dr400/)

I am hoping JustFlight can incorporate these changes into the official plugin
and make this plugin redundant!

Something I'd like to add is commands/datarefs to let the canopy have
positions other than "fully open" and "locked closed" e.g. you'd drag the
canopy forward/backward and click the handle to lock it.
