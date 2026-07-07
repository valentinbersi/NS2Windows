import {execSync} from 'child_process';

const version = process.argv[2];

if (!version) {
    console.error("❌ Usage: npm run release -- <version-number>");
    process.exit(1);
}

console.log(`\n📦 Bumping version to ${version} and building...\n`);

try {
    // 1. Sync the versions across Tauri and Rust
    execSync(`npx tauri-version-sync ${version}`, {stdio: 'inherit'});

    // 2. Update package-lock.json
    execSync('npm install', {stdio: 'inherit'});

    // 3. Build the app (which automatically updates Cargo.lock)
    execSync('npx tauri build', {stdio: 'inherit'});

} catch (error) {
    console.error("❌ Build process failed.");
    process.exit(1);
}
