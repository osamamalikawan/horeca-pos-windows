# Horeca POS - Windows shell (Tauri)

A Windows program that wraps the Horeca POS web app. It works offline, prints straight to your printers,
and updates itself.

Where things come from (nothing depends on a folder on your computer):
- The web app is fetched **from git** (the `horeca-pos-web` repository) every time an installer is built.
  It is the same code Vercel hosts for the browser version.
- The installer is published as a **GitHub Release** of this repository. Installed apps read the release's
  `latest.json` and update themselves.

## One-time setup
1. Make the update signing key:
   `npx @tauri-apps/cli@latest signer generate -w C:\dev\keys\horeca-updater.key`
   Choose a password. Keep the key file and password safe and NEVER commit them. If they are lost, installed apps
   cannot be updated.
2. Put the public key into `tauri.conf.json`: `node scripts/setup-updater-key.mjs C:\dev\keys\horeca-updater.key.pub`
   and commit `tauri.conf.json`.
3. In this repository on GitHub: Settings > Secrets and variables > Actions > New repository secret:

| Secret | Value |
|---|---|
| `TAURI_SIGNING_PRIVATE_KEY` | the full text of `horeca-updater.key` |
| `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` | the key's password |
| `NEXT_PUBLIC_SUPABASE_URL` | your Supabase project URL |
| `NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY` | your Supabase publishable key (never the secret key) |
| `WEB_REPO_TOKEN` | only while `horeca-pos-web` is private: a fine-grained token with read access to its contents |

## Releasing a version (GitHub builds it)
Push the web app changes to `horeca-pos-web` first: the installer contains whatever is on the branch you build.
Then either
- push a tag: `git tag v1.0.2 && git push origin v1.0.2`, or
- GitHub website: Actions > "Release desktop app" > Run workflow > enter the version (optionally a web branch/tag/commit and notes).

The workflow fetches and builds the web app from git, builds and signs the installer, and publishes
`Horeca-Store-POS_<version>_x64-setup.exe` and `latest.json`. The version comes from the tag.
This repository's releases must be publicly downloadable, because installed apps read
`https://github.com/osamamalikawan/horeca-pos-windows/releases/latest/download/latest.json`.

## Building on your own computer (only if you ever need to)
Needs Node, Rust 1.90 or newer and the Visual Studio C++ Build Tools.
Put your two Supabase values in a file named `.env.web` in this folder (it is git-ignored):
```
NEXT_PUBLIC_SUPABASE_URL=https://YOUR-PROJECT.supabase.co
NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY=sb_publishable_...
```
Then, in PowerShell:
```
$env:TAURI_SIGNING_PRIVATE_KEY = "C:\dev\keys\horeca-updater.key"
$env:TAURI_SIGNING_PRIVATE_KEY_PASSWORD = "your password"
node scripts/set-version.mjs 1.0.2
node scripts/build-installer.mjs "What changed"
```
The installer and `latest.json` appear in `release\`. To build a different web version:
`$env:WEB_REF = "some-branch"` (add `$env:WEB_REPO_TOKEN` while the web repository is private).

## Printing
In the desktop app, printing goes straight to the printer chosen in Settings (no dialog). Receipts use the receipt
printer, labels the sticker printer, statements the Windows default printer. If a direct print fails, the normal
print dialog opens.

The app identifier (`com.osamamalik.horecastorepos`) decides where local data is stored. Never change it.
