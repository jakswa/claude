# Claude

It's a discord chatbot. I built it to support corny zingers
that we can't get the discord AI chatbot Clyde to engage with.

> Clyde: I'm sorry, @Jake, but as an AI language model, I'm not programmed to engage in personal attacks or insults. My primary purpose is to provide helpful and informative responses to users' questions, and I strive to do so in a respectful and professional manner.

And it a work in progress but right now it supports a `commands.toml` config in the root like:

```toml
# commands.toml.example

[[commands]]
matches = ['are you there']
[[commands.responses]]
is_reply = true
text = "yup <author>, I'm here!"

[[commands]]
# matches specially need '<<' and '>>', if you don't know the regex syntax
matches = ['say hi', 'say hi to <<target>>']
defaults = { target = "<author>" }
[[commands.responses]]
text = "hi <target>"

[[commands]]
matches = ['give me a cactus fact']
[[commands.responses]]
text = "Did you know..."
[[commands.responses]]
delay = { secs = 5, nanos = 0 }
choices = ["cactus are pokey", "cacti or cactus?", "cactus are us"]
```

With that config file, and with the bot running in your server, you could do:
```
you> @bot say hi
bot> hi @you
```
or
```
you> @boy say hit to @them
bot> hi @them
```

Commands can have multiple responses. The cactus one goes:
```
you> @bot give me a cactus fact
bot> Did you know...
(5s goes by, then a random choice)
bot> cactus are pokey
```
