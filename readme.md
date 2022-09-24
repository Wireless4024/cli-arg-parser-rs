# (WIP) Cli args parser (rust)

# How it work?

This library help to parse cli argument (inspired from linux cli)  
currently working with utf-8 input, ascii mode for zero-copy parsing is coming soon.

```
\'\"\n\r\t\v\061\u1F600\\\\A 'word with space' also\ space
```

will output as

```
"\'\"\n\r\t\x0b\x61😀\\\\A"
"word with space"
"also space"
```