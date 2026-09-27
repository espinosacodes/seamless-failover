# iPhone Shortcuts automation (Phase 2)

Goal: Personal Hotspot turns itself on when the Mac needs it, with no app and
no MDM. This depends on an iOS Shortcuts action that is still unverified on
iOS 26. See OQ-001 and OQ-002.

## First check (2 minutes on the iPhone)

1. Open Shortcuts.
2. Tap plus, Add Action, search for "Hotspot".
3. Look for an action named "Set Personal Hotspot" with an On value.
4. If it exists, note the exact iOS version under Settings, General, About.
5. If it does not exist, stop here and use the fallback below.

## Automation recipe (if the action exists)

1. In Shortcuts, go to Automation, New Personal Automation.
2. Pick a trigger. Best candidates in order:
   a. Bluetooth connects to the Mac. Fires when the iPhone sees the Mac.
   b. Connects to power or USB. Fires when you plug the cable.
   c. Time of day. Reliable but keeps hotspot on wastefully.
3. Add Action, "Set Personal Hotspot", set to On.
4. Disable Ask Before Running and confirm no notification prompt.
5. Test over several plug and unplug cycles. Record how often it fires alone.

## Known caveats

* Wi-Fi disconnect triggers may not fire when the Mac is still associated to
  the dead network while the iPhone sits on cellular.
* Bluetooth triggers depend on a steady pairing between the two devices.
* If the action is only a Settings deep link, it does not toggle anything and
  Phase 2 stays blocked. MDM on a supervised device is then the only forced
  path.

## Fallback (always works)

Control Center, long press connectivity, tap Personal Hotspot. One tap, manual,
required once per session in Phase 1. Wireless daily use keeps this manual step
until the automation above is verified.
