# hydra-v3

## OpenCode Integration

This repository is integrated with [OpenCode](https://opencode.ai), an AI coding agent powered by Claude.

### Usage

Mention `/opencode` or `/oc` in any issue or pull request comment to trigger the OpenCode agent.

### Setup

1. Add your AI provider API key as a repository secret:
   - Go to **Settings → Secrets and variables → Actions**
   - Add `ANTHROPIC_API_KEY` with your Anthropic API key

2. The workflow is defined in [`.github/workflows/opencode.yml`](.github/workflows/opencode.yml).