# Capability PoC

## Important setting

When using `cargo` and you are trying to fetch the lib repo from GitHub,
you might get problems with your credentials.
Workaround for this is to set this environmental variable:

```sh
> export CARGO_NET_GIT_FETCH_WITH_CLI=true
```

This will make you able to run the following commands without problems:

```sh
> cargo update
> cargo check / clippy / build # only the first time
```

