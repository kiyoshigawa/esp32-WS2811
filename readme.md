# ESP32 WS2811 

This is a project to control the WS2811 LEDs on the ceiling of my office. It may be repurposed later into more useful things, but it begins here.

## To Flash to the Chip:

- flash command :
```powershell
xtensa-cargo espflash --chip esp32 --speed 115200 --features="xtensa-lx-rt/lx6,xtensa-lx/lx6,esp32-hal" COM#
```
- Alternatively just run `./flash.ps1 COM#` in the root directory of this project.

- When running the flash command, to get the chip to talk, we had to connect to and then disconnect from the COM port in putty first.
