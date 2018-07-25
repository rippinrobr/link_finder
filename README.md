## Link Finder

I use this utility to search the given markdown file for links and print them out to stdout.

### Usage
Parses Markdown files looking for Links and creates a list of all links found in the file

```
USAGE:
    link_finder --input <MARKDOWN FILE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <MARKDOWN FILE>    A path to a markdown file
```

### Running

If you are new to the Rust world and you are going to run link_finder with cargo you'll need to run like this: 

`cargo run -- --input <path to markdownfile>`
