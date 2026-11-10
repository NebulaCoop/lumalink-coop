/* The lantern's paper as a light. The program Nebula makes for it (lumalink shader) multiplies its picture by the
   light falling on it; this one doesn't: the picture, warmer and brighter, glows most at the middle of the paper, with
   soft bands across it, the same by day and by night. */
#include "program.hlsli"
#define NEBULA_MOTION_CURRENT_INDEX 1
#define NEBULA_MOTION_PREVIOUS_INDEX 2
struct Varyings {
    float4 pos : SV_Position;
    PER_SAMPLE float4 color : COLOR0;
    PER_SAMPLE float2 uv0 : TEXCOORD0;
    VARYINGS_MOTION
};
NativeOut program_ps(Varyings i) {
    const float4 t = TEX(0).Sample(SAMP(0), i.uv0).rgba;
    // 1 at the middle of the paper, 0 at its top and bottom.
    const float middle = 1.0 - abs(i.uv0.y - 0.5) * 2.0;
    const float bands = 0.85 + 0.15 * cos(i.uv0.y * 25.13274);
    const float3 glow = t.rgb * float3(1.7, 1.2, 0.55) * (0.75 + 0.6 * middle) * bands;
    const float4 colour = float4(saturate(glow), t.a);
    NATIVE_STAGE(0, colour)
    PROGRAM_RETURN(colour);
}
