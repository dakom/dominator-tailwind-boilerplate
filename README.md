# Boilerplate for Dominator w/ Tailwind (built w/ Trunk)

# [Live Demo](https://dakom.github.io/dominator-tailwind-boilerplate/)

Aside for just totally barebones dominator/tailwind/trunk setup, it has a couple extra bells and whistles:

1. Media served separately. Trunk doesn't have a way to serve media without copying everything over. That's fine for small projects, but gets unwieldy at scale. So instead we spin up a separate server for media and only copy it over to production at deploy time. This is also a good separation of concerns, e.g. if the media should come from some remote url anyway.

2. Github Pages index.html patching. Unnecessary for building on your own custom domain like https://example.com, but for serving under a github username w/ pages, the index.html needs to be patched so that everything works ok

Bottom line - pleasant dev and deploy experience, this boilerplate is ready to go and built to scale ;)

# Prerequisites

1. Install tooling - Rust, yarn, trunk, etc.
2. `yarn install`
3. Change `uri_root` in `app.config.json` for your relative url base

Might need to adapt Trunk.toml if on windows (it assumes `sh` exists, maybe, dunno)

# Dev

1. `yarn start`
2. visit http://127.0.0.1:8080/

Media is served at `http://127.0.0.1:9000` (and `/{uri_root}/media/*` in production) but this is abstracted away in code