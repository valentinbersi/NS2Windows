export enum ProfileKind {
    Ps4 = "Ps4",
    Xbox360 = "Xbox360",
}

export enum ControllerKind {
    LeftJoyCon = "LeftJoyCon",
    RightJoyCon = "RightJoyCon",
    ProController = "ProController",
    NsoGcController = "NsoGcController",
}

export interface Connection {
    id: string;
    controller_kind: ControllerKind;
}

export interface SingleController {
    id: string;
}

export interface DualJoyCon {
    left_id: string;
    right_id: string;
    motion_source: "Left" | "Right";
}

export type NsConnectedController =
    | { SingleController: SingleController }
    | { DualJoyCon: DualJoyCon };

export interface EmulatedController {
    profile_name: string;
    connected_controller: NsConnectedController;
}

export type NsInput =
    | "B" | "A" | "Y" | "X" | "Home" | "Capture" | "Chat"
    | "R" | "Zr" | "Tr" | "Sr" | "Gr" | "RTrigger"
    | "L" | "Zl" | "Tl" | "Sl" | "Gl" | "LTrigger"
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

export type Input =
    | { Value: { input: NsInput } }
    | { Grouping: { input: Input } }
    | { Binary: { left: Input, right: Input, operator: "And" | "Or" } };

export interface Profile {
    name: string;
    kind: ProfileKind;
    outputs: Partial<Record<Output, Input>>;
}

export const DEFAULT_XBOX: Profile = {
    name: "Default Xbox",
    kind: ProfileKind.Xbox360,
    outputs: {
        CrossA: {Value: {input: "B"}},
        CircleB: {Value: {input: "A"}},
        SquareX: {Value: {input: "Y"}},
        TriangleY: {Value: {input: "X"}},

        PsGuide: {Value: {input: "Home"}},

        R1Rb: {Value: {input: "R"}},
        R2Rt: {Value: {input: "Zr"}},
        R3Rs: {Value: {input: "Tr"}},

        L1Lb: {Value: {input: "L"}},
        L2Lt: {Value: {input: "Zl"}},
        L3Ls: {Value: {input: "Tl"}},

        OptionsStart: {Value: {input: "Plus"}},
        TouchpadBack: {Value: {input: "Minus"}},

        Down: {Value: {input: "Down"}},
        Left: {Value: {input: "Left"}},
        Right: {Value: {input: "Right"}},
        Up: {Value: {input: "Up"}},

        LeftXMinus: {Value: {input: "LeftXMinus"}},
        LeftXPlus: {Value: {input: "LeftXPlus"}},
        LeftYMinus: {Value: {input: "LeftYMinus"}},
        LeftYPlus: {Value: {input: "LeftYPlus"}},

        RightXMinus: {Value: {input: "RightXMinus"}},
        RightXPlus: {Value: {input: "RightXPlus"}},
        RightYMinus: {Value: {input: "RightYMinus"}},
        RightYPlus: {Value: {input: "RightYPlus"}},
    }
}

export const DEFAULT_PS4: Profile = {
    name: "Default Ps4",
    kind: ProfileKind.Ps4,
    outputs: {
        CrossA: {Value: {input: "B"}},
        CircleB: {Value: {input: "A"}},
        SquareX: {Value: {input: "Y"}},
        TriangleY: {Value: {input: "X"}},

        PsGuide: {Value: {input: "Home"}},
        Share: {Value: {input: "Capture"}},

        R1Rb: {Value: {input: "R"}},
        R2Rt: {Value: {input: "Zr"}},
        R3Rs: {Value: {input: "Tr"}},

        L1Lb: {Value: {input: "L"}},
        L2Lt: {Value: {input: "Zl"}},
        L3Ls: {Value: {input: "Tl"}},

        OptionsStart: {Value: {input: "Plus"}},
        TouchpadBack: {Value: {input: "Minus"}},

        Down: {Value: {input: "Down"}},
        Left: {Value: {input: "Left"}},
        Right: {Value: {input: "Right"}},
        Up: {Value: {input: "Up"}},

        LeftXMinus: {Value: {input: "LeftXMinus"}},
        LeftXPlus: {Value: {input: "LeftXPlus"}},
        LeftYMinus: {Value: {input: "LeftYMinus"}},
        LeftYPlus: {Value: {input: "LeftYPlus"}},

        RightXMinus: {Value: {input: "RightXMinus"}},
        RightXPlus: {Value: {input: "RightXPlus"}},
        RightYMinus: {Value: {input: "RightYMinus"}},
        RightYPlus: {Value: {input: "RightYPlus"}},

        AccelUp: {Value: {input: "AccelUp"}},
        AccelDown: {Value: {input: "AccelDown"}},
        AccelLeft: {Value: {input: "AccelLeft"}},
        AccelRight: {Value: {input: "AccelRight"}},
        AccelForward: {Value: {input: "AccelForward"}},
        AccelBackward: {Value: {input: "AccelBackward"}},

        GyroPitchUp: {Value: {input: "GyroPitchUp"}},
        GyroPitchDown: {Value: {input: "GyroPitchDown"}},
        GyroRollLeft: {Value: {input: "GyroRollLeft"}},
        GyroRollRight: {Value: {input: "GyroRollRight"}},
        GyroYawLeft: {Value: {input: "GyroYawLeft"}},
        GyroYawRight: {Value: {input: "GyroYawRight"}},
    }
}

