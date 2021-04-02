# Data

Validation data for the [CISTEM](https://github.com/LeonieWeissweiler/CISTEM) stemmer, copied from https://github.com/LeonieWeissweiler/CISTEM/pull/6#issuecomment-524457146.

Each line of `perl.txt` has the following tab-delimited fields: 

1. original word
2. stem
3. stem with case insensitive
4. first output of segment
5. second output of segment
6. first output of segment with case insensitive
7. second output of segment, new line;

The Rust translation produces exactly the same output.