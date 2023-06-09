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
openai dall-e generate "a white siamese cat"

# Generate with custom params
openai dall-e generate --n 4 --size 512 --dir "$HOME" "a white siamese cat"

# Edit with default params
openai dall-e edit ./image.png "A sunlit indoor lounge area with a pool containing a flamingo"

# Edit with default params
openai dall-e edit --mask ./mask.png --n 4 --size 512 --dir "$HOME" ./image.png "A sunlit indoor lounge area with a pool containing a flamingo"
```
