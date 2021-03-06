# libspecnfra

## What is this

Libspecinfra is a project to make the alternative of [specinfra gem](https://github.com/mizzy/specinfra) by Rust.

Specinfra gem is a base library of [serverspec](http://serverspec.org/) .

Specinfra gem is made of Ruby, so you cannot call the functions of specinfra from other languages.

Libspecinfra project will provide binary library and language bindings to call this library. So you will be able to call the functions of specinfra from many languages.

This project is in the phase of very beginning. If you have comments, questions and so on, feel free to post comments in GitHub Issues.

[Here](http://atl.recruit-tech.co.jp/blog/4339/) is the Japanese post about this project.

## Language bindings

We provide only Ruby and mruby bindings currently.

* [Ruby](https://github.com/libspecinfra/libspecinfra-ruby)
* [mruby](https://github.com/libspecinfra/mruby-libspecinfra)

Other languages will be supported.

## Sample code

This is a sample mruby code to get the permission of `/etc/passwd` of local host:

```ruby
b = Libspecinfra::Backend::Direct.new()
s = Libspecinfra::Specinfra.new(b)
f = s.file("/etc/passwd")

printf("%#o", f.mode)
```

This is a sample mruby code to get the permission of `/etc/passwd` via SSH:


```ruby
b = Libspecinfra::Backend::SSH.new("localhost")
s = Libspecinfra::Specinfra.new(b)
f = s.file("/etc/passwd")

printf("%#o", f.mode)
```

Other language examples are [here](https://github.com/libspecinfra/examples) .


## TODO

### Resources

Resources should be implemented. These resources are picked up from [Resource Types of Serverspec](http://serverspec.org/resource_types.html) . So some resources are implemented in serverspec, not in specinfra.

* bond
* bridge
* cgroup
* command
* cron
* default gateway
* docker container
* docker image
* file
* group
* host
* iis app pool
* iis website
* interface
* ip6tables
* ipfilter
* ipnat
* iptables
* kernel module
* linux audit system
* linux kernel parameter
* lxc
* mail alias
* mysql config
* package
* php config
* port
* ppa
* process
* routing table
* selinux
* selinux module
* service
* user
* x509_certificate
* x509_private_key
* windows_feature
 

### Platforms

Platforms should be implemented. Specinfra gem supports these platforms.

* AIX
* Alpine Linux
* Amazon Linux
* Arch Linux
* CoreOS
* Cumulus Linux
* MacOS(Darwin)
* Debian Linux
* elementary OS
* EOS(Arista)
* VMWare ESXi
* Fedora
* FreeBSD
* Gentoo Linux
* Linux MInt
* NixOS
* OpenBSD
* openSUSE
* Plamo Linux
* Poky(Yokto)
* Redhat Linux
* SUSE Linux Enterprise Server
* SmartOS
* Solaris
* SuSE Linux
* Ubuntu
* Windows


### Backends

Backends should be implemented. Specinfra gem supports these backends.

* CMD(Windows)
* Docker
* Exec
* jexex
* LXC
* SSH
* Telnet
* WinRM
