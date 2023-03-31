# Boilerplate for Dominator w/ Tailwind

This isn't actively maintained, but, should work fine :)

Aside for just totally barebones dominator/tailwind setup, it has a couple extra bells and whistles:

1. Media served separately. Trunk doesn't have a way to serve media without copying everything over. That's fine for small projects, but gets unwieldy at scale. So instead we spin up a separate server for media and only copy it over to production at deploy time

2. Github path patching. Unnecessary for building on your own custom domain like https://example.com, but for serving under a github username w/ pages, the output needs to be patched so that everything works ok

Bottom line - pleasant dev and deploy experience, this boilerplate is built to scale ;)

# Prerequisites

1. Install tooling - Rust, yarn, trunk, etc.
2. `yarn install`
3. Change `BASE_URL` in `app.config.json` for your relative url base

Might need to adapt Trunk.toml if on windows (it assumes `sh` exists)

# Dev

1. `yarn start`
2. visit http://127.0.0.1:8080/

Media is served at `http://127.0.0.1:9000` but this is abstracted away in code