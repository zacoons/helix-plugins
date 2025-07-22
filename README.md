# Configuration

Clone this repo, then modify `~/.config/helix/plugins.toml` as below:

```toml
sources = [
	# TODO: "git+https://github.com/zacoons/helix-plugins",
	"/path/to/helix-plugins",
]

enabled = [
	"hello_world",
	"parrot",
]

[hello_world]
str = "Hello world!"
```
