# Contributing to LVS Core

Thank you for your interest in contributing to **LVS Core**, the reference research implementation of the Living Value System (LVS) protocol.

This document explains how to work with the repository, how to propose changes, and how to maintain code quality and security.

---

## 1. Prerequisites

Before contributing, ensure you have:

- **Node.js 18+**
- **npm** or **yarn**
- **Git**
- Basic understanding of TypeScript

---

## 2. Repository Structure

lvs-core/
├── src/ # TypeScript simulation engine (logic + demo)
├── docs/ # Full PDF documentation and research papers
├── assets/ # Logos and media resources
├── LICENSE
├── SECURITY.md
├── GOVERNANCE.md
└── README.md

---

## 3. Running the project locally

```bash
npm install
npm run start
This launches the simulation engine (TS demo with multiple nodes and ticks).

4. How to Submit Changes
Step 1 — Fork the repository
Click “Fork” on GitHub.

Step 2 — Create a feature branch
bash
Копировать код
git checkout -b feature/my-improvement
Step 3 — Make changes
Follow code style, provide comments if logic is complex.

Step 4 — Commit

sql
Копировать код
git commit -m "Describe your change clearly"
Step 5 — Push and open a Pull Request
Your PR will be reviewed by the Core Maintainer.

5. Code Guidelines

Use TypeScript strict mode.

Prefer small, clean commits over one giant change.

Write clear variable names.

Provide inline comments for simulation logic.

Avoid adding unnecessary dependencies.

6. Documentation Contributions
PDF documentation belongs in:

css
Копировать код
docs/<section>/
All files must be in English.
Large updates should include a short description in the pull request.

7. Security Guidelines
Do not:

Publish vulnerabilities publicly.

Submit consensus-level changes without review.

Attempt to modify protocol protections without approval.

All vulnerabilities must be reported privately (see SECURITY.md).

8. Licensing
Documentation is under Apache License 2.0.
Protocol-level implementations may not be reused commercially.
See LICENSE for details.

9. Respect
By contributing, you agree to follow professional behavior and respect other contributors.
Abusive or hostile behavior results in removal from the project.
