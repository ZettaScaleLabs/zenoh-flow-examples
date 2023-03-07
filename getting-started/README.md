# Getting started

The purpose of this example is to introduce the different concepts of Zenoh-Flow by creating a
simple application that generates "Hello, World!" types of greetings.

## How to run

### Build (for Rust nodes only)

We first generate the different shared libraries of the different nodes.

```shell
cd ~/dev/zenoh-flow-examples/getting-started/nodes/rust/ && cargo build --workspace
```

### Update the paths

For each YAML file in the list below, check that the paths and filenames are
correct:
- data-flow.yaml
- Rust nodes:
  - nodes/rust/period-miss-detector/period-miss-detector.yaml
  - nodes/rust/file-writer/file-writer.yaml
- Python nodes:
  - nodes/python/period-miss-detector/period-miss-detector.yaml
  - nodes/python/file-writer/file-writer.yaml

:bulb: Note that you actually only need to update the files of the nodes you are going to use —
which could be a mix of Python and Rust nodes.

### Launch

#### 1st terminal: Zenoh

```shell
cd ~/dev/zenoh && ./target/debug/zenohd -c ~/.config/zenoh-flow/zenoh.json
```

#### 2nd terminal: Zenoh-Flow daemon

```shell
cd ~/dev/zenoh-flow/ && ./target/debug/zenoh-flow-daemon -c ~/.config/zenoh-flow/runtime.yaml
```

#### 3rd terminal: launch the flow

```shell
cd ~/dev/zenoh-flow && ./target/debug/zfctl launch ~/dev/zenoh-flow-examples/getting-started/data-flow.yaml
```

Then, if the flow was successfully launched, put values at regular intervals:

```shell
# If you have compiled the `z_put` example of Zenoh in debug
$ZENOH/target/debug/examples/z_put -k "zf/getting-started/hello" -v "Alice"

# If you have enabled the REST plugin of Zenoh
curl -X PUT -H "content-type:text/plain" -d 'Bob' http://localhost:8000/zf/getting-started/hello
```
