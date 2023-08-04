# Eclipse Zenoh-Flow Examples

[![Join the chat at https://gitter.im/atolab/zenoh-flow](https://badges.gitter.im/atolab/zenoh-flow.svg)](https://gitter.im/atolab/zenoh-flow?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

Zenoh-Flow provides a zenoh-based dataflow programming framework for computations that span from the cloud to the device.

:warning: **This software is still in alpha status and should _not_ be used in production. Breaking changes are likely to happen and the API is not stable.**

-----------
## Description

Zenoh-Flow allow users to declare a dataflow graph, via a YAML file, and use tags to express location affinity and requirements for the operators that makeup the graph. When deploying the dataflow graph, Zenoh-Flow automatically deals with distribution by linking remote operators through zenoh.

A dataflow is composed of set of _sources_ — producing data, _operators_ — computing over the data, and _sinks_ — consuming the resulting data. These components are _dynamically_ loaded at runtime.

Remote source, operators, and sinks leverage zenoh to communicate in a transparent manner. In other terms, the dataflow the dafalow graph retails location transparency and could be deployed in different ways depending on specific needs.

Zenoh-Flow provides several working examples that illustrate how to define operators, sources and sinks as well as how to declaratively define they dataflow graph by means of a YAML file.

-----------
## Examples


### Getting Started

The purpose of this example is to introduce the different concepts of Zenoh-Flow by creating a
simple application that generates "Hello, World!" types of greetings.

Go to the [README](./getting-started/README.md) for instructions on how to run it.

---

### Period Miss detector

The purpose of this example is to showcase how one can implement a node that (i)
expects data at regular intervals and (ii) sends a default value if no data was
received within an interval.

Go to the [README](./period-miss-detector/README.md) for instructions on how to run it.

#### Montblanc

The purpose of this example is to demonstrate how Zenoh-Flow can handle a
complex dataflow graph, like a robotic application.

Go to the [README](./montblanc/README.md) for instructions on how to run it.


#### Transcoding

The purpose of this example is to demonstrate how Zenoh-Flow can handle be used within a Zenoh
router to transcode live data.

Go to the [README](./transcoding/README.md) for instructions on how to run it.
