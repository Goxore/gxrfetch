# gxrfetch
A simple and customizable fetch program written in rust.
Works on most linux distros, probably works on mac, and maybe even somewhat works on windows (I did not test)

# Screenshots

[img/trident.png](img/trident.png)

[img/portal.png](img/portal.png)

[img/meamind.png](img/megamind.png)

# Dependencies
* rust
* coreutils

optional:
* nerdfonts - icons

# Installation
run the command

```bash
cargo build
```

binary executable will be avaliable in `./target/debug/gxrfeth`

# Configuration

Configuration files are generated in `~/.config/gxrfetch` directory

`~/.config/gxrfetch/config` - right part of the fetch, where modules are located
`~/.config/gxrfetch/ascii` - left part of the fetch, where ascii art is located

all spaces and tabs in both files affect how the final result looks.
I suggest leaving a couple of extra spaces before every line in `config` file,
and check line endings in `ascii` file, so you don't get unexpected results.

syntax:

* (x) - color, where x is the first letter of the main 8 terminal colors
* (xl) - light colors, where x is the first letter of the 8 terminal colors
* \<B> - make text bold
* \<I> - make text italic
* \<BI> - make text bold and italic
* [module] - module to insert (name,cpu,cores,bat,mem,os,distro,kernel,shell,term,col,col2)
* [\[date]] - insert any bash command into double square brackets, and it will be replaced
with it's output

[gpu] module is also avaliable, but highly discouraged to use, as it decreases performance

[env] desktop environment, if avaliable

# Contributing

Feel free to contribute code, ascii arts or config examples, i'm more than happy to include those!
