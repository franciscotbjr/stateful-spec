# Project Definition — Research / Study Project

> Pre-filled preset for a **studies** project: research whose deliverables are
> documents, figures, and datasets (reviews, analyses, reports, papers). Customize for
> your project. See [`methodology/project-types.md`](../methodology/project-types.md).

---

## Project Identity

- **Project Name:** [your-study]
- **Description:** [One-sentence description — e.g., a literature review of X]
- **Project Type:** studies
- **Repository URL:** [https://github.com/user/repo]
- **License:** [e.g., CC-BY-4.0 for text, MIT for code]

## Stack / Materials

- **Subject area:** [e.g., NLP, economic history]
- **Sources / corpus:** [e.g., bibliography in Zotero, primary-source PDFs, a dataset]
- **Tooling:** [e.g., LaTeX, Zotero, Python/R, Jupyter]

## Repository Structure

```
project/
├── references/               # Bibliography (.bib) and source PDFs
│   └── references.bib
├── notes/                    # Reading notes, annotations
├── analysis/                 # Notebooks / scripts (if any)
│   └── *.ipynb
├── data/                     # Raw and processed datasets
├── figures/                  # Generated figures and tables
├── paper/                    # LaTeX or Markdown manuscript
│   └── main.tex
└── README.md                 # Overview, method, reproduction steps
```

### Key Directories

| Directory | Purpose |
|-----------|---------|
| `references/` | Bibliography and primary sources |
| `analysis/` | Notebooks/scripts that produce results |
| `data/` | Datasets (raw + processed) |
| `figures/` | Generated figures and tables |
| `paper/` | The manuscript / report |

## Conventions

- **Citation style:** [e.g., APA, ABNT, IEEE]
- **Notation and symbols:** [describe the symbols and units used]
- **Figure / table / section numbering:** [e.g., figures numbered per section; tables captioned above]
- **Writing language and register:** [e.g., academic English]

## Verification

- Every claim traceable to a cited, resolvable source
- Analysis is reproducible from data + scripts (seeds and environment recorded)
- Internal consistency (no contradictory statements; consistent terminology/notation)

## Quality Gates

> Checklist (no commands to run). All must hold before the deliverable is considered done.

- [ ] All citations complete and resolvable
- [ ] No unsourced normative claims; no plagiarism
- [ ] Figures/tables referenced in text and numbered
- [ ] Analysis reproducible (seeds, scripts, environment noted)
- [ ] Argument coherent end to end

## Documentation

### Required Documentation Files

| File | Purpose |
|------|---------|
| README.md | Overview, method summary, reproduction steps |
| references.bib | Bibliography |
| paper/ | The manuscript or report |

### Documentation Style

- **Citations:** follow the chosen citation style consistently
- **Figures:** captioned and referenced from the text; regenerated from the committed pipeline

## Delivery / Distribution

- **Target venue/format:** [e.g., internal report, arXiv, journal]
- **Dataset sharing:** [e.g., repository, DOI, none]

## Constraints & Non-Negotiables

- No unsourced claims; every assertion is backed by a resolvable citation
- Analyses must be reproducible from committed data + scripts
- No plagiarism; quotations and close paraphrases are attributed
- Raw data is never silently edited — transformations live in scripts
