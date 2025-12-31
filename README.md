# The curse of the headless goose

## Bill of materials

- ILI9486 (3.5", Rgb666, the red pcb ones that also have an SD card reader worked best for me)
- Keymatrix (0-9, \*, #, A-D)
- AT24C256 I2C
- RP2040-zero

## Pinout

- GPIO 0: Screen Reset
- GPIO 1: Screen CS
- GPIO 7: Keymatrix row 2
- GPIO 8: Keymatrix row 3
- GPIO 9: Keymatrix column 3
- GPIO 10: Keymatrix row 4
- GPIO 11: Keymatrix column 1
- GPIO 12: Keymatrix row 1
- GPIO 13: Keymatrix column 2
- GPIO 14: Screen SCK
- GPIO 15: Screen SDI
- GPIO 26: AT24C256 SDA
- GPIO 27: AT24C256 SCK
- GPIO 28: Screen D/C

## License

`GPL-3.0-or-later`
