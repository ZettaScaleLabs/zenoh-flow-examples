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


### Runtime
First, let's build an example runtime to run the examples
```bash
cargo build --release -p runtime
```

This will create the runtime binary in `./target/release/runtime`

---

### FizzBuzz

First, compile the relevant examples:

```bash
cargo build --release -p manual-source -p example-fizz -p example-buzz -p generic-sink
```

This will create, depending on your OS, the libraries that the pipeline will fetch.

#### Single runtime

To run all components on the same Zenoh Flow runtime:

```bash
./target/release/runtime --graph-file ./graphs/fizz_buzz_pipeline.yaml --runtime foo
```

_Note: in that particular case the `--runtime foo` is discarded._

#### Multiple runtimes

In a first machine, run:

```bash
./target/release/runtime --graph-file ./graphs/fizz-buzz-multiple-runtimes.yaml --runtime foo
```

In a second machine, run:

```bash
./target/release/runtime --graph-file ./graphs/fizz-buzz-multiple-runtimes.yaml --runtime bar
```

:warning: If you change the name of the runtime in the yaml file, the name(s) passed as argument of the previous commands must be changed accordingly.

:warning: Without configuration, the different machines need to be on the _same local network_ for this example to work. See how to add a [Zenoh router](https://zenoh.io/docs/getting-started/key-concepts/#zenoh-router) if you want to connect them through the internet.

---

### OpenCV FaceDetection - Haarcascades

:warning: This example works only on Linux and it require OpenCV to be installed, please follow the instruction on the [OpenCV documentation](https://docs.opencv.org/4.5.2/d7/d9f/tutorial_linux_install.html) to install it.

:warning: You need a machine equipped of a webcam in order to run this example.

First, compile the relevant examples:

```bash
cargo build --release -p camera-source -p face-detection -p video-sink
```

This will create, depending on your OS, the libraries that the pipeline will fetch.

#### Single runtime

To run all components on the same Zenoh Flow runtime:

```bash
./target/release/runtime --graph-file ./graphs/face_detection.yaml --runtime foo
```

_Note: in that particular case the `--runtime foo` is discarded._

#### Multiple runtimes

In a first machine, run:

```bash
./target/release/runtime --graph-file ./graphs/face-detection-multi-runtime.yaml --runtime gigot
```

In a second machine, run:

```bash
./target/release/runtime --graph-file ./graphs/face-detection-multi-runtime.yaml --runtime nuc
```

In a third machine, run:

```bash
./target/release/runtime --graph-file ./graphs/face-detection-multi-runtime.yaml --runtime leia
```

:warning: If you change the name of the runtime in the yaml file, the name(s) passed as argument of the previous commands must be changed accordingly.

:warning: Without configuration, the different machines need to be on the _same local network_ for this example to work. See how to add a [Zenoh router](https://zenoh.io/docs/getting-started/key-concepts/#zenoh-router) if you want to connect them through the internet.

---

### OpenCV Object Detection - Deep Neural Network - CUDA powered

:warning: This example works only on Linux and it require OpenCV with CUDA enabled to be installed, please follow the instruction on [this gits](https://gist.github.com/raulqf/f42c718a658cddc16f9df07ecc627be7) to install it.

:warning: This example works only on Linux and it require a **CUDA** capable **NVIDIA GPU**, as well as NVIDIA CUDA and CuDNN to be installed, please follow [CUDA instructions](https://docs.nvidia.com/cuda/cuda-installation-guide-linux/index.html) and [CuDNN instructions](https://docs.nvidia.com/deeplearning/cudnn/install-guide/index.html).

:warning: You need a machine equipped of a webcam in order to run this example.

:warning: You need to download a YOLOv3 configuration, weights and classes, you can use the ones from [this GitHub repository](https://github.com/sthanhng/yoloface).

First, compile the relevant examples:

```bash
cargo build --release -p camera-source -p object-detection-dnn -p video-sink
```

This will create, depending on your OS, the libraries that the pipeline will fetch.

Then please update the files `./graphs/dnn-object-detection.yaml` and `./graphs/dnn-object-detection-multi-runtime.yaml` by changing the `neural-network`, `network-weights`, and `network-classes` to match the absolute path of your *Neural Network* configuration

#### Single runtime

To run all components on the same Zenoh Flow runtime:

```bash
./target/release/runtime --graph-file ./graphs/dnn-object-detection.yaml --runtime foo
```

_Note: in that particular case the `--runtime foo` is discarded._

#### Multiple runtimes

In a first machine, run:

```bash
./target/release/runtime --graph-file ./graphs/dnn-object-detection-multi-runtime.yaml --runtime foo
```

In a second machine, run:

```bash
./target/release/runtime --graph-file ./graphs/dnn-object-detection-multi-runtime.yaml --runtime cuda
```

In a third machine, run:

```bash
./target/release/runtime --graph-file ./graphs/dnn-object-detection-multi-runtime.yaml --runtime bar
```

:warning: If you change the name of the runtime in the yaml file, the name(s) passed as argument of the previous commands must be changed accordingly.

:warning: Without configuration, the different machines need to be on the _same local network_ for this example to work. See how to add a [Zenoh router](https://zenoh.io/docs/getting-started/key-concepts/#zenoh-router) if you want to connect them through the internet.


### OpenCV Car Vision - Deep Neural Network - CUDA powered

![Car vision dataflow](./car-pipeline.png)

:warning: This example works only on Linux and it require OpenCV with CUDA enabled to be installed, please follow the instruction on [this gits](https://gist.github.com/raulqf/f42c718a658cddc16f9df07ecc627be7) to install it.

:warning: This example works only on Linux and it require a **CUDA** capable **NVIDIA GPU**, as well as NVIDIA CUDA and CuDNN to be installed, please follow [CUDA instructions](https://docs.nvidia.com/cuda/cuda-installation-guide-linux/index.html) and [CuDNN instructions](https://docs.nvidia.com/deeplearning/cudnn/install-guide/index.html).

:warning: You need a machine equipped of a webcam in order to run this example.

:warning: You need to download a YOLOv3 configuration, weights and classes, you can use the ones from [this GitHub repository](https://github.com/tooth2/YOLOv3-Object-Detection/tree/main/dat/yolo).

:warning: You need to download a camera car video,  you can use the ones from [this data set](https://www.mrpt.org/Karlsruhe_Dataset_Rawlog_Format).
This dataset contains the frames, in oder to merge them in video you need `ffmpeg` and run the following command: `ffmpeg -framerate 15 -pattern_type glob -i 'I1*.png' -c:v libx264 I1.mp4`.

First, compile the relevant examples:

```bash
cargo build --release -p video-file-source -p object-detection-dnn -p video-sink
```

This will create, depending on your OS, the libraries that the pipeline will fetch.

Then please edit the file `./graphs/car-pipeline-multi-runtime.yaml` by changing the `neural-network`, `network-weights`, and `network-classes` to match the absolute path of your *Neural Network* configuration.

You also need to edit the `file` in `./graphs/car-pipeline-multi-runtime.yaml` to match the absolute path of your video file.

#### Multiple runtimes

In a first machine, run:

```bash
./target/release/runtime --graph-file ./graphs/car-pipeline-multi-runtime.yaml --runtime gigot
```

In a second machine, run:

```bash
./target/release/runtime --graph-file ./graphs/car-pipeline-multi-runtime.yaml --runtime cuda
```

In a third machine, run:

```bash
./target/release/runtime --graph-file ./graphs/car-pipeline-multi-runtime.yaml --runtime macbook
```

:warning: If you change the name of the runtime in the yaml file, the name(s) passed as argument of the previous commands must be changed accordingly.

:warning: Without configuration, the different machines need to be on the _same local network_ for this example to work. See how to add a [Zenoh router](https://zenoh.io/docs/getting-started/key-concepts/#zenoh-router) if you want to connect them through the internet.
