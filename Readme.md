# A4Tech Mouse Control

This project is just for fun to try to reverse engeeniring USB mouse and create GTK app use Rust lang. I have 2 mouse V7M and V8M which I use for test.

![image](https://user-images.githubusercontent.com/2198153/93603940-9fe56400-f9bc-11ea-9fd3-1e1f34773fba.png)


Udev and user permissions:
```shell
# copy you product id of device like ...ID 09da:xxxx where is xxxx is what you need and to change it in the file udev/10-a4tech.rules
lsusb
sudo cp udev/10-a4tech.rules /etc/udev/rules.d/
sudo groupadd a4tech
sudo usermod -aG a4tech $USER
sudo udevadm control --reload-rules && udevadm trigger
```

Big thanks with part of work the guys

- https://github.com/openrazer/openrazer/wiki/Reverse-Engineering-USB-Protocol
- https://gill.net.in/posts/reverse-engineering-a-usb-device-with-rust/
- https://www.linuxvoice.com/drive-it-yourself-usb-car-6/
- https://github.com/a1ien/rusb
- https://gitlab.com/C0rn3j/configs/-/blob/master/ansible/serverPlaybooks/roles/bree/files/html/a4/bloody.php
- https://gitlab.com/C0rn3j/a4tech_bloody_p85_driver
- https://github.com/Marisa-Chan/init-gmouse

And like a reference for good GTK ui for my third mouse
- https://github.com/pwr-Solaar/Solaar
