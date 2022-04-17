# gxrfetch
A simple fetch program written in rust

# Installation
run the command

```bash
cargo build
```

after which binary executable will be avaliable in `./target/debug/gxrfeth`

# Configuration

Configuration files will be generated in `~/.config/gxrfetch` directory

syntax:

* (x) - color, where x is the first letter of the main 8 terminal colors
* (xl) - light colors, where x is the first letter of the 8 terminal colors
* \<B> - make text bold
* \<I> - make text italic
* \<BI> - make text bold and italic
* [module] - module to insert (name,cpu,cores,bat,mem,os,distro,kernel,shell,term)
