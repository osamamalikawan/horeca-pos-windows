// Gets the web app from GIT, builds it, and leaves the finished files in .web-src/out,
// which is what the installer packs (tauri.conf.json: "frontendDist": "./.web-src/out").
// Nothing depends on a folder you keep on your computer.
//
//   WEB_REPO_URL   where the web app lives            (default: the horeca-pos-web repository on GitHub)
//   WEB_REF        branch, tag or commit to build     (default: main)
//   WEB_REPO_TOKEN only while the web repository is private: a GitHub token that can read it
//   NEXT_PUBLIC_SUPABASE_URL, NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY   baked into the web app
//     (or put them as KEY=VALUE lines in a file named .env.web next to this project; it is git-ignored)
import { execFileSync, execSync } from 'node:child_process';
import fs from 'node:fs';

const REPO = process.env.WEB_REPO_URL || 'https://github.com/osamamalikawan/horeca-pos-web.git';
const REF = process.env.WEB_REF || 'main';
const DIR = '.web-src';

if (fs.existsSync('.env.web')) {
  for (const line of fs.readFileSync('.env.web', 'utf8').split(/\r?\n/)) {
    const m = line.match(/^\s*([A-Z0-9_]+)\s*=\s*(.*?)\s*$/);
    if (m && !process.env[m[1]]) process.env[m[1]] = m[2].replace(/^["']|["']$/g, '');
  }
}
for (const key of ['NEXT_PUBLIC_SUPABASE_URL', 'NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY']) {
  if (!process.env[key]) { console.error(`Missing ${key}. Set it, or put it in a .env.web file (see README).`); process.exit(1); }
}

// the token goes in through git's environment, so it never lands in a file or on a command line
const gitEnv = { ...process.env };
if (process.env.WEB_REPO_TOKEN) {
  const basic = Buffer.from(`x-access-token:${process.env.WEB_REPO_TOKEN}`).toString('base64');
  Object.assign(gitEnv, {
    GIT_CONFIG_COUNT: '1', GIT_CONFIG_KEY_0: 'http.https://github.com/.extraheader', GIT_CONFIG_VALUE_0: `AUTHORIZATION: basic ${basic}`,
  });
}
const git = (...args) => execFileSync('git', args, { cwd: DIR, stdio: 'inherit', env: gitEnv });

console.log(`Getting the web app from ${REPO} (${REF})`);
fs.rmSync(DIR, { recursive: true, force: true });
fs.mkdirSync(DIR, { recursive: true });
git('init', '-q');
git('remote', 'add', 'origin', REPO);
git('fetch', '-q', '--depth', '1', 'origin', REF);      // works for a branch, a tag or a commit
git('checkout', '-q', 'FETCH_HEAD');
console.log('Web app commit: ' + execFileSync('git', ['rev-parse', '--short', 'HEAD'], { cwd: DIR, env: gitEnv }).toString().trim());

execSync('npm ci', { cwd: DIR, stdio: 'inherit' });
execSync('npm run build', { cwd: DIR, stdio: 'inherit' });
if (!fs.existsSync(`${DIR}/out/index.html`)) { console.error('The web build did not produce out/index.html'); process.exit(1); }
console.log(`Web app ready in ${DIR}/out`);
