// Sets the app version in tauri.conf.json (used by the release workflow: the git tag decides the version).
// Usage: node scripts/set-version.mjs 1.0.2
import fs from 'node:fs';

const version = (process.argv[2] || '').replace(/^v/, '');
if (!/^\d+\.\d+\.\d+$/.test(version)) { console.error(`"${process.argv[2] ?? ''}" is not a version like 1.0.2`); process.exit(1); }
const conf = JSON.parse(fs.readFileSync('tauri.conf.json', 'utf8'));
conf.version = version;
fs.writeFileSync('tauri.conf.json', JSON.stringify(conf, null, 2) + '\n');
console.log(`Version set to ${version}`);
