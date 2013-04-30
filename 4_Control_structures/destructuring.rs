fn angle(vector: (float, float)) -> float {
    let pi = float::consts::pi;
    match vector {
        (0f, y) if y < 0f => 1.5 * pi,
        (0f, y) => 0.5 * pi,
        (x, y) => float::atan(y / x)
    }
}
