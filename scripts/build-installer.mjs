// One command for a full build: web app from git -> signed Windows installer -> files to publish.
// Used by the GitHub release workflow, and by you if you ever need to build on your own computer.
//   node scripts/build-installer.mjs "What changed in this version"
// Needs TAURI_SIGNING_PRIVATE_KEY (the key text, or the path to the key file) and TAURI_SIGNING_PRIVATE_KEY_PASSWORD.
import { execFileSync, execSync } from 'node:child_process';

if (!process.env.TAURI_SIGNING_PRIVATE_KEY) {
  console.error('TAURI_SIGNING_PRIVATE_KEY is not set (the updater key, or the path to its file).');
  process.exit(1);
}
process.env.TAURI_SIGNING_PRIVATE_KEY_PASSWORD ??= '';
const notes = process.argv[2] || '';

execFileSync('node', ['scripts/fetch-web.mjs'], { stdio: 'inherit' });
execSync('npx --yes @tauri-apps/cli@latest build', { stdio: 'inherit' });
execFileSync('node', ['scripts/make-latest-json.mjs', notes], { stdio: 'inherit' });
