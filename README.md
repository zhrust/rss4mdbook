# RSS for mdBook
> a generator for mdBook as CLI tool, export RSS.xml into u want path...

------
## background

mdBook is great, but not auto export RSS,
and the mdbook-rss is not work now...

so make it self ;-)


------
### goal
> as Rustacean homework ...

as crate, can:

- easy install
- usage at local
- usage after mdBook generated static site, 
    - scanning .md path, 
    - generat RSS.xml into export path
    - ...so we hold lasted upgrade content's RSS

------
## Installation

### Cargo
If you already have a Rust environment set up, you can use the cargo install command:

> $ cargo install rss4mdbook

Cargo will build the `rss4mdbook` binary and place it in $HOME/.cargo.


### Manual installation from GitHub
Compiled binary versions of `rss4mdbook` are uploaded to GitHub when a release is made. You can install `rss4mdbook` manually by downloading a release, extracting it, and copying the binary to a directory in your `$PATH`, such as `/usr/local/bin`.

For more information, 

...TBD

### Homebrew

..TBD


------
## Usage
> daily usage , only one shot:

- 0: config mdBook's book.toml, append such as:

```toml
...
[rss4mdbook]
url-base = "https://rs.101.so"
```
- 1: mdbook build
- 2: use `gen` command, append the lasted 4 articles as rss.xml 

```
$ rss4mdbook gen /path/2u/mdbook/book.toml
```

that all, 
should make `pub.sh` under u mdBook site root,
include commands like:

- mdbook build
- rss4mdbook gen /path/2u/mdbook/book.toml
- git add .
- git commit -a .
- git push
- ...

will auto upgrade site and RSS.

> BYW:

u need modify u `theme/index.hbs` insert the rss.xml at some where.

## logging

- ...
- 230308 ZQ publish 2 version
- 230306 ZQ init.


### refer.


- [clap::_derive::_cookbook::git_derive - Rust](https://docs.rs/clap/latest/clap/_derive/_cookbook/git_derive/index.html)
- [Building a CLI from scratch with Clapv3 | by Ukpai Ugochi | Medium](https://medium.com/javascript-in-plain-english/coding-wont-exist-in-5-years-this-is-why-6da748ba676c)
    - odd, can not cargo check
- [mdbook - crates.io: Rust Package Registry](https://crates.io/crates/mdbook/0.4.28)
    - [Preprocessors - mdBook Documentation](https://rust-lang.github.io/mdBook/format/theme/syntax-highlighting.html)
    - [mdbook-rss - crates.io: Rust Package Registry](https://crates.io/crates/mdbook-rss)
    - ...


------


```
       _~-+`~_
   () /  > ♡  \ \/
     '_   ⎕   _'
     / '-----' |

...act by ferris-actor v0.2.4 (built on 23.0303.201916)
```






