---
hide:
  - navigation
---

# CLI Usage

The command line interface is more robust than the web-server. This interface can run without
a graphical display and has no timeout issues and thus makes it ideal for usage on remote servers
where it can work on larger datasets over longer time-periods.

## Commands

The list of available commands are:

- extract
- clean
- format
- pipeline
- server

You can view this list in your terminal by running:

```bash
drug-extraction -h
```

and you should see brief descriptions on each command:
![img](sample-command-help.png)

Each command can be further inspected by running `drug-extraction <COMMAND> -h`.
For example:

```bash
drug-extraction pipeline -h
```

which would output:

![img](sample-pipeline-help.png)

The command documentation available in this manner will _always_ be up to date, even when this
documentation site may be behind.

## Flags

The list of available flags are:

- clean
- format
- format-type
- strict
- id-col
- target-col

Some flags only correspond to certain commands and thus are not globally available.
For more information run `drug-extraction <command-of-interest> -h`
