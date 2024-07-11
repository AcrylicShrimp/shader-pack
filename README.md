# shader-pack

Very strong and flexible shader language for game engines.

## Example

```
# From Uniform Bindings
in main_tex: t2; # 2D Texture
in second_tex: t2;

# From Unified Uniform Buffer (they are merged into a single uniform buffer automatically)
in color: f3; # vec3<f32>
in light_intensity: f; # f32
in thinkness: f;

# From read-only Storage Buffer
in bone_pos m4[512];

# From read-only Storage Buffer
# the `const` is expanded by the engine, allowing users not to refer implementation details
# think it as templating, but it envolves type systems so it's very safe
in bone_pos_auto_max m4[const("max-bone-count")];

# vertex input mapping (from "position" attribute of mesh)
@vertex = "position"
in pos: f3;

# vertex input mapping (from "normal" attribute of mesh)
@vertex = "normal"
in normal: f3;

# compile-time loop; it will be unrolled during compilation
compile loop n times 10 {
    in !ident("uv_{}", n): f2;
}

compile loop n times const("some-count") {
    pass !ident("pass_{}", n) {
        # ...
    }
}

compile if ("flag1" and "flag2") or not "flag3" {
    @vertex = "tangent"
    in tangent: f3;
} else if "flag4" {
    @vertex = "bitangent"
    in bitangent: f3;
}

# defining a pass, with mode = "Base"
# this `mode` tag controls how the pass is used by the engine
@mode = "Base"
pass first {
    # vertex program
    vertex {
        # ...

        # the shader compiler automatically inferences the type of the return value
        # this value will be used as fragment program's input
        compile if ("flag1" and "flag2") or not "flag3" {
            return {
                tangent: # ...
            }
        } else if "flag4" {
            return {
                bitangent: # ...
            }
        } else {
            return {
                color: # ...
            };
        }
    }

    # fragment program
    fragment {
        # ...
        # here, can access the return value of the vertex program defined above
    }
}

# defining a pass, with mode = "Additive"
# this `mode` tag controls how the pass is used by the engine
@mode = "Additive"
pass second {
    vertex {
        # ...
    }

    fragment {
        # ...
    }
}

fn compute_something(param1: f, param2: m4, param3: t2) -> f4 {
    # ...
}

compile if "flag5" {
    @mode = "Shadow"
    pass third {
        # ...
    }
} else {
    fn test() { }
}
```
