// After `tauri build`, prepares the files to upload for an update:
//   release/Horeca-Store-POS_<version>_x64-setup.exe   (installer, space-free name)
//   release/latest.json                                  (what installed apps check)
// Usage: node scripts/make-latest-json.mjs "What changed in this version"
import fs from 'node:fs';
import path from 'node:path';

const REPO = process.env.RELEASE_REPO || 'osamamalikawan/horeca-pos-windows';
const conf = JSON.parse(fs.readFileSync('tauri.conf.json', 'utf8'));
const version = conf.version;
const dir = path.join('target', 'release', 'bundle', 'nsis');
if (!fs.existsSync(dir)) { console.error(`Not found: ${dir}. Run the build first.`); process.exit(1); }

const exe = fs.readdirSync(dir).find((f) => f.endsWith(`_${version}_x64-setup.exe`));
if (!exe) { console.error(`No installer for version ${version} in ${dir}. Did you raise "version" and build?`); process.exit(1); }
const sigFile = path.join(dir, exe + '.sig');
if (!fs.existsSync(sigFile)) { console.error(`No ${exe}.sig. Build with TAURI_SIGNING_PRIVATE_KEY set.`); process.exit(1); }

const out = 'release';
fs.rmSync(out, { recursive: true, force: true });
fs.mkdirSync(out);
const assetName = `Horeca-Store-POS_${version}_x64-setup.exe`;   // no spaces, so the download address matches exactly
fs.copyFileSync(path.join(dir, exe), path.join(out, assetName));
const latest = {
  version,
  notes: process.argv[2] || `Version ${version}`,
  pub_date: new Date().toISOString(),
  platforms: {
    'windows-x86_64': {
      signature: fs.readFileSync(sigFile, 'utf8').trim(),
      url: `https://github.com/${REPO}/releases/download/v${version}/${assetName}`,
    },
  },
};
fs.writeFileSync(path.join(out, 'latest.json'), JSON.stringify(latest, null, 2));
console.log(`Ready in .\\${out}\\ :\n  ${assetName}\n  latest.json\nThen:\n  gh release create v${version} release\\* --title "v${version}" --notes "${latest.notes.replace(/"/g, "'")}"`);
