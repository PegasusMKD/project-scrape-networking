## Reading in a map

So far in the development journey, we've managed to create a UDP server, connect it to Godot using our own library and implementations, 
with the help of ["gdext"](https://github.com/godot-rust/gdext), and have successfully managed to connect multiple clients together. 
Other than that, we've also developed a sample map to be used for development in Blender 3.6, which we also properly load into Godot.

https://github.com/PegasusMKD/scrape-gltf-loader/assets/44293462/851c7dbc-2352-4c58-9dc0-791f40351b67

Our next steps, in terms of game logic, is implementing collision with the map so that the player cannot go out of bounds, or for example clip through floors.
But, before we can implement collision detection, we need a **THING** to collide *with*, meaning, we also need to load in that map into the UDP Server.
Why we need to implement the collision in the UDP Server as well is because the server is the main authority for game state, meaning that any positional updates for example
need to be done server-side due to the architecture we've decided to use.

But, we also don't need *all* of the data which the blend files provide, like the material & colors for example of the mesh. All that we need, game logic wise,
is just the vertices and indices to be able to do all the required calculations in terms of collision.

### The beginning

Since this is our first game, we were at first confused as to which options are even available to us. We knew that we were able to load in a .blend file
and that it also provides the data which we would need for collision detection since that's how Godot does it. But, we thought that approach would be
overly complicated, so we first attempted to think through a simpler solution. One idea which came to mind was just "hard-coding" in "walls & floors"
(aka just simple 4 point planes).

But, after thinking through it and experimenting with the idea, we quickly figured out that that approach wouldn't be sufficient for what we wanted, both in terms of
development, and in terms of implementation. More specifically, we wanted to let changes done in Blender automatically be imported over as a simple "update" to the map,
rather than forcing a developer to change the server parameters as well. Then, we also noticed that it can be very error-prone, given that you could potentially 
define separate "starting points" meaning that the center point of the map in Blender, and all items from that center point (walls, floors, etc.),
might not get mapped over 1-to-1 in the game.

So, we scratched that approach and decided to look into existing ideas and implementations. First thing we looked at was the source code of Doom, after which we
quickly realized that we most likely can't use that, since Doom actually uses 2D physics (or from what we saw it looked like 2D physics),
while we'd like full 3D physics in our game.

Next up on the table was to look at the source code of Godot. We knew that it already has collision detection and map loading, so we should be able to extract and 
rewrite parts of the Godot source code which correlate to that particular aspect of the game engine. We did some attempts at extracting the specific methods, but 
we'd always run into the same 2 problems:

 - Godot is an entire game engine, and things are linked very tightly together
 - Godot is written in C++ and makes **heavy** use of polymorphism which Rust doesn't *directly* provide

So, effectively, we'd still be re-inventing the wheel with this approach, which we'd like to avoid.

After doing a bit more research, we ended up finding [Bevy](https://bevyengine.org/), which is a game engine written entirely in Rust.

### Bevy

Finding Bevy solved our second problem, meaning that at least we can use things through composition to "inherit" properties. But, the real saviour is Bevy's architecture.
More specifically, Bevy is a ***fully modular*** game engine, meaning, we can extract specific crates to use (for example 'bevy_math' and 'bevy_transform'). Not only that,
we can also look through any module by itself, rather than having the case of everything depending on everything else.

So, we looked through the Bevy source code and found that Bevy also support the GLTF 2.0 format which Blender files can export to.

Thus, we found our perfect "inspiration" candidate. If anything, we could at least use it as a guide for the development to come.

Ofcourse, we'd have to cut out parts of the Bevy implementation, parts which we don't need, but we should be able to parse out the data which we need from the GLTF file.

### General implementation process

A short explanation of the implementation process, as well as the data we need.

#### Implementation process

We do the following steps in more-or-less a loop:
 - Copy over part of the 'bevy_gltf' loader code which we need
 - Save the file and see which functions and structs are missing
 - Go through the Bevy source code to find the missing information
 - Copy it over into a Rust file
 - Check if any more source code needs to be added, if not, go back to the 'bevy_gltf'

#### Needed information

Blender (and gltf) files provide a lot of data, but aS previously mentioned we don't require all of it.
We need the following information:
 - Mesh attribute metadata (name & type of an attribute)
 - Mesh attribute data (the actual data related to the attributes)
 - Mesh primitives
 - Mesh indices

We can narrow the data down even more by specifying the attribute types which we need:
 - Normals
 - Tangents
 - Positions

So, having all of that in mind, we can now create a project.


### The implementation

First, we need to create a simple project in which we can experiment so as to not "taint" the server with our experimentation.

```bash
cargo new scrape-gltf-loader
```

This should create a binary crate in which we can experiment. Why we start with a binary crate is simply because it'll be a lot easier
to change code and instantly test it (without the need to write tests).

Then, change into the project and open up the `main.rs` file.

After that, you can start inspecting the `loader.rs` file to maybe remove parts of the code which aren't needed, just to make all of the steps easier.
In this documentation, we'll only add code which is relevant to us and explain it, as well as it's related functions.

That should be all of the setup, so, lets start coding!


#### Babies first steps

Before we start copying over code, we need to look at which crate is being used to process the GLTF file. Bevy uses the '[gltf](https://docs.rs/gltf/latest/gltf/)' 
crate, which we can also use. To add the 'gltf' crate, all we need to do is run the following command:

```bash
cargo add gltf
```

The crate provides multiple approaches for loading in a GLTF file. The easiest/simplest one is with the following line:

```rust
let (document, buffers, _images) = gltf::import(file_path).expect("Couldn't read provided file_path");
```

Here, we only need 2 parameters:
 - document - contains all of the metadata of the file
 - buffers - the actual data for the attributes

And since we won't be rendering the map, we don't need any of the images.

Finally, we can start copying over some code!

The first segment which we'll copy over is

```rust
let mut meshes = vec![];
for gltf_mesh in gltf.meshes() {
    let mut primitives = vec![];
    for primitive in gltf_mesh.primitives() {
        let primitive_label = primitive_label(&gltf_mesh, &primitive);
        let primitive_topology = get_primitive_topology(primitive.mode())?;
    }
}
```

After we copy over this code, we need to make multiple changes. First change is to swap over the the "gltf" variable being used
to the "document" variable from our previous load since document is the one which provides that same data.
Then, we also change over the `vec![]` to `Vec::new()` just to be consistent with all of our coding practices.
We also do some small renamings to the variables to have the naming be clearer.
After that, when trying to fetch the label and topology of the primitives, we'll get an error stating that those functions aren't available.

We then look through the loader code to find those functions, and we do indeed find and copy them over.

```rust
/// Returns the label for the `mesh` and `primitive`.
fn primitive_label(mesh: &gltf::Mesh, primitive: &Primitive) -> String {
    format!("Mesh{}/Primitive{}", mesh.index(), primitive.index())
}

/// Maps the `primitive_topology` form glTF to `wgpu`.
fn get_primitive_topology(mode: Mode) -> Result<PrimitiveTopology, GltfError> {
    match mode {
        Mode::Points => Ok(PrimitiveTopology::PointList),
        Mode::Lines => Ok(PrimitiveTopology::LineList),
        Mode::LineStrip => Ok(PrimitiveTopology::LineStrip),
        Mode::Triangles => Ok(PrimitiveTopology::TriangleList),
        Mode::TriangleStrip => Ok(PrimitiveTopology::TriangleStrip),
        mode => Err(GltfError::UnsupportedPrimitive { mode }),
    }
}
```

Then we also copy over the GltfError and PrimitiveTopology from Bevy as well.
We then also update the usage of `get_primitive_topolgy` to instead just `unwrap()` the error since we know that this
will be used mainly for TriangleList, so it should just error out at that point in case it doesn't properly process a topology.

Next up, in the inner-most loop, we add this single line.

```rust
let mut mesh = Mesh::new(primitive_topology);
```

Why it's only a single line is that we also need to find the `Mesh` struct and copy over some functions which we need.
We can find the Mesh in the 'bevy_render' crate.

```rust
#[derive(Asset, Debug, Clone, Reflect)]
pub struct Mesh {
    #[reflect(ignore)]
    primitive_topology: PrimitiveTopology,
    /// `std::collections::BTreeMap` with all defined vertex attributes (Positions, Normals, ...)
    /// for this mesh. Attribute ids to attribute values.
    /// Uses a BTreeMap because, unlike HashMap, it has a defined iteration order,
    /// which allows easy stable VertexBuffers (i.e. same buffer order)
    #[reflect(ignore)]
    attributes: BTreeMap<MeshVertexAttributeId, MeshAttributeData>,
    indices: Option<Indices>,
    morph_targets: Option<Handle<Image>>,
    morph_target_names: Option<Vec<String>>,
}
```

From here, again, we don't need all of these properties. We can also swap out (or rename) some of the properties and structs as well, so we end up with a struct
which looks like this.

```rust
pub struct MeshPrimitive {
    pub topology: PrimitiveTopology,
    pub attributes: BTreeMap<u64, Attribute>,
    pub indices: Option<IndexVec>
}
```

We don't need the `morph_targets` (and related properties) since those are related to rendering and animation. We also remove the `MeshVertexAttributeId` since we don't 
particularly need a specific type for the attribute IDs.

We also rename the `Mesh` struct to `MeshPrimitive` since in the Bevy source code it's also mentioned that a `Mesh` in their code is actually just a primitive, while
for our usage, it would make more sense to have the entire map as a `Mesh` and have it be composed of multiple `MeshPrimitive`s.


Then, again, we also need the `IndexVec` and `Attribute` structs (which in Bevy are called `MeshAttributeData` and `Indices`).

```rust
/// An array of indices into the [`VertexAttributeValues`] for a mesh.
///
/// It describes the order in which the vertex attributes should be joined into faces.
#[derive(Debug, Clone, Reflect)]
pub enum Indices {
    U16(Vec<u16>),
    U32(Vec<u32>),
}

impl Indices {
    /// Returns an iterator over the indices.
    pub fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        match self {
            Indices::U16(vec) => IndicesIter::U16(vec.iter()),
            Indices::U32(vec) => IndicesIter::U32(vec.iter()),
        }
    }

    /// Returns the number of indices.
    pub fn len(&self) -> usize {
        match self {
            Indices::U16(vec) => vec.len(),
            Indices::U32(vec) => vec.len(),
        }
    }
}

#[derive(Debug, Clone)]
struct MeshAttributeData {
    attribute: MeshVertexAttribute,
    values: VertexAttributeValues,
}
```

Again, rename the appropriate structs and properties to something which makes sense to your usage, code practices and team.
So, we now end up with the following

```rust
#[derive(Debug, Clone)]
pub enum IndexVec {
    U16(Vec<u16>),
    U32(Vec<u32>)
}

impl IndexVec {
    /// Returns an iterator over the indices.
    pub fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        match self {
            IndexVec::U16(vec) => IndexVecIter::U16(vec.iter()),
            IndexVec::U32(vec) => IndexVecIter::U32(vec.iter()),
        }
    }

    /// Returns the number of indices.
    pub fn len(&self) -> usize {
        match self {
            IndexVec::U16(vec) => vec.len(),
            IndexVec::U32(vec) => vec.len(),
        }
    }
    
}

// MeshVertexData
#[derive(Debug, Clone)]
pub struct Attribute {
    pub metadata: AttributeMetadata,
    pub data: AttributeData,
}

```

Yet again, same process, compiler complains about `AttributeMetadata` and `AttributeData`, so we try to find them in the Bevy source code, rename appropriately,
add any other structs, enums or implementations we need, etc.

From here on out, we'll just copy over loader code, and we'll leave all of the renaming and digging through the Bevy source code to the reader. We'll only touch
upon the copied source code if there's some bigger difference in the implementations.

Next segment would be the following, which goes after the last line we added

```rust
for (semantic, accessor) in primitive.attributes() {
    if [Semantic::Joints(0), Semantic::Weights(0)].contains(&semantic) {
        if !meshes_on_skinned_nodes.contains(&gltf_mesh.index()) {
            warn!(
            "Ignoring attribute {:?} for skinned mesh {:?} used on non skinned nodes (NODE_SKINNED_MESH_WITHOUT_SKIN)",
            semantic,
            primitive_label
        );
            continue;
        } else if meshes_on_non_skinned_nodes.contains(&gltf_mesh.index()) {
            error!("Skinned mesh {:?} used on both skinned and non skin nodes, this is likely to cause an error (NODE_SKINNED_MESH_WITHOUT_SKIN)", primitive_label);
        }
    }
    match convert_attribute(
        semantic,
        accessor,
        &buffer_data,
        &loader.custom_vertex_attributes,
    ) {
        Ok((attribute, values)) => mesh.insert_attribute(attribute, values),
        Err(err) => warn!("{}", err),
    }
} 
```

This simply goes through the GLTF Mesh data and processes all of the attributes as well as their data, and adds them to the primitive.

Here, we don't need `Joints`, `Weights` or `Skinned Nodes` so we can just remove all of the code related to the `skinned nodes`, and skip any attributes which are
related to `Joints` and `Weights` since this is a map. We also change up the names of the variables, and the formatting, so this is the final segment which we end up with

```rust
for (semantic, accessor) in primitive.attributes() {
   if [Semantic::Joints(0), Semantic::Weights(0)].contains(&semantic) {
       continue; 
    }

    match conversion::convert_attribute(semantic, accessor, &buffers, None) {
        Ok((attribute, values)) => mesh_primitive.insert_attribute(attribute, values),
        Err(_err) => eprintln!("Something went wrong with adding the attribute..."),
    }
    
}
```

Another pretty important mention here is that we need to parse the attribute data, so for that we needed to copy-over the vertex iterators.
For that we need to copy over each possible combination/represantation of vertices, which we already have in the Bevy source code.

Next up we need to copy over the indices, which is this segment of code 

```rust
// Read vertex indices
let reader = primitive.reader(|buffer| Some(buffer_data[buffer.index()].as_slice()));
if let Some(indices) = reader.read_indices() {
    mesh.set_indices(Some(match indices {
        ReadIndices::U8(is) => Indices::U16(is.map(|x| x as u16).collect()),
        ReadIndices::U16(is) => Indices::U16(is.collect()),
        ReadIndices::U32(is) => Indices::U32(is.collect()),
    }));
};
```

Here, we just rename the `Indices` to `IndexVec`, nothing special.

After that ,we need to try to normalize the data

```rust
if mesh.attribute(Mesh::ATTRIBUTE_NORMAL).is_none()
    && matches!(mesh.primitive_topology(), PrimitiveTopology::TriangleList)
{
    let vertex_count_before = mesh.count_vertices();
    mesh.duplicate_vertices();
    mesh.compute_flat_normals();
    let vertex_count_after = mesh.count_vertices();

    if vertex_count_before != vertex_count_after {
        bevy_log::debug!("Missing vertex normals in indexed geometry, computing them as flat. Vertex count increased from {} to {}", vertex_count_before, vertex_count_after);
    } else {
        bevy_log::debug!(
            "Missing vertex normals in indexed geometry, computing them as flat."
        );
    }
}
```

We do multiple things here, we first swap out the bevy related logging since we plan to run this in a UDP Server, rather than using the Bevy game engine.
We should've mentioned this earlier, since we do do it earlier on, but since here we have a direct import of the 'bevy_log', we'll mention it here.
Then, another important part here is that at this point we need/would like to use `Vec3` from 'bevy_math', so, we add in that crate.

```bash
cargo add bevy_math
```

We need `Vec3` since for the normalization we do some vector calculations and the crate already provides all of those to us.

This is what the final copied segment looks like

```rust
if mesh_primitive.attribute(attribute_metadata::AttributeMetadata::ATTRIBUTE_NORMAL).is_none()
&& matches!(mesh_primitive.topology.clone(), enums::PrimitiveTopology::TriangleList)
{
    let vertex_count_before = mesh_primitive.count_vertices();
    mesh_primitive.duplicate_vertices();
    mesh_primitive.compute_flat_normals();
    let vertex_count_after = mesh_primitive.count_vertices();

    if vertex_count_before != vertex_count_after {
        println!("Missing vertex normals in indexed geometry, computing them as flat. Vertex count increased from {} to {}", vertex_count_before, vertex_count_after);
    } else {
        println!("Missing vertex normals in indexed geometry, computing them as flat.");
    }
}
```

Finally, we add the tangent data, as well as the mesh primitives to a list.

```rust
if let Some(vertex_attribute) = reader
    .read_tangents()
    .map(|v| VertexAttributeValues::Float32x4(v.collect()))
{
    mesh.insert_attribute(Mesh::ATTRIBUTE_TANGENT, vertex_attribute);
} else if mesh.attribute(Mesh::ATTRIBUTE_NORMAL).is_some()
    && primitive.material().normal_texture().is_some()
{
    bevy_log::debug!(
        "Missing vertex tangents, computing them using the mikktspace algorithm"
    );
    if let Err(err) = mesh.generate_tangents() {
        bevy_log::warn!(
            "Failed to generate vertex tangents using the mikktspace algorithm: {:?}",
            err
        );
    }
}
meshes.push(handle);
```

Again, we can remove a lot of this code since we don't expect many of these errors. This is the code we end up with

```rust
if let Some(vertex_attribute) = reader.read_tangents().map(|v| attribute_data::AttributeData::Float32x4(v.collect())) {
    mesh_primitive.insert_attribute(attribute_metadata::AttributeMetadata::ATTRIBUTE_TANGENT, vertex_attribute);
}


mesh_primitives.push(mesh_primitive);
```

Finally, collect all of the primitives into a struct and then return the meshes.

```rust
meshes.push(Mesh {
    mesh_primitives
});
```

### Clean-up

For the clean-up, we'd like to do the following things:
 - Transfer over the binary crate to a library crate so that we can use it for the collision or directly in the UDP Server
 - Divide up the copied code into a more 'appropriate' folder structure
 - Add some tests

#### Binary to Library crate

To transfer the binary crate to a library crate, all we need to do is create a `lib.rs` file and transfer all of the `main.rs` code there (except the `fn main() {...}`).

#### Folder structure

Then, we should divide up the code, and we decided on this file structure:

<pre>
    ├───core                # Enums and utilities
    ├───indices             # All IndexVec related structs and implementations
    ├───mesh                # Mesh related data
    │   └───attributes      # Attribute related structs and implementations
    └───vertex_iterator     # Vertex Iterator related structs and implementations
</pre>

And after that, we of course need to update all of the imports to reflect the new structure.

#### Tests

And finally, we'll add a simple test using the sample map which we have. The test looks like this:
```rust

#[cfg(test)]
mod tests {
    use crate::loader::load_gltf_file;

    #[test]
    fn test_load() {
        let meshes = load_gltf_file("./blend-files/Envoirment.gltf".to_string());
        assert_eq!(meshes.len(), 1);

        let mut vertices = 0;
        let mut indice_count = 0;
        for mesh in meshes.iter() {
            for item in &mesh.mesh_primitives {
                vertices += item.count_vertices();
                if let Some(index) = &item.indices {
                    indice_count += index.len();
                }
            }
        }

        assert_eq!(meshes.len(), 1);

        assert_eq!(vertices, 1992);
        println!("Total vertex count: {}", vertices);

        assert_eq!(indice_count/3, 1256);
        println!("Total triangles (indices/3): {}", indice_count/3);
    }
}
```

We can check that those are the numbers by using 'Microsoft Paint 3D' to open the GLTF file.

<TODO: Add image of Microsoft Paint here>


### Next steps

Now that we have the correct Mesh data, we can go on to implement Colliders into the game. 
We'll be using the `rapier` crate which has very handy functions for collision detection.
