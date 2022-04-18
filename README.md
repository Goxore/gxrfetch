# gxrfetch
A simple and customizable fetch program written in rust.
Works on most linux distros, probably works on mac, and maybe even somewhat works on windows (I did not test)

# Installation
run the command

```bash
cargo build
```

binary executable will be avaliable in `./target/debug/gxrfeth`

# Configuration

Configuration files are generated in `~/.config/gxrfetch` directory

syntax:

* (x) - color, where x is the first letter of the main 8 terminal colors
* (xl) - light colors, where x is the first letter of the 8 terminal colors
* \<B> - make text bold
* \<I> - make text italic
* \<BI> - make text bold and italic
* [module] - module to insert (name,cpu,cores,bat,mem,os,distro,kernel,shell,term)

[gpu] module is also avaliable, but highly discouraged to use, as it decreases performance
