> [!WARNING]
> This code is currently being rewritten in Rust. You can check out the original code at [val.town/@jordan.tornado_bot](https://val.town/@jordan.tornado_bot). 

# @nwstornado.bsky.social
This is a rough mirror of the [NWS Tornado Twitter bot](https://twitter.com/nwstornado) but it's built to be distributed to multiple platforms. It's purely a tool for social media platforms and should not be relied upon for safety. Please see the official [NWS website](https://weather.gov) for time- (and life-) critical information.

## Check it out
You can check it out here: [@nwstornado.bsky.social](https://bsky.app/profile/nwstornado.bsky.social).

## How to run
1. Clone the repo:
```
git clone https://git.sr.ht/~jordanreger/nwstornado
```

2. Add a `.env` in the `src` directory:
```
touch src/.env
```

3. Add a file called `warning` in the `src` directory:
```
touch src/warning
```

4. Run it:
```
cargo run
```