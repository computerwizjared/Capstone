# Assignment 0

I began this project by following the steps located at the CS140e course [here](https://cs140e.sergio.bz/assignments/0-blinky/). All steps in this documentation are executed within the `cs140e/assignment-0` directory. I am using an Adafruit USB to TTL Serial Cable from [here](https://www.adafruit.com/product/954) which is compatible with the same driver used in the CS140e course. I also use a generic USB-C to USB-A adapter, since my MacBook only has USB-C ports and the cable is USB-A.

## Phase 0

I cloned the "blinky" skeleton project from the CS140e Git repository into this repository under the `blinky` folder.

```
git clone https://cs140e.sergio.bz/assignments/0-blinky/skeleton.git blinky
rm -rf blinky/.git
```

Then I ran the commands instructed to setup the assignment:

```
cd blinky
make fetch
```

However, I ran into this issue:

```
wget https://cs140e.sergio.bz/assignments/0-blinky/data/firmware.tar.gz -O files/firmware.tar.gz
make: wget: No such file or directory
make: *** [files/firmware.tar.gz] Error 1
```

I found out that modern versions of macOS don't come pre-installed with `wget`.
Rather than install `wget`, I adjusted the `Makefile` to use `curl`.

```diff
$(ASSIGNMENT_FILES): | $(FILES_DIR)
-	wget $(BASE_URL)/assignments/0-blinky/data/$(@:$(FILES_DIR)/%=%) -O $@
+	curl $(BASE_URL)/assignments/0-blinky/data/$(@:$(FILES_DIR)/%=%) -o $@
```

To include the `files` directory in this Git repository (in case the original source goes offline),
I modified the `.gitignore` file to include it:

```diff
# Assignment 0 specific
phase4/build
phase4/target
-files/
+-files/**
```

## Phase 1

Since I run macOS, I began by installing the `Silicon Labs VPC Driver` for macOS from [here](https://www.silabs.com/documents/public/software/Mac_OSX_VCP_Driver.zip).

I then connected my Pi to my USB to Serial cable using this GPIO diagram and table:

![Diagram of Raspberry Pi GPIO pinout](assets/pi3-gpio-pinout.svg?raw=true&sanitize=true)

| GPIO Pin (Function) | USB to Serial Wire Color (Function) |
| --- | --- |
| 4 (5V PWR) | Red (5V @ 500mA) |
| 6 (GND) | Black (Ground) |
| 8 (UART_TXD0) | White (RX) |
| 10 (UART_RXD0) | Green (TX) |

I then plugged in my USB to Serial cable to my MacBook, and saw the red power LED on the Pi light up, which means that the Pi powered on properly! I ran this command on my MacBook to determine the serial device:

```
ls /dev/tty.*
```

I found that my USB to Serial cable device is located at `/dev/tty.usbserial-0001`.

Here is a picture of my setup!

![Picture of the Raspberry Pi plugged into MacBook](assets/serial-setup.jpg?raw=true)

Next, unplugged my Pi, inserted my SD card into my MacBook, and opened Disk Utility.
I selected the SD card, and clicked `Erase` in the top bar of the window.
I named the SD card `BOOT` and selected `MS-DOS (FAT)` as the format, as shown below, and clicked `Erase`.

![Screenshot of Disk Utility](assets/erase-sd-card.png?raw=true)

I then copied `blinky/files/firmware/*` to the root of the SD card.
I also copied `blinky/files/activity-led-blink.bin` to the root of the SD card, and renamed it to `kernel8.img`.
I ejected my SD card, and inserted it into my Pi.

Then, I plugged my Pi back into my MacBook.
I saw the red light come back on, but also saw a flashing green light!
This means that the custom OS image worked!

Then, I ran this command to connect to the Pi over the serial console:

```
screen /dev/tty.usbserial-0001 115200
```

Note, `/dev/tty.usbserial-0001` is the device of the USB to Serial cable from earlier, and 115200 is the baud rate of the connection.

I then saw the console output below indicating the LED status, which means the serial connection worked!

```
On!
Off!
On!
Off!
```

I then used `Ctrl+A`, then `K`, then `y` to exit the serial console, and unplugged my Pi from my MacBook.