export const DEFAULT_NSO_GC_XBOX: Profile = {
    name: "Default NSO GC XBOX",
    kind: ProfileKind.Xbox360,
    outputs: {
        CrossA: {Value: {input: "B"}},
        CircleB: {Value: {input: "A"}},
        SquareX: {Value: {input: "Y"}},
        TriangleY: {Value: {input: "X"}},

        PsGuide: {Value: {input: "Home"}},

        R1Rb: {Value: {input: "Zr"}},
        R2Rt: {Binary: {left: {Value: {input: "RTrigger"}}, right: {Value: {input: "R"}}, operator: "Or"}},

        L1Lb: {Value: {input: "Zl"}},
        L2Lt: {Binary: {left: {Value: {input: "LTrigger"}}, right: {Value: {input: "L"}}, operator: "Or"}},

        OptionsStart: {Value: {input: "Plus"}},

        Down: {Value: {input: "Down"}},
        Left: {Value: {input: "Left"}},
        Right: {Value: {input: "Right"}},
        Up: {Value: {input: "Up"}},

        LeftXMinus: {Value: {input: "LeftXMinus"}},
        LeftXPlus: {Value: {input: "LeftXPlus"}},
        LeftYMinus: {Value: {input: "LeftYMinus"}},
        LeftYPlus: {Value: {input: "LeftYPlus"}},

        RightXMinus: {Value: {input: "RightXMinus"}},
        RightXPlus: {Value: {input: "RightXPlus"}},
        RightYMinus: {Value: {input: "RightYMinus"}},
        RightYPlus: {Value: {input: "RightYPlus"}},
    }
}

export const DEFAULT_NSO_GC_PS4: Profile = {
    name: "Default NSO GC Ps4",
    kind: ProfileKind.Ps4,
    outputs: {
        CrossA: {Value: {input: "B"}},
        CircleB: {Value: {input: "A"}},
        SquareX: {Value: {input: "Y"}},
        TriangleY: {Value: {input: "X"}},

        PsGuide: {Value: {input: "Home"}},
        Share: {Value: {input: "Capture"}},

        R1Rb: {Value: {input: "Zr"}},
        R2Rt: {Binary: {left: {Value: {input: "RTrigger"}}, right: {Value: {input: "R"}}, operator: "Or"}},

        L1Lb: {Value: {input: "Zl"}},
        L2Lt: {Binary: {left: {Value: {input: "LTrigger"}}, right: {Value: {input: "L"}}, operator: "Or"}},

        OptionsStart: {Value: {input: "Plus"}},

        Down: {Value: {input: "Down"}},
        Left: {Value: {input: "Left"}},
        Right: {Value: {input: "Right"}},
        Up: {Value: {input: "Up"}},

        LeftXMinus: {Value: {input: "LeftXMinus"}},
        LeftXPlus: {Value: {input: "LeftXPlus"}},
        LeftYMinus: {Value: {input: "LeftYMinus"}},
        LeftYPlus: {Value: {input: "LeftYPlus"}},

        RightXMinus: {Value: {input: "RightXMinus"}},
        RightXPlus: {Value: {input: "RightXPlus"}},
        RightYMinus: {Value: {input: "RightYMinus"}},
        RightYPlus: {Value: {input: "RightYPlus"}},

        AccelUp: {Value: {input: "AccelUp"}},
        AccelDown: {Value: {input: "AccelDown"}},
        AccelLeft: {Value: {input: "AccelLeft"}},
        AccelRight: {Value: {input: "AccelRight"}},
        AccelForward: {Value: {input: "AccelForward"}},
        AccelBackward: {Value: {input: "AccelBackward"}},

        GyroPitchUp: {Value: {input: "GyroPitchUp"}},
        GyroPitchDown: {Value: {input: "GyroPitchDown"}},
        GyroRollLeft: {Value: {input: "GyroRollLeft"}},
        GyroRollRight: {Value: {input: "GyroRollRight"}},
        GyroYawLeft: {Value: {input: "GyroYawLeft"}},
        GyroYawRight: {Value: {input: "GyroYawRight"}},
    }
}

