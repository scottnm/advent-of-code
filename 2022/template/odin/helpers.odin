package main

//
// Vec2 helpers
//
vec2 :: struct {
    x, y: int,
}

vec2_add :: proc(lhs: vec2, rhs: vec2) -> vec2 {
    x := lhs.x + rhs.x
    y := lhs.y + rhs.y
    return vec2 {x,y}
}

vec2_sub :: proc(lhs: vec2, rhs: vec2) -> vec2 {
    x := lhs.x - rhs.x
    y := lhs.y - rhs.y
    return vec2 {x,y}
}

vec2_in_bounds :: proc(v: vec2, lower_bound: vec2, upper_bound: vec2) -> bool {
        return (v.x >= lower_bound.x) &&
               (v.x < upper_bound.x) &&
               (v.y >= lower_bound.y) &&
               (v.y < upper_bound.y)
}
