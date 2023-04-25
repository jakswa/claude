# Claude

It's a discord chatbot.

Very WIP but right now it supports a `.toml` config file like:

```toml
[[prompts]]
triggers = ["are you there"]
prompt = "yup <author>, I'm here!"

[[prompts]]
triggers = ["say hi", "say hi to <target>"]
prompt = "hi <target>"
defaults = { target = "<author>" }

[[prompts]]
triggers = ["give me a cactus fact"]
prompt = "Did you know..."
answers = ["cactus are pokey", "cacti or cactus?", "cactus are us"]
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

The "answers" config is intended to be an added response, chosen randomly, and sent after waiting 5s right now.
