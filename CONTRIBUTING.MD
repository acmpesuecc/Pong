## Getting the code locally
Create a fork of the repository and create a clone of your fork
```bash
git clone <your-fork-url>
cd Pong
```

## Execution Instructions
Inside the `Pong` directory
```bash
cargo build
cargo run
```

## Branching
Create a descriptive branch from `main` for your work
```bash
git checkout main
git pull origin main
git checkout -b feat/short-description
```

## Submitting a PR
Push your branch to your fork
```bash
git add .
git commit -m "Meaningful description of your changes, keep it concise"
git push -u origin feat/short-description
```

Open a PR against `acmpesuecc:main` (the main repo). Include a PR description:
- What issue you worked on
- Any setup needed to test it
- Screenshots(if relevant)

## Additional guidelines
- Keep functions small and focused
- Add comments where behaviour isn't obvious
- Use clear and relevant variable names

## PR Checklist
Before submitting a PR, run through the following checklist,
- [ ] The code follows the repository style
- [ ] The code has been tested locally
- [ ] Documentation has been added wherever necessary
- [ ] PR has a clear title and description

## Thank You
Thanks again for helping improve `pongDaddy`. If you'd like help getting started, open an issue with the label `good first issue` or message the maintainers, we're happy to guide new contributors.