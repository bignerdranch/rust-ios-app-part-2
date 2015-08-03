# Let's Build an iOS App in Rust, part 2

This is the repository accompanying the [Let's Build an iOS App in Rust, part
2][TODO-LINK] blog post.

## Building and Running

After cloning the repo, go into the `rust/data_passing` directory and run
`make`. This requires you to have a working Rust-to-iOS toolchain; see [this
blog post][TODO-LINK] for instructions on setting up `multirust` appropriately.
(Don't forget to `multirust override ios` this project directory.)

After `libdata_passing.a` is built, open up
`ios/RustDataPassing/RustDataPassing.xcodeproj` and run the app.

## Author

John Gallagher, jgallagher@bignerdranch.com

## License

This sample app is available under the MIT license. See the LICENSE file for
more info.
