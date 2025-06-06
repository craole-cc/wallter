# Walter

A simple, command-line driven slideshow and wallpaper manager.

## Features

* Downloads wallpapers from [Wallhaven](https://wallhaven.cc/) and sets them as your desktop background
* Supports multiple monitors and resolutions
* Supports custom commands to run before and after setting the wallpaper
* Supports custom image directories

## Installation

### From source

1. Clone the repository: `git clone https://github.com/benfrain/walter.git`
2. Change into the directory: `cd walter`
3. Build the project: `cargo build --release`
4. Install the project: `cargo install --path .`

### From crates.io

1. Install the project: `cargo install walter`

## Usage

### Initializing the configuration file

`walter init`

### Downloading wallpapers

`walter download`

### Setting the wallpaper

`walter set`

### Running the slideshow

`walter slideshow`

### Customizing the configuration

`walter config`

## Configuration

The configuration file is located at `~/.config/walter/config.toml`. You can customize the following settings:

* `api_key`: Your Wallhaven API key
* `download_dir`: The directory to download wallpapers to
* `image_dir`: The directory to use for the slideshow
* `custom_commands`: A list of custom commands to run before and after setting the wallpaper
* `monitors`: A list of monitor IDs and their corresponding resolutions
* `slideshow_interval`: The interval to wait between slideshow images
* `slideshow_unit`: The unit of time for the slideshow interval

## License

This project is licensed under the MIT license. See [LICENSE](LICENSE) for more information.
