#!/usr/bin/env python3

import bpy
import os

csv_file: str = "1.csv"
cube_size: float = (8, 8, 12)

lines: list[str] = None

try:
    with open(csv_file) as f:
        lines = f.readlines()
except:
    with open(os.path.join(os.path.dirname(bpy.data.filepath), csv_file)) as f:
        lines = f.readlines()

collection: object = bpy.ops.collection.create()

for i, line in enumerate(lines):
    line = line.strip()
    if (line == ''):
        break
    for j, flag in enumerate(line.strip().split(',')):
        if (flag == '0'):
            continue
        x: float = i * cube_size[0]
        y: float = j * cube_size[1]
        z: float = cube_size[2] / 2
        # We cannot use `scale` paramter (see |https://blender.stackexchange.com/questions/212886/strange-size-when-creating-a-cube-using-bpy-ops-mesh-primitive-cube-add|).
        bpy.ops.mesh.primitive_cube_add(size=1, location=(x, y, z))
        bpy.context.object.scale = list(cube_size)
