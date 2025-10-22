const { findAndDeleteNodeModules } = require('./index.js');
const fs = require('fs');
const path = require('path');
const I18n = require('./lib/i18n');

const testDir = path.join(__dirname, 'test-temp');
let testsPassed = 0;
let testsFailed = 0;

function runTest(name, fn) {
  try {
    console.log(`\n🧪 ${name}...`);
    fn();
    testsPassed++;
    console.log(`✅ PASSED`);
  } catch (err) {
    testsFailed++;
    console.log(`❌ FAILED: ${err.message}`);
  }
}

function assert(condition, message) {
  if (!condition) {
    throw new Error(message || 'Assertion failed');
  }
}

function setupTestStructure() {
  if (fs.existsSync(testDir)) {
    fs.rmSync(testDir, { recursive: true, force: true });
  }

  fs.mkdirSync(testDir);
  fs.mkdirSync(path.join(testDir, 'project1'));
  fs.mkdirSync(path.join(testDir, 'project1', 'node_modules'));
  fs.mkdirSync(path.join(testDir, 'project1', 'node_modules', 'package1'));
  fs.writeFileSync(path.join(testDir, 'project1', 'node_modules', 'package1', 'index.js'), '// test file');

  fs.mkdirSync(path.join(testDir, 'project2'));
  fs.mkdirSync(path.join(testDir, 'project2', 'node_modules'));
  fs.mkdirSync(path.join(testDir, 'project2', 'node_modules', 'package2'));

  fs.mkdirSync(path.join(testDir, 'nested'));
  fs.mkdirSync(path.join(testDir, 'nested', 'sub'));
  fs.mkdirSync(path.join(testDir, 'nested', 'sub', 'node_modules'));
  
  fs.mkdirSync(path.join(testDir, 'ignored'));
  fs.mkdirSync(path.join(testDir, 'ignored', 'node_modules'));
}

function cleanup() {
  if (fs.existsSync(testDir)) {
    fs.rmSync(testDir, { recursive: true, force: true });
  }
}

console.log('═══════════════════════════════════════════════════');
console.log('         🧪 DNM Automated Test Suite 🧪           ');
console.log('═══════════════════════════════════════════════════');

runTest('Test 1: Basic deletion (no safe mode)', async () => {
  setupTestStructure();
  
  const result = await findAndDeleteNodeModules(testDir, { 
    safeMode: false,
    silent: true,
    showProgress: false 
  });
  
  assert(result.total === 4, `Expected 4 deletions, got ${result.total}`);
  assert(!fs.existsSync(path.join(testDir, 'project1', 'node_modules')), 'project1/node_modules still exists');
  assert(!fs.existsSync(path.join(testDir, 'project2', 'node_modules')), 'project2/node_modules still exists');
  assert(!fs.existsSync(path.join(testDir, 'nested', 'sub', 'node_modules')), 'nested/sub/node_modules still exists');
  
  cleanup();
});

runTest('Test 2: Ignore functionality', async () => {
  setupTestStructure();
  
  const result = await findAndDeleteNodeModules(testDir, { 
    safeMode: false,
    silent: true,
    showProgress: false,
    ignore: ['ignored']
  });
  
  assert(result.total === 3, `Expected 3 deletions (1 ignored), got ${result.total}`);
  assert(fs.existsSync(path.join(testDir, 'ignored', 'node_modules')), 'ignored/node_modules should still exist');
  assert(!fs.existsSync(path.join(testDir, 'project1', 'node_modules')), 'project1/node_modules should be deleted');
  
  cleanup();
});

runTest('Test 3: I18n - Chinese', () => {
  const i18n = new I18n('zh-CN');
  assert(i18n.t('helpTitle').includes('dnm'), 'Chinese title should contain dnm');
  assert(i18n.t('scanning', { path: 'test' }).includes('正在扫描'), 'Should show Chinese text');
});

runTest('Test 4: I18n - English', () => {
  const i18n = new I18n('en-US');
  assert(i18n.t('helpTitle').includes('dnm'), 'English title should contain dnm');
  assert(i18n.t('scanning', { path: 'test' }).includes('Scanning'), 'Should show English text');
});

runTest('Test 5: Empty directory (no node_modules)', async () => {
  const emptyDir = path.join(testDir, 'empty');
  if (fs.existsSync(emptyDir)) {
    fs.rmSync(emptyDir, { recursive: true, force: true });
  }
  fs.mkdirSync(emptyDir, { recursive: true });
  
  const result = await findAndDeleteNodeModules(emptyDir, { 
    safeMode: false,
    silent: true,
    showProgress: false 
  });
  
  assert(result.total === 0, `Expected 0 deletions, got ${result.total}`);
  
  cleanup();
});

runTest('Test 6: Nested node_modules (should not recurse into deleted)', async () => {
  setupTestStructure();
  
  fs.mkdirSync(path.join(testDir, 'deep'));
  fs.mkdirSync(path.join(testDir, 'deep', 'node_modules'));
  fs.mkdirSync(path.join(testDir, 'deep', 'node_modules', 'package'));
  fs.mkdirSync(path.join(testDir, 'deep', 'node_modules', 'package', 'node_modules'));
  
  const result = await findAndDeleteNodeModules(testDir, { 
    safeMode: false,
    silent: true,
    showProgress: false 
  });
  
  assert(result.total === 5, `Expected 5 deletions, got ${result.total}`);
  assert(!fs.existsSync(path.join(testDir, 'deep', 'node_modules')), 'deep/node_modules should be deleted');
  
  cleanup();
});

runTest('Test 7: Multiple ignore patterns', async () => {
  setupTestStructure();
  
  fs.mkdirSync(path.join(testDir, 'backup'));
  fs.mkdirSync(path.join(testDir, 'backup', 'node_modules'));
  
  const result = await findAndDeleteNodeModules(testDir, { 
    safeMode: false,
    silent: true,
    showProgress: false,
    ignore: ['ignored', 'backup']
  });
  
  assert(result.total === 3, `Expected 3 deletions (2 ignored), got ${result.total}`);
  assert(fs.existsSync(path.join(testDir, 'ignored', 'node_modules')), 'ignored/node_modules should exist');
  assert(fs.existsSync(path.join(testDir, 'backup', 'node_modules')), 'backup/node_modules should exist');
  
  cleanup();
});

console.log('\n═══════════════════════════════════════════════════');
console.log('                 Test Results                      ');
console.log('═══════════════════════════════════════════════════');
console.log(`✅ Passed: ${testsPassed}`);
console.log(`❌ Failed: ${testsFailed}`);
console.log(`📊 Total:  ${testsPassed + testsFailed}`);
console.log('═══════════════════════════════════════════════════\n');

if (testsFailed > 0) {
  process.exit(1);
} else {
  console.log('🎉 All tests passed!\n');
  process.exit(0);
}
