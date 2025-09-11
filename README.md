Experimenting with reverse-engineering a GMMK Compact to control its LEDs.

In the source code, you can toggle the LEDs on or off (assuming the other default options):

```c
// comment one or the other
led_on(handle, buf);
// led_off(handle, buf);
```

## Build

```bash
gcc src/driver.c -o driver -lhidapi-hidraw && sudo ./driver 
```

