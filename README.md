# hackxpansion-joystick

The joystick module for hackxpansion!
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

The joystick axes are connected as potentiometers between 3.3 V and GND, with the wipers connected to the ADC pins.


---

This was all possible thanks to [Hackspansion: A hackclub YSWS](http://hackxpansion.hackclub.com/)
