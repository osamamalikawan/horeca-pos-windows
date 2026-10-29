// Copies the PUBLIC updater key into tauri.conf.json.
// Usage:  node scripts/setup-updater-key.mjs C:\dev\keys\horeca-updater.key.pub
import fs from 'node:fs';

const file = process.argv[2];
if (!file) { console.error('Usage: node scripts/setup-updater-key.mjs <path to the .key.pub file>'); process.exit(1); }
const pub = fs.readFileSync(file, 'utf8').trim();
if (!pub || pub.includes('PRIVATE')) { console.error('That does not look like the public key file (.pub).'); process.exit(1); }
const conf = JSON.parse(fs.readFileSync('tauri.conf.json', 'utf8'));
conf.plugins.updater.pubkey = pub;
fs.writeFileSync('tauri.conf.json', JSON.stringify(conf, null, 2) + '\n');
console.log('Public key saved in tauri.conf.json');
