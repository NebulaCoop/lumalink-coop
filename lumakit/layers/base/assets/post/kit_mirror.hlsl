// kit.camera mirror (draw.post WORLD): the 3-D picture flipped left to right.
float4 post(PostIn i) {
    return scene(float2(1.0 - i.uv.x, i.uv.y));
}
