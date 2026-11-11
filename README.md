# Flags to Futures

**Build. Play. Teach. Inspire.**

Hands-on CTFd educator toolkit based on *From Setup to Student Engagement: A Hands-On CTFd Tutorial for Cybersecurity, IT, and Computer Science* (SIGCITE 2026).

**Website:** https://kristinechristensen.github.io/ctf/  
**Style guide:** [style-guide.html](style-guide.html) and [STYLE_GUIDE.md](STYLE_GUIDE.md)  
**CTFd homepage:** [ctfd/](ctfd/)  

> This is a practice collection. Instructor flags and solutions are deliberately visible. Change flags before using for graded assignments. Never commit API tokens.

# Faculty CTFd Challenge Library: 20 Challenges

A GitHub-ready, editable collection of **20 classroom CTFd challenges**: 7 general IT, 7 networking, 6 computer science (3 Python and 3 Rust). Each challenge includes a `challenge.yml` specification, student evidence/starter file under `dist/`, a zero-cost hint, and an instructor solution under `writeup/SOLUTION.md`.

**Status:** YAML and artifact consistency validated locally. The package has **not** been imported into a running CTFd instance in this environment. Verify with your installed ctfcli/CTFd versions on a clean test instance before using it in class.

## Quick overview

| Area | Challenges | Points |
| --- | ---: | ---: |
| General IT | 7 | 335 |
| Networking | 7 | 395 |
| Computer Science / Python | 3 | 210 |
| Computer Science / Rust | 3 | 285 |
| **Total** | **20** | **1225** |

See `CHALLENGE_INDEX.csv` for the full inventory.

## Publish to GitHub

1. Unzip the project.
2. Use the existing repository at `https://github.com/kristinechristensen/ctf`.
3. In a terminal inside this project folder:

```bash
git clone https://github.com/kristinechristensen/ctf.git
cd ctf
# Copy the new site and challenge files into this clone
git add .
git commit -m "Add Flags to Futures website and challenges"
git push origin main
```

Do not commit a `.ctf/config` containing your real CTFd API token; `.ctf/` is excluded by `.gitignore`. **This educational package deliberately includes answer flags and instructor solutions. Make the repository private if students should not see them.** A public GitHub repository is not suitable for keeping assessment answers secret. Consider using a private instructor repository and distributing only the `dist/` artifacts to students through CTFd.

## Import using ctfcli (recommended on a TEST CTFd instance)

Install and initialize `ctfcli` in this project directory:

```bash
pipx install 'ctfcli==0.1.8'
cd ctf
ctf init
```

Supply your CTFd URL, such as `http://localhost:8000`, and an administrator API token. This stores credentials in `.ctf/config` locally. Do not upload it.

Then register each local challenge folder. From the project root:

```bash
for dir in challenges/*; do
  ctf challenge add "$dir"
done
```

After registering the challenge folders, run:

```bash
ctf challenge install
```

If your installed ctfcli expects individual paths, use the single-challenge approach below to confirm the CLI first:

```bash
ctf challenge add challenges/01-it-ticket-triage
ctf challenge install challenges/01-it-ticket-triage
```

**Important:** `ctf challenge install` creates new challenges. If the destination already contains a challenge with the same name, prefer testing against a new CTFd instance and review sync behavior before updating anything. No challenge in this collection requires a live vulnerable server.

### Alternative: manual upload

For any one challenge, use `challenge.yml` to copy the description, flags, and hint into the CTFd admin interface; upload the referenced file from `dist/`; paste the instructor `writeup/SOLUTION.md` into the Solution tab and select **Visible after Solve**. This works without ctfcli.

## Editing and reuse

- Change the scenario, points, flags, and file references in each `challenge.yml`.
- Edit or replace evidence under `dist/` and update the corresponding writeup.
- Use `ctf challenge sync` only after verifying the target is the correct existing challenge.
- Keep an instructor master in GitHub and import selected activities into a new CTFd instance each semester.
- The names and flags in this repository are intentionally predictable for training and demonstrations. Replace flags before graded student use, particularly if the repository is public.
- Rust examples can be compiled using `rustc filename.rs` and run as local command-line programs; Python examples use Python 3.

## Challenge structure

```text
challenges/18-cs-rust-ownership/
  challenge.yml
  README.md
  dist/ownership.rs
  writeup/SOLUTION.md
```

## Compatibility and safety

- Challenge YAML follows the official ctfcli challenge specification v0.1, including `solution.path` and `state: solved` as supported by newer ctfcli releases.
- This set uses **standard static-flag challenges** and local evidence files only. It does not run remote container services or execute untrusted student code on the CTFd server.
- The log entries, tickets, and IPs are educational examples, not evidence from a real person or institution.
- Test challenge availability, file downloads, flags, and solution visibility using a separate participant account before deploying to a class.
- In an isolated NETLAB+ environment, ensure GitHub access only during import, or import from the local extracted project.

Official references: https://github.com/CTFd/ctfcli and https://docs.ctfd.io/docs/management/ctfcli/challenges/


## Flags to Futures CTF workshop website

This repository also includes a static GitHub Pages website at `index.html`, with copy-and-paste VM commands, the 20-challenge browser, and instructor-friendly downloads. See `GITHUB_PAGES_GUIDE.md` for publication steps. Instructor solutions are intentionally included for this training collection.
