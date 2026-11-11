# Publish Flags to Futures CTF on GitHub Pages

This repository is a static website plus 20 challenge folders. No build system, Node.js, or hosting account beyond GitHub Pages is required.

## Upload the repository

1. Extract the distribution ZIP.
2. On GitHub create a new repository, for example `zero-to-hero-ctf`.
3. Upload **all contents of the extracted project folder**, including `index.html`, `site.css`, `site.js`, `assets/`, `challenges/`, and `CHALLENGE_INDEX.csv`. Do not upload the outer ZIP in place of the files.
4. Commit your changes.
5. Under **Settings → Pages**, choose **Deploy from a branch** and select `main` and `/(root)`.
6. GitHub will show the live Pages URL in Settings → Pages when publication completes. It will normally be `https://kristinechristensen.github.io/ctf/`.
7. On the published site, verify that the ZIP link, individual evidence files, command copy buttons, and filters work.

## Important: shared instructor files

This **training repository intentionally contains answers, flags, and instructor solutions**. It should not be treated as a secret or graded challenge bank. GitHub Pages from a public repository is accessible to anyone. For a graded CTF, publish only student evidence and keep answer materials separately, and rotate all flags.

## Copy/paste command placeholders

The Git clone command already points to `https://github.com/kristinechristensen/ctf.git` and creates a local `ctf` folder. A participant on the Ubuntu VM can also download the ZIP directly from the website, unzip it, and change into that folder.

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

## Your existing repository: missing folders to publish

Your GitHub repository already contains root files, but ensure you ALSO upload the **full** `assets/` and `challenges/` folders from this ZIP. Without `assets/`, the artwork, JavaScript inventory, and downloads will be missing. Without `challenges/`, `ctfcli` cannot load the 20 challenges and the evidence/solution links break. Use Git locally to copy the extracted contents into a clone, then commit and push; do not upload only the ZIP file.

```bash
git clone https://github.com/kristinechristensen/ctf.git
cd ctf
# Copy the extracted website contents here, preserving their folders.
git add .
git commit -m "Publish complete Flags to Futures CTF workshop"
git push origin main
```

GitHub Pages URL: https://kristinechristensen.github.io/ctf/ (after enabling Pages in repository settings).


## Updated visual assets
The `assets/`, `site.css`, `style-guide.html`, `STYLE_GUIDE.md` and `ctfd/` files are part of the refreshed Flags to Futures identity. Copy them alongside `index.html` to preserve the look and the CTFd homepage snippet.

## CTFd appearance tutorial
The landing page includes an interactive [Customize CTFd](https://kristinechristensen.github.io/ctf/#customize) section. Buttons copy the exact contents of `ctfd/homepage.html` and `ctfd/theme-header.html` with no manual file opening. These buttons work when deployed via GitHub Pages (HTTPS).
