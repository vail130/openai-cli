# openai-cli

A CLI tool to interact with OpenAI APIs

## Configuration

Uses `OPENAI_API_KEY` env var (used by underlying library)

## Run

```sh
#############
# Dall-E
#############

# Generate with default params
openai dall-e generate "A cute baby sea otter"

# Generate with custom params
openai dall-e generate --n 4 --size 512 --dir "$HOME" "A cute baby sea otter"
```
