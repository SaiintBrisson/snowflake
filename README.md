
## DISCLAIMER

The original crate can generate the same ID twice, this fork fixes it.

# SnowFlake

The `snowflake` crate(not available on cargo currently) is an implement of [twitter's snowflake algorithm](https://github.com/twitter/snowflake) written in rust.
The bits are organized as follows:

- 1 -> Future use
- 41 -> Epoch
- 10 -> Worker ID
- 12 -> Sequence counter, 0 through 4096

TODO:  

- Make the epoch adjustable

Created by [h_ang!(J27);](mailto:hunagjj.27@qq.com)
Refactored by [saiintbrisson](mailto:luizcarlosmpc@gmail.com)
