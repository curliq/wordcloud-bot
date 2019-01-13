# generator-bot
A discord bot that generates a wordcloud which is currently a sort of pyramid, from a user's messages.

TODO:
- Render the words in a circular shape
- Add different colors to each word
- Scan messages from beginning to end to preserve contextual sequences
- Improve wordcloud generator:
    - Remove pronouns
    - Remove space characters as expressions
    - Find a way to remove other words that build uncontextualizable expressions, e.g. "a", "of", etc