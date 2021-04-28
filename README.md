# snippets-make

Convert snippets toml config -> vscode, neosnippet, ultisnps

## Installation

Support
- Homebrew (mac)
- Linuxbrew (Linux, WSL)

install
```
brew install fuyutarow/tap/snippets-make
```

clean uninstall
```
brew uninstall snippets-make
brew untap fuyutarow/tap
```


## Usage
```
$ snippets-make -h
snippets-make 0.202104.10

USAGE:
    snippets-make [OPTIONS] <fpath>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --over <over>    Overwrite config of editor. [vscode, vscode-insiders]
    -t, --to <to>        Snippet format of output. [possible values: vscode, neosnippet, ultisnips] [default: vscode]

ARGS:
    <fpath>    Input toml file
```

### `snippets-make -t <to>`

from [samples/typescript](https://github.com/fuyutarow/snippets-make/blob/alpha/samples/typescript.toml)
```toml
lang = 'typescript'
[snippets.gcd]
body = '''
const gcd = (a: number, b: number) => {
return b == 0 ? a : gcd(b, a % b)
};
'''

[snippets.factional]
body = '''
const factional = (n: number) => {
return n <= 1 ? 1 : n * factional(n - 1);
};
'''
```

to vscode
```
$ snippets-make samples/typescript.toml -t vscode
{"gcd":{"prefix":"gcd","body":["const gcd = (a: number, b: number) => {","  return b == 0 ? a : gcd(b, a % b)","};"]},"factional":{"prefix":"factional","body":["const factional = (n: number) => {","  return n <= 1 ? 1 : n * factional(n - 1);","};"]}}
```

to neosnippet
```
$ snippets-make samples/typescript.toml -t neosnippet
snippet factional
    const factional = (n: number) => {
      return n <= 1 ? 1 : n * factional(n - 1);
    };

snippet gcd
    const gcd = (a: number, b: number) => {
      return b == 0 ? a : gcd(b, a % b)
    };

```

to ultisnips
```
$ snippets-make samples/typescript.toml -t ultisnips
snippet gcd
const gcd = (a: number, b: number) => {
  return b == 0 ? a : gcd(b, a % b)
};
endsnippet

snippet factional
const factional = (n: number) => {
  return n <= 1 ? 1 : n * factional(n - 1);
};
endsnippet
```


### `snippets-make --over <over>`

Run with the --over option to overwrite the vscode snippet configuration file.
```
snippets-make samples/typescript.toml --over vscode
```

This is equivalent to the following for a mac
```
snippets-make samples/typescript.toml > ~/Library/Application Support/Code/User/snippets/typescript.json
```

for Windows
```
snippets-make snippets/typescript.toml > $(wslpath "$(wslvar USERPROFILE)")/AppData/Roaming/Code/User/snippets/typescript.json
```
