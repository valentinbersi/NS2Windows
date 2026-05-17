export enum ProfileKind {
    Ps4 = "Ps4",
    Xbox360 = "Xbox360",
}

export enum ControllerKind {
    LeftJoyCon = "LeftJoyCon",
    RightJoyCon = "RightJoyCon",
    DualJoyCons = "DualJoyCons",
    ProController = "ProController",
    NsoGcController = "NsoGcController",
}

export interface Connection {
    id: string;
    controller_kind: ControllerKind;
}

export type NsInput =
    | "B" | "A" | "Y" | "X" | "Home" | "Capture"
    | "R" | "Zr" | "Tr" | "Sr" | "Gr"
    | "L" | "Zl" | "Tl" | "Sl" | "Gl"
    | "Plus" | "Minus"
    | "Down" | "Left" | "Right" | "Up"
    | "LeftXMinus" | "LeftXPlus" | "LeftYMinus" | "LeftYPlus"
    | "RightXMinus" | "RightXPlus" | "RightYMinus" | "RightYPlus"
    | "AccelUp" | "AccelDown" | "AccelLeft" | "AccelRight" | "AccelForward" | "AccelBackward"
    | "GyroPitchUp" | "GyroPitchDown" | "GyroRollLeft" | "GyroRollRight" | "GyroYawLeft" | "GyroYawRight";

export type Output =
    | "CrossA" | "CircleB" | "SquareX" | "TriangleY"
    | "PsGuide" | "Share"
    | "R1Rb" | "R2Rt" | "R3Rs"
    | "L1Lb" | "L2Lt" | "L3Ls"
    | "OptionsStart" | "TouchpadBack"
    | "Down" | "Left" | "Right" | "Up"
    | "LeftXMinus" | "LeftXPlus" | "LeftYMinus" | "LeftYPlus"
    | "RightXMinus" | "RightXPlus" | "RightYMinus" | "RightYPlus"
    | "AccelUp" | "AccelDown" | "AccelLeft" | "AccelRight" | "AccelForward" | "AccelBackward"
    | "GyroPitchUp" | "GyroPitchDown" | "GyroRollLeft" | "GyroRollRight" | "GyroYawLeft" | "GyroYawRight";

export interface Condition {
    Value: NsInput;
}

export interface Profile {
    profile_name: string;
    profile_kind: ProfileKind;
    outputs: Partial<Record<Output, Condition>>;
}

export const CONTROLLER_KIND_LABELS: Record<ControllerKind, string> = {
    [ControllerKind.LeftJoyCon]: "Left Joy-Con",
    [ControllerKind.RightJoyCon]: "Right Joy-Con",
    [ControllerKind.DualJoyCons]: "Dual Joy-Cons",
    [ControllerKind.ProController]: "Pro Controller",
    [ControllerKind.NsoGcController]: "NSO GC Controller",
};

export const NS_INPUT_LABELS: Record<NsInput, string> = {
    B: "B", A: "A", Y: "Y", X: "X",
    Home: "Home", Capture: "Capture",
    R: "R", Zr: "ZR", Tr: "TR", Sr: "SR", Gr: "GR",
    L: "L", Zl: "ZL", Tl: "TL", Sl: "SL", Gl: "GL",
    Plus: "Plus", Minus: "Minus",
    Down: "D-Pad Down", Left: "D-Pad Left", Right: "D-Pad Right", Up: "D-Pad Up",
    LeftXMinus: "Left Stick Left", LeftXPlus: "Left Stick Right",
    LeftYMinus: "Left Stick Down", LeftYPlus: "Left Stick Up",
    RightXMinus: "Right Stick Left", RightXPlus: "Right Stick Right",
    RightYMinus: "Right Stick Down", RightYPlus: "Right Stick Up",
    AccelUp: "Accel Up", AccelDown: "Accel Down", AccelLeft: "Accel Left", AccelRight: "Accel Right",
    AccelForward: "Accel Forward", AccelBackward: "Accel Backward",
    GyroPitchUp: "Gyro Pitch Up", GyroPitchDown: "Gyro Pitch Down",
    GyroRollLeft: "Gyro Roll Left", GyroRollRight: "Gyro Roll Right",
    GyroYawLeft: "Gyro Yaw Left", GyroYawRight: "Gyro Yaw Right",
};

export const PS4_OUTPUT_LABELS: Record<Output, string | null> = {
    CrossA: "Cross",
    CircleB: "Circle",
    SquareX: "Square",
    TriangleY: "Triangle",
    PsGuide: "PS Button",
    Share: "Share",
    R1Rb: "R1",
    R2Rt: "R2",
    R3Rs: "R3",
    L1Lb: "L1",
    L2Lt: "L2",
    L3Ls: "L3",
    OptionsStart: "Options",
    TouchpadBack: "Touchpad",
    Down: "D-Pad Down",
    Left: "D-Pad Left",
    Right: "D-Pad Right",
    Up: "D-Pad Up",
    LeftXMinus: "Left Stick Left",
    LeftXPlus: "Left Stick Right",
    LeftYMinus: "Left Stick Down",
    LeftYPlus: "Left Stick Up",
    RightXMinus: "Right Stick Left",
    RightXPlus: "Right Stick Right",
    RightYMinus: "Right Stick Down",
    RightYPlus: "Right Stick Up",
    AccelUp: "Accel Up",
    AccelDown: "Accel Down",
    AccelLeft: "Accel Left",
    AccelRight: "Accel Right",
    AccelForward: "Accel Forward",
    AccelBackward: "Accel Backward",
    GyroPitchUp: "Gyro Pitch Up",
    GyroPitchDown: "Gyro Pitch Down",
    GyroRollLeft: "Gyro Roll Left",
    GyroRollRight: "Gyro Roll Right",
    GyroYawLeft: "Gyro Yaw Left",
    GyroYawRight: "Gyro Yaw Right",
};

export const XBOX360_OUTPUT_LABELS: Record<Output, string | null> = {
    CrossA: "A",
    CircleB: "B",
    SquareX: "X",
    TriangleY: "Y",
    PsGuide: "Guide",
    Share: null, // Not used
    R1Rb: "RB",
    R2Rt: "RT",
    R3Rs: "RS",
    L1Lb: "LB",
    L2Lt: "LT",
    L3Ls: "LS",
    OptionsStart: "Start",
    TouchpadBack: "Back",
    Down: "D-Pad Down",
    Left: "D-Pad Left",
    Right: "D-Pad Right",
    Up: "D-Pad Up",
    LeftXMinus: "Left Stick Left",
    LeftXPlus: "Left Stick Right",
    LeftYMinus: "Left Stick Down",
    LeftYPlus: "Left Stick Up",
    RightXMinus: "Right Stick Left",
    RightXPlus: "Right Stick Right",
    RightYMinus: "Right Stick Down",
    RightYPlus: "Right Stick Up",
    // Motion not used for Xbox360
    AccelUp: null,
    AccelDown: null,
    AccelLeft: null,
    AccelRight: null,
    AccelForward: null,
    AccelBackward: null,
    GyroPitchUp: null,
    GyroPitchDown: null,
    GyroRollLeft: null,
    GyroRollRight: null,
    GyroYawLeft: null,
    GyroYawRight: null,
};
