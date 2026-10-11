# hackxpansion-joystick

The joystick module for hackxpansion!
The module uses a two-axis potentiometer joystick to provide X and Y analog readings, allowing projects to interpret directional movement and user input. The center button adds an extra digital input for selection or other interactions.
<br/>
**Crates:** pkg:cargo/JoystickHackx@0.1.0
# Images
![case](https://cdn.hackclub.com/01a0e3a9-230c-7f25-a3b6-166c52b7dd30/image.png)
![pcb](https://cdn.hackclub.com/01a0de11-4681-7c4f-a998-47afab4f1d09/image.png)
![schematic](https://cdn.hackclub.com/01a0de11-f6c5-7536-852f-4c2892e97569/image.png)


### Pin Mapping

| Function      | Xpanse GPIO  |
| ------------- | ------------ |
| X axis        | GPIO7 / ADC0 |
| Y axis        | GPIO8 / ADC1 |
| Center button | GPIO9        |

See the [Xpanse API docs](https://docs.rs/xpanse-api/latest/xpanse_api/index.html)

The joystick axes are connected as potentiometers between 3.3 V and GND, with their wipers connected to the ADC pins. This allows the X and Y positions to be read as analog voltage levels. The center button provides a digital input.

## BOM
| Designator | Footprint                  | Quantity | Value           | LCSC Part # | Price (CAD) |
| ---------- | -------------------------- | -------- | --------------- | ----------- | ----------- |
| C1, C2     | C338148_C0402              | 2        | 47nF            | C338148     | $0.08       |
| J1         | Male_Module                | 1        | Included in kit | —           | —           |
| R1         | 0805                       | 1        | 1k1             | C185294     | $1.00       |
| R2         | 0805                       | 1        | 1k2             | C269531     | $1.00       |
| VR1        | C918854_SW-TH_RKJXV1224005 | 1        | RKJXV122400R    | C918854     | $2.64       |
| PCB        | —                          | —        | PCB             | —           | $4.00       |
| PCBA Fee   | —                          | —        | PCBA Assembly   | —           | $18.26      |
| Shipping   | —                          | —        | Shipping        | —           | $6.45       |
| TOTAL      | —                          | —        | —               | —           | $33.43      |
---

This was all possible thanks to [Hackspansion: A hackclub YSWS](http://hackxpansion.hackclub.com/)
