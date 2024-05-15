# rust-state-machine-mdbook

[Rust state machine tutorial](https://github.com/shawntabrizi/rust-state-machine) rendered by [mdBook](https://github.com/rust-lang/mdBook).

## Generate

To re-generate the book:

1. Install node dependencies:

	```sh
	yarn --cwd generate
	```

2. Run the `generate` script:

	```sh
	yarn --cwd generate rust-state-machine
	```

3. Remove the existing `src` folder:

	```sh
	rm -rf src
	```

4. Move the generated `rust-state-machine` folder into the root with the name `src`.

	```sh
	mv generate/rust-state-machine src
	```
