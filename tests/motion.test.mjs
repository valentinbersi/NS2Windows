// Uses the project's existing TypeScript compiler and Node's built-in test
// runner; no test dependency or manifest change is needed.
// Run: node --test tests/motion.test.mjs
import {test} from "node:test";
import assert from "node:assert/strict";
import {readFile, mkdir, writeFile} from "node:fs/promises";
import ts from "typescript";

const compiled = new URL("../.svelte-kit/motion-tests/", import.meta.url);
await mkdir(compiled, {recursive: true});
for (const name of ["types", "defaultProfiles", "cemuhookSettings"]) {
    const source = await readFile(new URL(`../src/lib/${name}.ts`, import.meta.url), "utf8");
    const {outputText} = ts.transpileModule(source, {compilerOptions: {target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ES2022}});
    await writeFile(new URL(`${name}.mjs`, compiled), outputText.replaceAll('from "./types"', 'from "./types.mjs"'));
}
const {ProfileKind, XBOX360_OUTPUT_LABELS} = await import(new URL("types.mjs", compiled));
const {createDefaultProfile} = await import(new URL("defaultProfiles.mjs", compiled));
const {validateCemuHookSettings} = await import(new URL("cemuhookSettings.mjs", compiled));

test("Xbox defaults retain buttons and copy all motion axes for each setup and orientation", () => {
    for (const setup of ["standard", "nso_gc", "left_joycon", "right_joycon"]) {
        for (const orientation of ["upright", "front"]) {
            const ps4 = createDefaultProfile(ProfileKind.Ps4, setup, orientation);
            const xbox = createDefaultProfile(ProfileKind.Xbox360, setup, orientation);
            const motion = Object.keys(ps4.outputs).filter(key => key.startsWith("Accel") || key.startsWith("Gyro"));
            assert.equal(motion.length, 12);
            assert.equal(xbox.kind, ProfileKind.Xbox360);
            for (const output of motion) {
                assert.deepEqual(xbox.outputs[output], ps4.outputs[output]);
                assert.ok(XBOX360_OUTPUT_LABELS[output]);
            }
            assert.ok(xbox.outputs.CrossA);
        }
    }
});

test("front-facing defaults differ and generated profiles do not mutate templates", () => {
    const upright = createDefaultProfile(ProfileKind.Xbox360, "right_joycon", "upright");
    const front = createDefaultProfile(ProfileKind.Xbox360, "right_joycon", "front");
    assert.notDeepEqual(upright.outputs.GyroRollLeft, front.outputs.GyroRollLeft);
    front.outputs.GyroRollLeft.Value.input = "A";
    assert.notEqual(createDefaultProfile(ProfileKind.Xbox360, "right_joycon", "front").outputs.GyroRollLeft.Value.input, "A");
});

test("Settings accept IP literals and reject malformed IPs and invalid ports", () => {
    for (const ip of ["127.0.0.1", " 192.168.1.50 ", "0.0.0.0", "::1", "::", "2001:db8::1"]) {
        assert.equal(validateCemuHookSettings(ip, 26760), null);
    }
    for (const ip of ["", "localhost", "127.1", "1.2.3.256", "1.2.3.04", "http://127.0.0.1", "[::1]", "::gg", "::1/64"]) {
        assert.ok(validateCemuHookSettings(ip, 26760));
    }
    for (const port of [0, -1, 65536, 1.5, NaN]) assert.ok(validateCemuHookSettings("127.0.0.1", port));
    for (const port of [1, 65535]) assert.equal(validateCemuHookSettings("::1", port), null);
});
