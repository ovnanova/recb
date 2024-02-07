# recb
 A tool to encrypt images with a block cipher in ECB mode that still clearly shows the outline of the original. 
 
 When encrypting in ECB mode, identical plaintext blocks are encrypted into identical ciphertext blocks. Thus, if there are repetitive patterns in the original image data, these patterns might still be somewhat visible in the encrypted image because areas of the image that have the same color (and thus the same data representation) will look the same after encryption.

# Usage
 Install ImageMagick and make sure it's invokable from the command line.

 From the project directory:
 ```
 cargo run path_to_image
 ```
 The output quality will vary depending on the complexity of the source image (simpler images are better) and the passphrase. Try different passphrases and see what happens!

 An image of Tux is included for historical & testing purposes.

# Source
 The tool follows the following process:
 ```bash
 # First convert the Tux to PPM
 # Then take the header apart
 head -n 4 Tux.ppm > header.txt
 tail -n +5 Tux.ppm > body.bin
 # Then encrypt with ECB (experiment with some different keys)
 openssl enc -aes-128-ecb -nosalt -pass pass:"ANNA" -in body.bin -out body.ecb.bin
 # And finally put the result together and convert to some better format
 cat header.txt body.ecb.bin > Tux.ecb.ppm
 ```
 by: [Filippo Valsorda](https://words.filippo.io/the-ecb-penguin/)