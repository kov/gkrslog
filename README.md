# Log colorizer

This is a toy I wrote for learning some Rust basics. It is a recreation of one of my first C programs, which surprisingly [still lives](http://hpux.connect.org.uk/hppd/hpux/Sysadmin/gklog-0.7/) on the Internet… imagine that!

## Usage

Something like this:

```
$ journalctl --user -f | gkrslog --rule 'red=python3.*unhandled Python exception' --rule green=Succeeded
```