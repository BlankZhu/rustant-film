# rustant-film

Add an instant-film-style layout to your EXIF photos.

## Samples

(Example photos with the rustant-film-style layout go here...)

## Quick Start

To get started, first compile `rustant-film` using `cargo`:

```
cargo build --release
```

Then run the following command to setup some essential meterials:

```shell
# You may need to install some commands used by the scripts, such as 'wget', 'unzip', and 'convert'.
sh ./setup-example/font.sh
sh ./setup-example/logo.sh
sh ./setup-example/sample.sh
```

This will generate example fonts, logos, and a sample photo under the `./resources` directory.

Finally, run the following command:

```shell
./target/release/rustant-film -i ./resources/samples -o ./output -f ./resources/font/ttf/FiraCode-Regular.ttf -l ./resources/logos
```

The output will be saved in the `./output` directory.

For a more detailed guide on preparing media resources, refer to [here](./setup-example/readme.md).

## Layouts

Currently, `rustant-film` supports the following layouts:

- `triangle`: A traditional instant film layout with EXIF information displayed below.

```shell
./target/release/rustant-film -i ./resources/samples -o ./output -f ./resources/font/ttf/FiraCode-Regular.ttf -l ./resources/logos -p triangle
```

For a more classic instant film style, add padding around by using flag `-pad`:

```shell
./target/release/rustant-film -i ./resources/samples -o ./output -f ./resources/font/ttf/FiraCode-Regular.ttf -l ./resources/logos -p triangle -pad
```

## Roadmap

`rustant-film` aims to implement the following features in future versions:

- `More Layout`: Additional instant film layout options.
- `Adaptive Layout`: Automatically adjusts the layout for photos with missing EXIF data.
- `Watermark`: Adds watermarks, including invisible ones, to processed photos.
- `Copyright Support`: Allows users to add custom copyright information, such as the artist's name, to the final photo.
- `HTTP API`: Hosts the `rustant-film` command as an HTTP API server, providing both synchronous and asynchronous APIs.
- `Container Image`: Distributes the `rustant-film` command-line tool and HTTP API as a container image.
- `Cargo Binary Publishing`: Publishes the `rustant-film` command as a binary on crates.io.
- `Lens Logo Support`: Some layouts may include lens logos.