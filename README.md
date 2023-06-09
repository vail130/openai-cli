# openai-cli

A CLI tool to interact with OpenAI APIs

## Configuration

Uses `OPENAI_API_KEY` env var (used by underlying library)

## Run

```sh
#############
# Images Service
#############

# Create image with default params
openai images create "a white siamese cat"

# Create images with custom params
openai images create --n 4 --size 512 --dir "$HOME" "a white siamese cat"

# Edit with default params
openai images edit ./image.png "A sunlit indoor lounge area with a pool containing a flamingo"

# Edit with default params
openai images edit --mask ./mask.png --n 4 --size 512 --dir "$HOME" ./image.png "A sunlit indoor lounge area with a pool containing a flamingo"
```
