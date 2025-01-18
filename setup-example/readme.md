# Setup Media Resources

These scripts will guide you through setting up sample media resources, including logos, fonts, and a sample photo.

**ATTENTION**: To avoid potential copyright issues, the scripts only provide links to download the media resources, not the files themselves. Availability of these resources is not guaranteed.

## Getting Started

Ensure you are in the `setup-example` directory:

```shell
cd ./setup-example
```

## Setting Up Logos

Logos will be included in the `rustant-film` layout. To set up example logos, run:

```shell
sh logo.sh
```

This script will download logos of common camera manufacturers into `./resources/logos` and convert them to JPEG format.

The logo filenames should match the `Make` information in the EXIF data of your photo. To verify the `Make` field of a photo, use the following command:

```shell
exiftool ./your-image.jpg | grep Make
```

**We recommend obtaining your own versions of these logos.**

## Setting Up Fonts

In this example, we use `FiraCode` as the font for rendering EXIF information in the rustant-film layout. To set it up, run:

```
sh font.sh
```

The font files will be saved in the `./resources/font` directory. Since `rustant-film` only supports TrueType fonts (.ttf), ensure you select a .ttf file from `./resources/font/ttf`.

As with the logos, **we highly recommend using custom fonts of your choice**.

## Setting Up a Sample

`rustant-film` relies on EXIF information embedded in photos. To set up a sample image, run:

```
sh sample.sh
```

This command will download a sample image from Wikimedia Commons. However, **it's better to use photos from your own camera for more accurate results**.

To check the EXIF data of your photo, use:

```shell
exiftool ./your-image.jpg
```

## kicking off

After completing the preparation steps, your `resources` directory should look like this:

```
├── font
│   ├── ...
│   ├── ttf
│   │   ├── ...
│   │   └── FiraCode-SemiBold.ttf
│   └── ...
├── logos
│   ├── apple.jpg
│   ├── canon.jpg
│   ├── ...
│   └── sony.jpg
└── samples
    ├── ...
    └── IMAGE.jpg
```

### Running with Cargo

If you are in a Rust development environment, use the following command:

```shell
# Run from the repository's base directory
cargo run -- -i ./setup-example/resources/samples/ -o ./output -l ./setup-example/resources/logos/ -f ./setup-example/resources/font/FiraCode-SemiBold.ttf -p normal
```

The output will be saved in the `./output` directory.

### Running from the Command Line

Ensure you have built the binary using `cargo run`. Then, execute:

```shell
# Start from the repository's base directory
cd ./setup-example
rustant-film -i ./resources/samples/ -o ./output -l ./resources/logos/ -f ./resources/font/FiraCode-SemiBold.ttf -p normal
```

The processed images will be saved in the `./output` directory.

*(Note: The example output image may lack lens information because the original image's EXIF data does not include lens details.)*

## Cleaning Up

To remove the setup resources, simply delete the `./resources` directory:

```shell
rm -rf ./resources
```