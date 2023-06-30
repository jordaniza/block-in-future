# Block number from Timestamp

Don't know what to tell you really: put in the unix timestamp in the future. Get an expected block number.

# Installing

Download from source and build with `cargo build --release`

In the `target/releases` folder you'll have the binary. I personally just alias it to `eth-bn`, but you can also drop in your /usr binaries as needed. 

## Windows and MacOS

I think MacOS should work. Windows no idea. 

# Running

Run it with:

```sh
$ eth-bn -t 1688166730`
1688166730
```

Powerful stuff.

