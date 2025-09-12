Experimenting with reverse-engineering a GMMK Compact to control its LEDs.

Use with the `on` or `off` argument (with root permissions), e.g.

```bash
./driver on
./driver off
```

## Build

```bash
gcc src/driver.c -o driver -lhidapi-hidraw
```

