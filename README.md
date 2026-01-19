# greetd

greetd is a minimal and flexible login manager daemon that makes no assumptions about what you want to launch.

Use [gtkgreet](https://git.sr.ht/~kennylevinsen/gtkgreet) to launch [sway](https://github.com/swaywm/sway) if you want a fully graphical session, or use `agreety` to launch a shell if you want a drop-in replacement for `agetty(8)` and `login(1)`.

If you can run it from your shell in a TTY, greetd can start it. If it can be taught to speak a simple JSON-based IPC protocol, then it can be a greeter.

See the [wiki](https://man.sr.ht/~kennylevinsen/greetd) for FAQ, guides for common configurations, and troubleshooting information.

## List of known greetd greeters

- agreety - The simple, text-based greeter living in this repo is a simple example.
- [cosmic-greeter](https://github.com/pop-os/cosmic-greeter) - The default greeter for the COSMIC desktop environment
- [gtkgreet](https://git.sr.ht/~kennylevinsen/gtkgreet) - The flagship graphical, GTK based greeter (xdg-shell or wlr-layer-shell, to be used with something like `sway`)
- [qtgreet](https://gitlab.com/marcusbritanicus/QtGreet) - Qt-based greeter (using wlr-layer-shell, to be used with something like `sway`)
- [dlm](https://git.sr.ht/~kennylevinsen/dlm) - Dumb Login Manager (using fbdev)
- [ddlm](https://github.com/deathowl/ddlm) - Deathowl's dummy login manager (using fbdev)
- [wlgreet](https://git.sr.ht/~kennylevinsen/wlgreet) - Wayland greeter (using wlr-layer-shell, to be used with something like `sway`)
- [tuigreet](https://github.com/apognu/tuigreet) - Console UI greeter (using tui-rs)
- [ReGreet](https://github.com/rharish101/ReGreet) - Clean and customizable GTK4 based greeter (to be used with something like `sway`)
- [marine_greetdm](https://github.com/Decodetalkers/marine_greetdm) - A cli greeter, by rustyline and without gui. It can configure enviroment variables for special desktops.
- [greetd-qmlgreet](https://github.com/Decodetalkers/greetd-qmlgreet) - Qt6 qml greetd (using ext-session-shell, to be used with something like `river` and `sway`)
- [Phog](https://gitlab.com/mobian1/phog) - Obsolete, use Phrog instead.
- [Phrog](https://github.com/samcday/phrog) - A greeter that works on mobile devices and also other kinds of computers.

Patches expanding the list welcome.

## Installation

The below will install greetd, agreety and the default configuration. This looks *just* like `agetty(8)` and `login(1)`. See the manpages and the wiki for information on how to do more interesting things.

### From packages

#### Arch Linux

greetd and a few greeters are available in AUR for Arch Linux.

#### Gentoo

```sh
emerge gui-libs/greetd
```

#### FreeBSD
If you are using ports:
```sh
cd /usr/ports/x11/greetd
make install clean
```

or, if you are using packages:
```sh
pkg install greetd
```

### Manually from source

#### Linux

```sh
# Compile greetd and agreety.
cargo build --release

# Put things into place
sudo cp target/release/{greetd,agreety} /usr/local/bin/
sudo cp greetd.service /etc/systemd/system/greetd.service
mkdir /etc/greetd
cp config.toml /etc/greetd/config.toml
cp greetd.pam.linux.sample /etc/pam.d/greetd

# Create the greeter user
sudo useradd -M -G video greeter
sudo chmod -R go+r /etc/greetd/

# Look in the configuration files: `/etc/greetd/config.toml` and `/etc/pam.d/greetd`, and edit them as appropriate.
# When done, enable and start greetd
systemctl enable --now greetd
```

#### FreeBSD
```sh
# Compile greetd and agreety.
cd greetd
cargo build --release
cd ../agreety
cargo build --release --all-features
cd ../man
make greetd.1 greetd.5 greetd-ipc.7 agreety.1
cd ..

# Put things into place
cp target/release/{greetd,agreety} /usr/local/bin/
cp greetd.rc /usr/local/etc/rc.d/greetd
mkdir /usr/local/etc/greetd
cp config.toml /usr/local/etc/greetd/config.toml
cp greetd.pam.freebsd.sample /usr/local/etc/pam.d/greetd
cp man/greetd.1 man/agreety.1 /usr/local/share/man/man1/
cp man/greetd.5 /usr/local/share/man/man5/
cp man/greetd-ipc.7 /usr/local/share/man/man7/

# Create the greeter user and add them to the video group
pw useradd -u 101,1000 -i 101,1000 -g "" -d /nonexistent -s /usr/sbin/nologin -c "greetd Daemon" greeter 
pw groupmod video -m greeter

# Look in the configuration files: `/usr/local/etc/greetd/config.toml` and `/usr/local/etc/pam.d/greetd`, and edit them as appropriate.
# When done, enable and start greetd
sysrc greetd_enable=YES"
service start greetd
```

## How do I write my own greeter?

All you need is an application that can speak the greetd IPC protocol, which is documented in `greetd-ipc(7)`. See gtkgreet or agreety for inspiration.

# How to discuss

Go to #kennylevinsen @ irc.libera.chat to discuss, or use [~kennylevinsen/greetd-devel@lists.sr.ht](https://lists.sr.ht/~kennylevinsen/greetd-devel).
