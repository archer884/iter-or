# iter-or

> An iterator adapter which provides a single default value in the event its source iterator comes up empty.

Simple code for a simple job. At least, I hope that's an accurate description.

## ...nitty-gritty details...

To be clear, what this adapter actually does is that it spits out a default value if the source adapter spits out a `None` value *and* the source adapter has not already produced any other value. This means that, in theory, if your source adapter first produces a `None` and *then* produces something else it is possible to get both the realized default value followed by the other values afterward. This is not complicated code. In fact, here it is:

    iter::once(self.next().unwrap_or(item)).chain(self)

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE][license-url-ap2] or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT][license-url-mit] or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

[license-url-mit]: https://github.com/archer884/iter-or/blob/master/LICENSE-MIT
[license-url-ap2]: https://github.com/archer884/iter-or/blob/master/LICENSE-APACHE
