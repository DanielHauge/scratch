# Embedded pio

Simply install pio

Then do:

```sh
pio run
```

```sh
pio run --target upload
```

Check on serial monitor:

```sh
pio device monitor
```

Check usb settings with:

```sh
lsusb
dmesg -w
```

May have to check modprobes for uart-serial stuff

```sh
sudo modprobe ch341
sudo modprobe cp210x
```

If not then install linux headers.

```sh
paru -S linux usbutils
```
