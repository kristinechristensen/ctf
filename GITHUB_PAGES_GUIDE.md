# Publish Zero to Hero CTF on GitHub Pages

This repository is a static website plus 20 challenge folders. No build system, Node.js, or hosting account beyond GitHub Pages is required.

## Upload the repository

1. Extract the distribution ZIP.
2. On GitHub create a new repository, for example `zero-to-hero-ctf`.
3. Upload **all contents of the extracted project folder**, including `index.html`, `site.css`, `site.js`, `assets/`, `challenges/`, and `CHALLENGE_INDEX.csv`. Do not upload the outer ZIP in place of the files.
4. Commit your changes.
5. Under **Settings → Pages**, choose **Deploy from a branch** and select `main` and `/(root)`.
6. GitHub will show the live Pages URL in Settings → Pages when publication completes. It will normally be `https://USERNAME.github.io/zero-to-hero-ctf/`.
7. On the published site, verify that the ZIP link, individual evidence files, command copy buttons, and filters work.

## Important: shared instructor files

This **training repository intentionally contains answers, flags, and instructor solutions**. It should not be treated as a secret or graded challenge bank. GitHub Pages from a public repository is accessible to anyone. For a graded CTF, publish only student evidence and keep answer materials separately, and rotate all flags.

## Copy/paste command placeholders

The Git clone snippet contains `YOUR_USERNAME/YOUR_REPOSITORY`: replace these with the actual GitHub repository owner and name before asking participants to copy them. A participant on the Ubuntu VM can also download the ZIP directly from the website, unzip it, and change into that folder.

## Files

- `index.html`: website
- `site.css`, `site.js`: styles and copy/search interactions
- `assets/hero-mascot.png`, `assets/logo.svg`: illustrated hero and scalable logo
- `assets/Faculty-CTFd-20-Challenges.zip`: direct-download challenge library
- `assets/challenge-data.js`: searchable challenge inventory
- `challenges/`: 20 configurations, evidence, instructor solutions
- `README.md`: collection and ctfcli documentation

## Docker and ctfcli

The terminal instructions are provided for an Ubuntu VM. Verify package availability for the specific release. ctfcli is pinned to version 0.1.8 and should be tested with the CTFd version used in the workshop. The CTFd administrator token belongs on the VM only and must never be committed to the repository.