export const DEFAULT_FRONT_JOY_CON_PS4: Profile = {
    name: "Default Front Joy Con Ps4",
    kind: ProfileKind.Ps4,
    outputs: {
        CrossA: {Value: {input: "B"}},
        CircleB: {Value: {input: "A"}},
        SquareX: {Value: {input: "Y"}},
        TriangleY: {Value: {input: "X"}},

        PsGuide: {Value: {input: "Home"}},
        Share: {Value: {input: "Capture"}},

        R1Rb: {Value: {input: "R"}},
        R2Rt: {Value: {input: "Zr"}},
        R3Rs: {Value: {input: "Tr"}},

        L1Lb: {Value: {input: "L"}},
        L2Lt: {Value: {input: "Zl"}},
        L3Ls: {Value: {input: "Tl"}},

        OptionsStart: {Value: {input: "Plus"}},
        TouchpadBack: {Value: {input: "Minus"}},

        Down: {Value: {input: "Down"}},
        Left: {Value: {input: "Left"}},
        Right: {Value: {input: "Right"}},
        Up: {Value: {input: "Up"}},

        LeftXMinus: {Value: {input: "LeftXMinus"}},
        LeftXPlus: {Value: {input: "LeftXPlus"}},
        LeftYMinus: {Value: {input: "LeftYMinus"}},
        LeftYPlus: {Value: {input: "LeftYPlus"}},

        RightXMinus: {Value: {input: "RightXMinus"}},
        RightXPlus: {Value: {input: "RightXPlus"}},
        RightYMinus: {Value: {input: "RightYMinus"}},
        RightYPlus: {Value: {input: "RightYPlus"}},

        AccelUp: {Value: {input: "AccelBackward"}},
        AccelDown: {Value: {input: "AccelForward"}},
        AccelLeft: {Value: {input: "AccelLeft"}},
        AccelRight: {Value: {input: "AccelRight"}},
        AccelForward: {Value: {input: "AccelUp"}},
        AccelBackward: {Value: {input: "AccelDown"}},

        GyroPitchUp: {Value: {input: "GyroPitchUp"}},
        GyroPitchDown: {Value: {input: "GyroPitchDown"}},
        GyroRollLeft: {Value: {input: "GyroYawLeft"}},
        GyroRollRight: {Value: {input: "GyroYawRight"}},
        GyroYawLeft: {Value: {input: "GyroRollLeft"}},
        GyroYawRight: {Value: {input: "GyroRollRight"}},
    }
}

export const DEFAULT_LEFT_JOY_CON_SIDEWAYS_XBOX: Profile = {
    name: "Default Left Joy Con Sideways Xbox",
    kind: ProfileKind.Xbox360,
    outputs: {
        CrossA: {Value: {input: "Left"}},
        CircleB: {Value: {input: "Down"}},
        SquareX: {Value: {input: "Up"}},
        TriangleY: {Value: {input: "Right"}},

        R1Rb: {Value: {input: "Sr"}},

        L1Lb: {Value: {input: "Sl"}},
        L3Ls: {Value: {input: "Tl"}},

        OptionsStart: {Value: {input: "Minus"}},

        LeftXMinus: {Value: {input: "LeftYPlus"}},
        LeftXPlus: {Value: {input: "LeftYMinus"}},
        LeftYMinus: {Value: {input: "LeftXMinus"}},
        LeftYPlus: {Value: {input: "LeftXPlus"}},
    }
}

