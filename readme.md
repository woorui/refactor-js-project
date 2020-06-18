# refactor your js project

First, build It.

```bash
cargo build --release
```

Run bin.

```bash
$ ./target/release/refactor-js-project --help
refactor-js-project 0.1.0
Refactor js project to ts project

USAGE:
    refactor-js-project [OPTIONS] --path <path>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -p, --path <path>                            The path to the file to read
    -s, --source_extension <source-extension>    The source extension, default js [default: js]
    -t, --target_extension <target-extension>    The target extension, default ts [default: ts]
```

Pass-in your project path.

```bash
$ ./target/release/refactor-js-project --path ./_family/
Will change "./_family/children\\daughter\\Lisa.ts", and It's No 1
Will change "./_family/children\\index.ts", and It's No 2
Will change "./_family/children\\son\\Tom.ts", and It's No 3
Will change "./_family/father\\Jim.ts", and It's No 4
Will change "./_family\\index.ts", and It's No 5
Will change "./_family/mother\\Mary.ts", and It's No 6
The total number of changes was 6
```

Bingo! Your js project is ts project Now.

```bash
$ tree ./_family/
./_family/
├── children
│   ├── daughter
│   │   └── Lisa.ts
│   ├── index.ts
│   └── son
│       └── Tom.ts
├── father
│   └── Jim.ts
├── index.ts
├── mother
│   └── Mary.ts
└── readme.md
```
