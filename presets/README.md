# Presets

Pre-filled Project Definitions. Copy a preset as your starting point instead of filling the Project Definition template from scratch.

Most presets are **software-stack** starting points. For non-software project types, use the matching type preset (`skills-repo.md`, `studies-project.md`) or the type's project-definition skeleton in [`methodology/project-types.md`](../methodology/project-types.md). `software` is the default Project Type.

## How to Use

1. Find the preset that matches your stack (or is closest to it)
2. Copy the preset file to your project as your Project Definition
3. Customize the copied file to match your specific project (name, repo URL, dependencies, etc.)
4. Use it with the prompts in `prompts/`

## Available Presets

| Preset | Type | Stack / Materials | Best For |
|--------|------|-------------------|----------|
| [rust-library.md](rust-library.md) | software | Rust + Cargo + clippy/fmt | Rust libraries and crates |
| [node-express-api.md](node-express-api.md) | software | Node.js + Express + Jest + ESLint | REST API backends |
| [python-fastapi.md](python-fastapi.md) | software | Python + FastAPI + pytest + ruff | Python web APIs and services |
| [react-webapp.md](react-webapp.md) | software | React + TypeScript + Vite + Vitest | Frontend web applications |
| [go-service.md](go-service.md) | software | Go + standard library + go vet/test | Go backend services |
| [skills-repo.md](skills-repo.md) | skills | Markdown + YAML frontmatter; no build | Repositories of Agent Skills |
| [studies-project.md](studies-project.md) | studies | LaTeX / Zotero / Python / R | Research projects (reviews, analyses, papers) |

## Contributing a Preset

If you have a stack that's not covered, contributions are welcome. A good preset should:

1. Follow the same structure as `templates/project/project-definition.md`
2. Fill in realistic, production-ready values (not toy examples)
3. Include the most common quality gates for that stack
4. Document conventions that are idiomatic for the language/framework
