#!/usr/bin/env python3

from z3 import *

px, vx = Ints('px vx')
py, vy = Ints('py vy')
pz, vz = Ints('pz vz')

# Each hailstone gives us 2 linear equations with 6 unknowns. Here is how:
#
# A hailstone and a rock will reach the same position after some time t:
# h_px + t * h_vx = r_px + t * r_vx
# h_py + t * h_vy = r_py + t * r_vy
# h_pz + t * h_vz = r_pz + t * r_vz
#
# If we solve by t, we get three terms that are equal:
# t = (px - px1)/(vx1 - vx)
#   = (py - py1)/(vy1 - vy)
#   = (pz - pz1)/(vz1 - vz)
#
# From these three equalities, we can form two independent linear equations
# (the third one is dependent on the other two):
# (1) (px - px1)(vy1 - vy) = (py - py1)(vx1 - vx)
# (2) (px - px1)(vz1 - vz) = (pz - pz1)(vx1 - vx)
#
# We need three hailstones to get 6 linear equations to solve our 6 unknowns.
# Therefore, we'll use the first three hailstones from the input.
hailstones = [
    [216518090678054, 311610807965630, 244665409335040, -24, -43, 118],
    [119252599207972, 265844340901442, 404506989029618, 93, 9, -69],
    [366376232895280, 243548034524148, 222429607201000, 18, 38, 19],
]

s = Solver()

for [sx, sy, sz, ux, uy, uz] in hailstones:
    s.add((px - sx) * (uy - vy) == (py - sy) * (ux - vx))
    s.add((px - sx) * (uz - vz) == (pz - sz) * (ux - vx))

print(s.assertions())
print(s.check())
print()

m = s.model()
print(f"Position: ({m[px]}, {m[py]}, {m[pz]})")
print(f"Velocity: ({m[vx]}, {m[vy]}, {m[vz]})")
print()

answer = m[px].as_long() + m[py].as_long() + m[pz].as_long()
print(f"Answer: {answer}")
