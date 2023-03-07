# Period miss detector

The purpose of this example is to showcase how one can implement a node that (i)
expects data at regular intervals and (ii) sends a default value if no data was
received within an interval.

## How to run

### Build

We first generate the different shared libraries of the different nodes.

```shell
cd ~/dev/zenoh-flow-examples/period-miss-detector/nodes/rust/ && cargo build --workspace
```

### Update the paths

For each YAML file in the list below, check that the paths and filenames are
correct:
- data-flow.yaml
- nodes/period-miss-detector.yaml
- nodes/file-writer.yaml

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
cd ~/dev/zenoh-flow && ./target/debug/zfctl launch ~/dev/zenoh-flow-examples/period-miss-detector/data-flow.yaml
```

Then, if the flow was successfully launched, put values at regular intervals:

```shell
cd ~/dev/zenoh && ./target/debug/examples/z_put -k "zf/period-miss-detector" -v "3.1416"
```
