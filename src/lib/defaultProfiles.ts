import {
    DEFAULT_FRONT_NSO_GC_PS4, DEFAULT_FRONT_PS4,
    DEFAULT_FRONT_SIDEWAYS_LEFT_JOY_CON_PS4, DEFAULT_FRONT_SIDEWAYS_RIGHT_JOY_CON_PS4,
    DEFAULT_NSO_GC_XBOX, DEFAULT_SIDEWAYS_LEFT_JOY_CON_XBOX, DEFAULT_SIDEWAYS_RIGHT_JOY_CON_XBOX,
    DEFAULT_UPRIGHT_NSO_GC_PS4, DEFAULT_UPRIGHT_PS4,
    DEFAULT_UPRIGHT_SIDEWAYS_LEFT_JOY_CON_PS4, DEFAULT_UPRIGHT_SIDEWAYS_RIGHT_JOY_CON_PS4,
    DEFAULT_XBOX, ProfileKind, type Profile,
} from "./types";

export type PhysicalSetup = "standard" | "nso_gc" | "left_joycon" | "right_joycon";
export type MotionOrientation = "upright" | "front";

export function createDefaultProfile(kind: ProfileKind, setup: PhysicalSetup, orientation: MotionOrientation): Profile {
    const motionProfile = {
        standard: orientation === "upright" ? DEFAULT_UPRIGHT_PS4 : DEFAULT_FRONT_PS4,
        nso_gc: orientation === "upright" ? DEFAULT_UPRIGHT_NSO_GC_PS4 : DEFAULT_FRONT_NSO_GC_PS4,
        left_joycon: orientation === "upright" ? DEFAULT_UPRIGHT_SIDEWAYS_LEFT_JOY_CON_PS4 : DEFAULT_FRONT_SIDEWAYS_LEFT_JOY_CON_PS4,
        right_joycon: orientation === "upright" ? DEFAULT_UPRIGHT_SIDEWAYS_RIGHT_JOY_CON_PS4 : DEFAULT_FRONT_SIDEWAYS_RIGHT_JOY_CON_PS4,
    }[setup];
    if (kind === ProfileKind.Ps4) return structuredClone(motionProfile);

    const xboxProfile = {
        standard: DEFAULT_XBOX, nso_gc: DEFAULT_NSO_GC_XBOX,
        left_joycon: DEFAULT_SIDEWAYS_LEFT_JOY_CON_XBOX, right_joycon: DEFAULT_SIDEWAYS_RIGHT_JOY_CON_XBOX,
    }[setup];
    const motionOutputs = Object.fromEntries(Object.entries(motionProfile.outputs)
        .filter(([output]) => output.startsWith("Accel") || output.startsWith("Gyro")));
    return structuredClone({...xboxProfile, outputs: {...xboxProfile.outputs, ...motionOutputs}});
}
