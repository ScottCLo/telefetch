# telefetch
Its a system fetch utility that should work with Teletype, but I dont have a Teletype to test it on :(. I only made ASCII art for the distros I use. Feel free add your own and send a pull request.
## Installation
Just run:
```console
git clone https://github.com/ScottCLo/telefetch.git
cd telefetch
cargo build -r
cd target/release/
```
You can run the program from here with:
```console
./telefetch
```
Or move it to ```/usr/local/bin/``` to be able to run it globaly.
```console
sudo mv ./telefetch /usr/local/bin/
```
## Running
Once installed you can run it by using the ```telefetch``` command.
```console
$ telefetch
   ______     scott@shop-pc
 _ \____ \    os     Void Linux
: \  __ \ \   kernel 6.5.13_1
: : /  \ \ :  uptime 4d 23h 46m
: \ \__/ : :  cpu    AMD Ryzen 7 3700X 8-Core Processor
 - \____ \_:  memory 7468M / 15920M
  \_____\

```
You can change the ASCII art buy using ```telefetch --distro``` Current options are:
```console
--arch
--default
--linux
--void
```
