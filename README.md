# WebP Image Processor

This script converts the inputed directory contents to a webp format. 

### Usage
In the terminal goto the project folder and build:
`> cargo build --release`

Then from the project folder run:
`> ./target/release/ImageProcessor "/Documents/ImagesToOptomiseFolder"
`
This will iterate through each of the images in this folder and save a new webp version in the webp folder it
creates in the specified folder.