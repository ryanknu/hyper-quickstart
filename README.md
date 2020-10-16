# Hyper Quickstart
A hyper server that implements CORS and Auth0 out-of-box. This is meant to be a very fast get up and go and I do not claim that it is perfect. It is designed to get you a working HTTP server with CORS and Auth0 authentication as fast as possible so you can simply start working on building functions as soon as possible. I am not a Rust expert, just an enthusiast, pull requests are welcome.

*Note* It is helpful to already have a front end app that is connected to Auth0 and able to retrieve access tokens before trying to get the server going, as by default the server literally does nothing until you have working Auth0 access tokens to send it.

*Another Note* This server is designed to be ran on Google Cloud Run, so we do not issue any session tokens or anything like that. It is possible we could use JWT's from Auth0 to prevent spamming the Auth0 API, but, this is what works for me right now.

# How to Use
1. Download repository as a zip file (do not fork)
2. Edit the project name and author in `Cargo.toml`
3. Create the file `.env` and place the following content keys in it: `PORT`, `AUTH0_DOMAIN`.
4. Edit `cors.rs` to add any additional hosts your app will run on. I usually return your production host for the default result.
5. Run `cargo run`
6. The server should start.
7. Start building functions! You can build complex routing and "middleware" in the `route` function in `main.rs`.

# Built-in functions:

```
/     -> returns "OK"
/user -> returns your auth0 user ID
/path -> returns the requested path
```