export const DEFAULT_LEFT_JOY_CON_SIDEWAYS_PS4: Profile = {
    name: "Default Left Joy Con Sideways Ps4",
    kind: ProfileKind.Ps4,
    outputs: {
        CrossA: {Value: {input: "Left"}},
        CircleB: {Value: {input: "Down"}},
        SquareX: {Value: {input: "Up"}},
        TriangleY: {Value: {input: "Right"}},

        Share: {Value: {input: "Capture"}},

        R1Rb: {Value: {input: "Sr"}},

        L1Lb: {Value: {input: "Sl"}},
        L3Ls: {Value: {input: "Tl"}},

        OptionsStart: {Value: {input: "Minus"}},

        LeftXMinus: {Value: {input: "LeftYPlus"}},
        LeftXPlus: {Value: {input: "LeftYMinus"}},
        LeftYMinus: {Value: {input: "LeftXMinus"}},
        LeftYPlus: {Value: {input: "LeftXPlus"}},

        AccelUp: {Value: {input: "AccelRight"}},
        AccelDown: {Value: {input: "AccelLeft"}},
        AccelLeft: {Value: {input: "AccelUp"}},
        AccelRight: {Value: {input: "AccelDown"}},
        AccelForward: {Value: {input: "AccelForward"}},
        AccelBackward: {Value: {input: "AccelBackward"}},

        GyroPitchUp: {Value: {input: "GyroYawLeft"}},
        GyroPitchDown: {Value: {input: "GyroYawRight"}},
        GyroRollLeft: {Value: {input: "GyroRollLeft"}},
        GyroRollRight: {Value: {input: "GyroRollRight"}},
        GyroYawLeft: {Value: {input: "GyroPitchDown"}},
        GyroYawRight: {Value: {input: "GyroPitchUp"}},
    }
}

export const DEFAULT_RIGHT_JOY_CON_SIDEWAYS_XBOX: Profile = {
    name: "Default Right Joy Con Sideways Xbox",
    kind: ProfileKind.Xbox360,
    outputs: {
        CrossA: {Value: {input: "A"}},
        CircleB: {Value: {input: "X"}},
        SquareX: {Value: {input: "B"}},
        TriangleY: {Value: {input: "Y"}},

        PsGuide: {Value: {input: "Home"}},

        R1Rb: {Value: {input: "Sr"}},

        L1Lb: {Value: {input: "Sl"}},
        L3Ls: {Value: {input: "Tr"}},

        OptionsStart: {Value: {input: "Plus"}},

        LeftXMinus: {Value: {input: "LeftYMinus"}},
        LeftXPlus: {Value: {input: "LeftYPlus"}},
        LeftYMinus: {Value: {input: "LeftXPlus"}},
        LeftYPlus: {Value: {input: "LeftXMinus"}},
    }
}

export const DEFAULT_RIGHT_JOY_CON_SIDEWAYS_PS4: Profile = {
    name: "Default Right Joy Con Sideways Ps4",
    kind: ProfileKind.Ps4,
    outputs: {
        CrossA: {Value: {input: "A"}},
        CircleB: {Value: {input: "X"}},
        SquareX: {Value: {input: "B"}},
        TriangleY: {Value: {input: "Y"}},

        PsGuide: {Value: {input: "Home"}},

        R1Rb: {Value: {input: "Sr"}},

        L1Lb: {Value: {input: "Sl"}},
        L3Ls: {Value: {input: "Tl"}},

        OptionsStart: {Value: {input: "Plus"}},

        LeftXMinus: {Value: {input: "LeftYMinus"}},
        LeftXPlus: {Value: {input: "LeftYPlus"}},
        LeftYMinus: {Value: {input: "LeftXPlus"}},
        LeftYPlus: {Value: {input: "LeftXMinus"}},

        AccelUp: {Value: {input: "AccelLeft"}},
        AccelDown: {Value: {input: "AccelRight"}},
        AccelLeft: {Value: {input: "AccelDown"}},
        AccelRight: {Value: {input: "AccelUp"}},
        AccelForward: {Value: {input: "AccelForward"}},
        AccelBackward: {Value: {input: "AccelBackward"}},

        GyroPitchUp: {Value: {input: "GyroYawRight"}},
        GyroPitchDown: {Value: {input: "GyroYawLeft"}},
        GyroRollLeft: {Value: {input: "GyroRollLeft"}},
        GyroRollRight: {Value: {input: "GyroRollRight"}},
        GyroYawLeft: {Value: {input: "GyroPitchUp"}},
        GyroYawRight: {Value: {input: "GyroPitchDown"}},
    }
}

