# rustant-film

Add an instant-film-style layout to your EXIF photos.

## Samples

| index | layout | image |
|---|---|---|
| 1 | Diagonal, pos left with paddings | ![Image](https://github.com/user-attachments/assets/877dbc5e-ad2a-40cf-b3fe-6cfbaa1cd5b5) |
| 2 | Diagonal, pos right | ![Image](https://github.com/user-attachments/assets/0b0698b4-f722-40c3-9c75-35b68f3a4079) |
| 3 | Duel, pos left | ![Image](https://github.com/user-attachments/assets/10128f3b-a28a-4bca-b01f-6e5b1257a26d) |
| 4 | Duel, pos right with paddings | ![Image](https://github.com/user-attachments/assets/50a7cf98-63a4-4c09-93a6-532e89c5d970) |
| 6 | Triangular, pos bottom with paddings | ![Image](https://github.com/user-attachments/assets/163e743e-8e43-4d82-8333-f9f6426f5edc) |
| 7 | Triangular, pos bottom | ![Image](https://github.com/user-attachments/assets/2f983576-f21a-4766-83a6-6d0b69e0998e) |

## Features

- Written in Rust.
- Provides both command line & server API.
- Customizable logo & font.
- Cross-platfrom by cargo.

## Quick Start

To get `rustant-film`, use:

```shell
cargo install rustant-film
```

Or to build it from source: 

```shell
cargo build --release
```

It's recommended to clone this repository and go through the next tutorial section. 

## Tutorial

To get some materials to develop a good image result, you should prepare your own fonts and logos. You can find a example material setup scripts in [setup-example](setup-example)

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
rustant-film -i ./resources/samples -o ./output -f ./resources/font/ttf/FiraCode-SemiBold.ttf -l ./resources/logos
```

The output will be saved in the `./output` directory.

For a more detailed guide on preparing media resources, refer to [here](./setup-example/readme.md).

## Layouts

Currently, `rustant-film` supports the following layouts:

- `triangle`: A traditional instant film layout with EXIF information displayed below.
- `blank`: A raw instant film with only blank paddings, no extra info added.
- `duel`: A layout with EXIF information displayed on left or right.
- `diagonal`: A layout like `duel` by display EXIF information on top-left or bottom-right.

```shell
rustant-film -i ./resources/samples -o ./output -f ./resources/font/ttf/FiraCode-Regular.ttf -l ./resources/logos -p triangle
```

For a more classic instant film style, add padding around by using flag `-pad`:

```shell
rustant-film -i ./resources/samples -o ./output -f ./resources/font/ttf/FiraCode-Regular.ttf -l ./resources/logos -p triangle --pad
```

Some layout may use a sub-font to get a better look. To use a sub-font, use `--sub-font`:

```shell
rustant-film -i ./resources/samples -o ./output -f ./resources/font/ttf/FiraCode-SemiBold.ttf --sub-font ./resources/font/ttf/FiraCode-Regular.ttf -l ./resources/logos -p triangle --pad
```

## Server

In addition to the command-line interface, `rustant-film` can also operate as an HTTP server. This allows you to generate and download files via RPC.

To run `rustant-film` in server mode, use the command like:

```shell
rustant-film -m server -f ./resources/font/ttf/FiraCode-SemiBold.ttf --sub-font ./resources/font/ttf/FiraCode-Regular.ttf
```

Once the server is running, you can send requests to the API. For example, to process an image, use the following `curl` command:

```shell
curl --output result.jpg 'http://0.0.0.0:6400/api/v1/develop?painter=diagonal&pad=false&pos=r' \
    -X POST \
    -H "Content-Type: multipart/form-data" \
    -F 'image=@./test.jpg'
```

This will generate and return the processed image as `result.jpg`.

## Roadmap

`rustant-film` aims to implement the following features in future versions:

- `More Layout`: Additional instant film layout options.
- `Adaptive Layout`: Automatically adjusts the layout for photos with missing EXIF data.
- `Watermark`: Adds watermarks, including invisible ones, to processed photos.
- `Copyright Support`: Allows users to add custom copyright information, such as the artist's name, to the final photo.
- `Async HTTP API`: Hosts the `rustant-film` command as an HTTP API server, providing asynchronous APIs.
- `GUI`: Add a GUI for `rustant-film`, maybe as a new project.
- `Container Image`: Distributes the `rustant-film` command-line tool and HTTP API as a container image.
- `Cargo Binary Publishing`: Publishes the `rustant-film` command as a binary on crates.io.
- `Lens Logo Support`: Some layouts may include lens logos.

## Credit

### Sample Fonts

- "[FiraCode](https://github.com/tonsky/FiraCode)" by Nikita Prokopov, used under OFL-1.1 license.

### Sample Images

`rustant-film` will add instant film layout to the following images in showcase.

- "[Dahlia-in-bloom](https://commons.wikimedia.org/wiki/File:Dahlia-in-bloom.jpg)" by Changku88, used under CC BY-SA 4.0. <!-- Apple -->
- "[Kranhäuser Cologne, April 2018-01](https://commons.wikimedia.org/wiki/File:Kranh%C3%A4user_Cologne,_April_2018_-01.jpg)" by Martin Falbisoner, used under CC BY-SA 4.0. <!-- Canon -->
- "[Dendermonde town hall and belfry during golden hour](https://commons.wikimedia.org/wiki/File:Dendermonde_town_hall_and_belfry_during_golden_hour_(DSCF0501).jpg)" by Trougnouf, used under CC BY-SA 4.0. <!-- Fujifilm -->
- "[Annunciazione Santuario del Carmine San Felice del Benaco](https://commons.wikimedia.org/wiki/File:Annunciazione_Santuario_del_Carmine_San_Felice_del_Benaco.jpg)" by Wolfgang Moroder, used under CC BY-SA 4.0. <!-- Hasselblad -->
- "[Boat passing under Elgin Bridge during blue hour (2023)](https://commons.wikimedia.org/wiki/File:Boat_passing_under_Elgin_Bridge_during_blue_hour_(2023)-L1003785.jpg)" by Frank Schulenburg, used under CC BY-SA 4.0. <!-- Leica -->
- "[Puppy de Jeff Koons -- 2021 -- Bilbao, España](https://commons.wikimedia.org/wiki/File:Puppy_de_Jeff_Koons_--_2021_--_Bilbao,_Espa%C3%B1a.jpg)" by Jose María Ligero Loarte, used under CC BY-SA 4.0. <!-- Nikon -->
- "[Love padlocks on the Butchers' Bridge (Ljubljana)](https://commons.wikimedia.org/wiki/File:Love_padlocks_on_the_Butchers%27_Bridge_(Ljubljana).jpg)" by Petar Milošević, used under CC BY-SA 4.0. <!-- Olympus -->
- "[Akelei Blüte geschlossen stacking-20230506-RM-120202](https://commons.wikimedia.org/wiki/File:Akelei_Bl%C3%BCte_geschlossen_stacking-20230506-RM-120202.jpg)" by Ermell, used under CC BY-SA 4.0. <!-- OM Digital Solutions -->
- "[Parboiled rice with chicken, peppers, cucurbita, peas and tomato](https://commons.wikimedia.org/wiki/File:Parboiled_rice_with_chicken,_peppers,_cucurbita,_peas_and_tomato.jpg)" by Petar Milošević, used under CC BY-SA 4.0. <!-- Panasonic -->
- "[York Railway Museum in Evening Lights - Pastel Theme](https://commons.wikimedia.org/wiki/File:York_Railway_Museum_in_Evening_Lights_-_Pastel_Theme.jpg)" by Nedian91, used under CC BY-SA 4.0. <!-- Pantex -->
- "[D-6-73-141-83 Sühnekreuz in der Merklach 3](https://commons.wikimedia.org/wiki/File:D-6-73-141-83_S%C3%BChnekreuz_in_der_Merklach_3.jpg)" by Stephan van Helden, used under CC BY-SA 4.0. <!-- Ricoh -->
- "[Khairdeen urf Pritam, Black and White](https://commons.wikimedia.org/wiki/File:Khairdeen_urf_Pritam,_Black_and_White.jpg)" by Satdeep Gill, used under CC BY-SA 4.0. <!-- Sony -->
