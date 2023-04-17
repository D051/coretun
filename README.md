# Coretun


## Overview

Coretun is a rust library to create virtual network interfaces in userspace. The whole project is designed to be as userfriendly as possible.

## Table of contents
* [Overview](#overview)
* [Plattforms](#plattforms)
* [Installation](#installation)
* [Setup](#setup)
* [Contributing](#contributing)
* [License](#license)

## Plattforms

Coretun supports linux and macos. Windows support will be added soon.

| Plattform     | Supported |
| :-----------: | :-------: |
| linux         | ✅        |
| macos         | ✅        |
| windows       | ❌        |


## Installation

Use the rusts package manager [cargo](https://crates.io) to integrate the library into your project.

```bash
cargo add coretun
```




## Setup

```rust
let interface = coretun::open("coretun");
```




## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.





## License

[MPL 2.0](https://www.mozilla.org/en-US/MPL/2.0/)