export const DEFAULT_LEFT_JOY_CON_FRONT_SIDEWAYS_PS4: Profile = {
    name: "Default Left Joy Con Front Sideways Ps4",
    kind: ProfileKind.Ps4,
    outputs: {
        CrossA: {Value: {input: "Left"}},
        CircleB: {Value: {input: "Down"}},
        SquareX: {Value: {input: "Up"}},
        TriangleY: {Value: {input: "Right"}},

        Share: {Value: {input: "Capture"}},

        R1Rb: {Value: {input: "Sr"}},

        L1Lb: {Value: {input: "Sl"}},
        L3Ls: {Value: {input: "Tl"}},

        OptionsStart: {Value: {input: "Minus"}},

        LeftXMinus: {Value: {input: "LeftYPlus"}},
        LeftXPlus: {Value: {input: "LeftYMinus"}},
        LeftYMinus: {Value: {input: "LeftXMinus"}},
        LeftYPlus: {Value: {input: "LeftXPlus"}},

        AccelUp: {Value: {input: "AccelBackward"}},
        AccelDown: {Value: {input: "AccelForward"}},
        AccelLeft: {Value: {input: "AccelUp"}},
        AccelRight: {Value: {input: "AccelDown"}},
        AccelForward: {Value: {input: "AccelRight"}},
        AccelBackward: {Value: {input: "AccelLeft"}},

        GyroPitchUp: {Value: {input: "GyroYawLeft"}},
        GyroPitchDown: {Value: {input: "GyroYawRight"}},
        GyroRollLeft: {Value: {input: "GyroPitchDown"}},
        GyroRollRight: {Value: {input: "GyroPitchUp"}},
        GyroYawLeft: {Value: {input: "GyroRollRight"}},
        GyroYawRight: {Value: {input: "GyroRollLeft"}},
    }
}

export const DEFAULT_RIGHT_JOY_CON_FRONT_SIDEWAYS_PS4: Profile = {
    name: "Default Right Joy Con Front Sideways Ps4",
    kind: ProfileKind.Ps4,
    outputs: {
        CrossA: {Value: {input: "A"}},
        CircleB: {Value: {input: "X"}},
        SquareX: {Value: {input: "B"}},
        TriangleY: {Value: {input: "Y"}},

        PsGuide: {Value: {input: "Home"}},

        R1Rb: {Value: {input: "Sr"}},

        L1Lb: {Value: {input: "Sl"}},
        L3Ls: {Value: {input: "Tl"}},

        OptionsStart: {Value: {input: "Plus"}},

        LeftXMinus: {Value: {input: "LeftYMinus"}},
        LeftXPlus: {Value: {input: "LeftYPlus"}},
        LeftYMinus: {Value: {input: "LeftXPlus"}},
        LeftYPlus: {Value: {input: "LeftXMinus"}},

        AccelUp: {Value: {input: "AccelBackward"}},
        AccelDown: {Value: {input: "AccelForward"}},
        AccelLeft: {Value: {input: "AccelDown"}},
        AccelRight: {Value: {input: "AccelUp"}},
        AccelForward: {Value: {input: "AccelLeft"}},
        AccelBackward: {Value: {input: "AccelRight"}},

        GyroPitchUp: {Value: {input: "GyroYawRight"}},
        GyroPitchDown: {Value: {input: "GyroYawLeft"}},
        GyroRollLeft: {Value: {input: "GyroPitchUp"}},
        GyroRollRight: {Value: {input: "GyroPitchDown"}},
        GyroYawLeft: {Value: {input: "GyroRollRight"}},
        GyroYawRight: {Value: {input: "GyroRollLeft"}},
    }
}

export const CONTROLLER_KIND_LABELS: Record<ControllerKind, string> = {
    [ControllerKind.LeftJoyCon]: "Left Joy-Con",
    [ControllerKind.RightJoyCon]: "Right Joy-Con",
    [ControllerKind.ProController]: "Pro Controller (Beta)",
    [ControllerKind.NsoGcController]: "NSO GC Controller",
};

export const NS_INPUT_LABELS: Record<NsInput, string> = {
    B: "B", A: "A", Y: "Y", X: "X",
    Home: "Home", Capture: "Capture", Chat: "Chat",
    R: "R", Zr: "ZR", Tr: "TR", Sr: "SR", Gr: "GR", RTrigger: "R Trigger (Analog)",
    L: "L", Zl: "ZL", Tl: "TL", Sl: "SL", Gl: "GL", LTrigger: "L Trigger (Analog)",
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

export interface VirtualControllerState {
    id: string; // Frontend UUID
    profile_name: string | null;
    bound_controllers: Connection[];
    is_running: boolean;
    emulated_controller_id: string | null; // Backend UUID when running
    motion_source: "Left" | "Right"; // For Dual Joy-Con
}
