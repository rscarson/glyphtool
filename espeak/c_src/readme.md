# Espeak-mini

- This is just [libespeakng](https://github.com/espeak-ng/espeak-ng)!
- I have modified it to be callable independently from the filesystem and local install
- It is a fully static library, that can accept data files as buffers instead of actual files

Compile it with `make all`
- You will find the library in `./lib`, along with the header you need
- The example directory contains a simple example of how to use the library to translate phonemes (that's the only part I have modified for now)


espeak-ng is the copyright of its respective owners, I have only modified it to be more portable and easier to use in my project.

This project is licensed under GPLv3, just like espeak-ng itself.
