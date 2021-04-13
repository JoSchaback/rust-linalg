# Welcome to the home of `js_linalg` 
`js_linalg` is probably the smallest (yet sufficiently complete), fast linear algebra library written in Rust on this planet :earth_africa:
for use with WebGL/OpenGL. This repo is the home of the `js_linalg` crate :package:. It is the nihilist approach to linear algebra for 3D graphics. 

Here is an example of how to use it. This should get the read a feeling.
```Rust
use js_linalg::{Vector3, Matrix3, Matrix4};

// Create a 3-dimensional f32 vector with all components set to 0.
let a = Vector3::new();

// Create a 3-dimensional f32 vector with the x, y and z components set to 12, -3 and 23 respectively.
let mut b = Vector3::from_i32(12, -3, 23);

// Create a 4x4 matrix, initialized as identity matrix.
let mut c = Matrix4::new();

// Set matrix c to a rotation matrix 0.1 radians around the z-axis.
c.rotate_mut(0.1, &js_linalg::z_up());

// Multiply vector b onto matrix c and write it back to b. This is an "in-place" operation.
b *= c;

```
In the real world, computer graphics programs usually concise of computing a a set of matrices, uploading them to OpenGL/WebGL/Vulkan regularly and let them do their magic. 
Common matrix calculations entail projections matrices, model-view matrices, normal matrices and model-view-projection matrices (MVP matrices). 
These matrices are then also used for ray picking to determine where the closed clicked in the scene.
```Rust
// On itialization or resize of the view frame, users often (re-) compute the projection matrix.
// Create a new projection matrix on the stack.
let p = self.projection_matrix.projection(
    45.0,           // field of view
    width as f32,   // viewport width
    height as f32,  // viewport height
    0.1,            // close clipping plane 
    100.0);         // far clipping plane
// Upload new projection matrix to shader program
shader_program.uniform_matrix4f(&uniform_location, &p);

```
When drawing frames, view- and model-view matrices usually need to be recomputed oftern.
```Rust
// Set up a rotation matrix on the stack for a model
let mut rot = Matrix4::new();
rot.set_rotation(rotation_rad, &Vector3::new_z_up());

// existing &mut view matrix is set to become a mapping form world space to camera space.
view_matrix.look_at_mut(
    &Vector3 {x:1.5, y:1.5, z:2.0}, // position of camera
    &Vector3::from_i32(0, 0, 0),    // position of center (where the camera looks at)
    &Vector3::new_z_up()            // up direction of camera
);

// compute the actual model-view matrix out of the rotation matrix (model) and camera (view) matrix
let model_view = &self.view_matrix * &rot;
// Upload model-view matrix
shader_program.uniform_matrix4f(&location, &model_view);

```

## Design Principles
The mere and humble purpose of `js_linalg` is to make the life of a computer graphics programmer easier.
But not necessarily the life of a mathematician. Here are some principles that guide the development of `js_linalg`:
- `js_linalg` comes with zero dependencies.
- `js_linalg` contains only functionality that is necessary for building fast OpenGL/WebGL applications. This means in particular low-dimensionality linear algebra (no arbitrary YxZ matrices). For example, if you need to compute eigenvalues or find the inverse of large, non-quadratic matrices, `js_linalg` is unfortunately not for you, but there are many, many other crates out there that offer full linear algebra functionalities.
- `js_linalg` provides "in-place" functions (mult, sub, add, etc) to reduce allocation costs further when desired alongside normal "per-copy" operations where a new result struct is created within each operation.
- `js_linalg` allows for "chaining", meaning that function calls can be chained together (`m.mul_mut(&vector).sub_mut(1., 2., -0.3).normalize_mut()`) as most functions return an result struct on which further operations are possible.
- `js_linalg` aims for fast uploading to the GPU. Updating matrices (e.g. shader uniforms) needs to be fast as it usually happens several times per frame. That is why the matrix component 
order is aligned with OpenGL for simple as fast uploading. A (unsafe) pointer to the first component is enough for uploading to OpenGL. For WASM/WebGL applications, you need to fill
an (usually stack allocated) array first unfortunately on that you create a JavaScript "view" to read-out the WASM memory directly in JavaScript (memcopy to the GPU). This is still, however a very fast operation. See crate `js_webgl` for more examples.
- `js_linalg` uses only `f32` fields, no abstractions, no `num-trait` crate (although it is great), because `f32` provide a sufficient precision for 99% of all computer graphics projects while not wasting bandwidth when uploading.
- `js_linalg` uses explicit fields (no arrays) which makes it quite simple for look-ahead optimizations and vectorization to take place as the calculation code is pretty linear. Our benchmarks have shown slight improvement over array-based implementations on certain hardware. The effect is negletable on desktop CPUs however.
- `js_linalg` is pragmatic for use with 3D graphics and is willing to compromise over "mathematical cohersion", that means for example it allows multiplication of 4x4 matrics with a 3D vector where the missing fourth dimension is always 1 as this is a common operation. Also, `js_linalg` has no concept of transpositions such that the result of matrix-vector multiplications is again a vector, not a point or transpositioned vector.

## Even nihilists may need more
It may be that there is something missing. Please feel free to reach out to me or file a ticket if there is something missing. Pull requests are appreciated as well.

## Main structs
`js_linalg` consists of the following central structs that are instrumental.
- `Vector3`: A three dimensional vector. Bascially stuff that you remember from school such as scaling, length, normalization etc.
- `Vector4`: A four dimensional vector, usually used rarley except for exotic multiplications with 4x4 matrices where the fourth dimension plays a role.
- `Matrix3`: A 3x3 matrix with all you need for a happy and fulfilled life.
- `Matrix4`: A 4x4 matrix with all you need for a happy and fulfilled life.

## How to build
The project the standard project setup for Rust. It requires a recent Rust installation. A simple
```
>cargo build
```
is enough to build the project. With `cargo test` you may run an array of tests included in `js_linalg`.
