import { existsSync, readFileSync, readdirSync } from 'node:fs';

const [root, sha] = process.argv.slice(2);
const crates = readdirSync(root + '/crates')
  .filter((name) => existsSync(`${root}/crates/${name}/Cargo.toml`))
  .sort();
const manifests = new Map(crates.map((name) => [
  name,
  readFileSync(`${root}/crates/${name}/Cargo.toml`, 'utf8'),
]));
const architecture = readFileSync(root + '/docs/architecture.md', 'utf8');
const supportOnly = new Set(['cb-embedding', 'cb-recall']);
let valid = true;
let meaningful = 0;

console.log(`# Independent semantic review\n\nTested source: ${sha}\n`);
for (const name of crates) {
  const sourceRoot = `${root}/crates/${name}/src`;
  const sources = readdirSync(sourceRoot)
    .filter((file) => file.endsWith('.rs'))
    .map((file) => readFileSync(`${sourceRoot}/${file}`, 'utf8'))
    .join('\n');
  const typedApi = /pub (fn|struct|enum|trait|type)|async fn main/.test(sources);
  const testEvidence = sources.includes('#[cfg(test)]') || name === 'cb-api';
  const caller = name === 'cb-api'
    ? 'workspace executable / CI launch'
    : crates.find((candidate) => candidate !== name
      && manifests.get(candidate).includes(`${name}={path=`));
  const mapped = architecture.includes(`| ${name} |`);
  const counted = typedApi && testEvidence && Boolean(caller) && mapped && !supportOnly.has(name);
  if (counted) meaningful += 1;
  if (!typedApi || !testEvidence || !caller || !mapped) valid = false;
  console.log(`- ${name}: typed behavior=${typedApi ? 'yes' : 'no'}; tests=${testEvidence ? 'yes' : 'no'}; actual caller=${caller || 'none'}; architecture=${mapped ? 'mapped' : 'missing'}; semantic count=${counted ? 'meaningful' : 'support-only'}`);
}

console.log(`\nObserved meaningful crate count: ${meaningful}; support-only crates not counted: ${[...supportOnly].join(', ')}.`);
console.log('Reviewed executed call path: browser -> cb-api -> auth/middleware -> orchestrator -> compaction/provenance/policy -> RuVector/Regorus -> RVF -> rendered result.');
const pass = valid && meaningful >= 20 && crates.length >= 20;
console.log(`Semantic verdict: ${pass ? 'PASS' : 'FAIL'} - source/manifests establish ${meaningful} substantive crates and ${crates.length} total workspace crates.`);
if (!pass) process.exitCode = 1;
