
# GHOST

A command-line utility to save and restore terminal sessions.


## Installing

Installing using cargo:

```bash
  cargo install --git https://github.com/ruddpj/ghost.git --bin ghost
```


## Usage

Saving a session:

```bash
  ghost save <name>
```

Loading a session:

```bash
  cargo load <name>
```

Deleting a session:

```bash
  cargo delete <name>
```
All sessions are saved to (and loaded from): 
```~/ghost-snapshots/```


## License
This project is licensed under the 
[MIT](https://choosealicense.com/licenses/mit/) licence.
