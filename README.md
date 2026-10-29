# Horeca POS - Windows shell (Tauri)

Wraps the web app (`../horeca-pos-web`, built to `../horeca-pos-web/out`) in a Windows program that works offline
and updates itself.

Folder layout expected:
```
C:\dev\horeca-pos-web        (the Next.js app)
C:\dev\horeca-pos-windows    (this project)
```

## One-time setup: signing key for updates
1. `npx @tauri-apps/cli@latest signer generate -w C:\dev\keys\horeca-updater.key`
   Choose a password. This creates the private key and `horeca-updater.key.pub`.
   Keep the private key and its password safe and NEVER commit them. Lose them and installed apps cannot be updated.
2. `node scripts/setup-updater-key.mjs C:\dev\keys\horeca-updater.key.pub`
3. Install Rust 1.90 or newer and the Visual Studio C++ Build Tools.

## Releasing a version (GitHub builds it, nothing runs on your computer)
Releases are built by GitHub from the two repositories, signed, and published as a GitHub Release.
Installed apps find the new version through the release's `latest.json` and update themselves.

One-time setup, in this repository: Settings > Secrets and variables > Actions > New repository secret:
| Secret | Value |
|---|---|
| `TAURI_SIGNING_PRIVATE_KEY` | the full text of `C:\dev\keys\horeca-updater.key` |
| `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` | the password you chose for that key |
| `NEXT_PUBLIC_SUPABASE_URL` | your Supabase project URL |
| `NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY` | your Supabase publishable key (never the secret key) |
| `WEB_REPO_TOKEN` | only while `horeca-pos-web` is private: a fine-grained token with read access to its contents |

Also commit `tauri.conf.json` with your public updater key in it (step 2 above).

To release:
- Push a tag: `git tag v1.0.2 && git push origin v1.0.2`, or
- GitHub website: Actions > "Release desktop app" > Run workflow > enter the version (and, if you like, a web branch and notes).

The workflow builds the web app from `horeca-pos-web` (the same code Vercel hosts), builds and signs the installer,
and publishes `Horeca-Store-POS_<version>_x64-setup.exe` and `latest.json` to the release.
The version comes from the tag. This repository's releases must be publicly downloadable, because installed apps
read `https://github.com/osamamalikawan/horeca-pos-windows/releases/latest/download/latest.json`.

`release.ps1` is still there as a manual fallback.

## Printing
In the desktop app, printing goes straight to the printer chosen in Settings (no dialog). Receipts use the receipt
printer, labels the sticker printer, statements the Windows default printer. If the direct print fails, the normal
print dialog opens.

The app identifier (`com.osamamalik.horecastorepos`) decides where local data is stored. Never change it.
