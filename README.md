# recb
 A tool to encrypt images with a block cipher in ECB mode that still clearly shows the outline of the original. 
 
 When encrypting in ECB mode, identical plaintext blocks are encrypted into identical ciphertext blocks. Thus, if there are repetitive patterns in the original image data, these patterns might still be somewhat visible in the encrypted image because areas of the image that have the same color (and thus the same data representation) will look the same after encryption.

